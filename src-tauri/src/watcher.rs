use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    System::Threading::GetCurrentThreadId,
    UI::{
        Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_LMENU, VK_RMENU},
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageA, GetMessageA, PostThreadMessageA, SetWindowsHookExA,
            UnhookWindowsHookEx, HC_ACTION, HHOOK, MSG, WH_KEYBOARD_LL, WM_KEYUP, WM_SYSKEYDOWN,
        },
    },
};

use crate::{ime, keyboard};

pub const STOP_MSG: u32 = 0x8000;

#[derive(Debug, Default)]
pub struct Watcher {
    thread: Option<JoinHandle<()>>,
    windows_thread_id: Arc<AtomicU32>,
}

impl Watcher {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn start(&mut self) {
        println!("start");
        let windows_thread_id = self.windows_thread_id.clone();
        let t = thread::spawn(move || {
            windows_thread_id.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed);

            let hhk = unsafe {
                SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), HINSTANCE::default(), 0)
            }
            .unwrap();
            let mut msg = MSG::default();
            while unsafe { GetMessageA(&mut msg, HWND::default(), 0, 0) }.into() {
                if msg.message == STOP_MSG {
                    break;
                }
                unsafe { DispatchMessageA(&msg) };
            }

            unsafe { UnhookWindowsHookEx(hhk) }.ok().unwrap();
        });

        self.thread = Some(t);
    }

    pub fn stop(&mut self) -> thread::Result<()> {
        println!("stop");
        let windows_thread_id = self.windows_thread_id.load(Ordering::Relaxed);
        if windows_thread_id != 0 {
            unsafe { PostThreadMessageA(windows_thread_id, STOP_MSG, WPARAM(0), LPARAM(0)) };
            self.windows_thread_id.store(0, Ordering::Relaxed);
        }

        if let Some(thread) = self.thread.take() {
            thread.join()?;
        }
        Ok(())
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        println!("drop");
        self.stop().unwrap();
    }
}

static LEFT_ALT_KEY_PUSHING: AtomicBool = AtomicBool::new(false);
static RIGHT_ALT_KEY_PUSHING: AtomicBool = AtomicBool::new(false);
static SHORTCUT_KEY_PUSHED: AtomicBool = AtomicBool::new(false);

pub extern "system" fn callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code as u32 != HC_ACTION {
        return unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) };
    }

    match wparam.0 as _ {
        WM_KEYUP => {
            let vk = unsafe { *(lparam.0 as *const VIRTUAL_KEY) };

            match vk {
                VK_LMENU => {
                    LEFT_ALT_KEY_PUSHING.store(false, Ordering::Relaxed);

                    if !RIGHT_ALT_KEY_PUSHING.load(Ordering::Relaxed) {
                        if !SHORTCUT_KEY_PUSHED.load(Ordering::Relaxed) {
                            if let Err(e) = ime::disable() {
                                eprintln!("ime disable failed: {e}");
                            }
                        }

                        SHORTCUT_KEY_PUSHED.store(false, Ordering::Relaxed);
                    }

                    keyboard::send_vk_ff();
                }
                VK_RMENU => {
                    RIGHT_ALT_KEY_PUSHING.store(false, Ordering::Relaxed);

                    if !LEFT_ALT_KEY_PUSHING.load(Ordering::Relaxed) {
                        if !SHORTCUT_KEY_PUSHED.load(Ordering::Relaxed) {
                            if let Err(e) = ime::enable() {
                                eprintln!("ime enable failed: {e}");
                            }
                        }

                        SHORTCUT_KEY_PUSHED.store(false, Ordering::Relaxed);
                    }

                    keyboard::send_vk_ff();
                }
                _ => (),
            }
        }
        WM_SYSKEYDOWN => {
            let vk = unsafe { *(lparam.0 as *const VIRTUAL_KEY) };

            match vk {
                VK_LMENU => {
                    LEFT_ALT_KEY_PUSHING.store(true, Ordering::Relaxed);
                    if RIGHT_ALT_KEY_PUSHING.load(Ordering::Relaxed) {
                        SHORTCUT_KEY_PUSHED.store(true, Ordering::Relaxed);
                    }
                }
                VK_RMENU => {
                    RIGHT_ALT_KEY_PUSHING.store(true, Ordering::Relaxed);
                    if LEFT_ALT_KEY_PUSHING.load(Ordering::Relaxed) {
                        SHORTCUT_KEY_PUSHED.store(true, Ordering::Relaxed);
                    }
                }
                _ => {
                    SHORTCUT_KEY_PUSHED.store(true, Ordering::Relaxed);
                }
            }
        }
        _ => (),
    }

    unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) }
}
