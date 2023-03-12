mod ime;
mod keyboard;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA, UnhookWindowsHookEx,
        HC_ACTION, HHOOK, MSG, WH_KEYBOARD_LL, WM_KEYUP, WM_SYSKEYDOWN,
    },
};

struct AltKeyPushing {
    left: bool,
    right: bool,
}

static ALT_KEY_PUSHING: Mutex<AltKeyPushing> = Mutex::new(AltKeyPushing {
    left: false,
    right: false,
});
static SHORTCUT_KEY_PUSHED: AtomicBool = AtomicBool::new(false);

fn main() -> windows::core::Result<()> {
    let khk =
        unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), HINSTANCE::default(), 0)? };

    let mut msg = MSG::default();
    while unsafe { GetMessageA(&mut msg, HWND::default(), 0, 0) }.into() {
        unsafe { DispatchMessageA(&msg) };
    }

    unsafe { UnhookWindowsHookEx(khk) };

    Ok(())
}

extern "system" fn callback(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if ncode as u32 == HC_ACTION {
        match wparam.0 as _ {
            WM_KEYUP => {
                let key_code = unsafe { *(lparam.0 as *const u16) };
                match key_code {
                    0xa4 => {
                        ALT_KEY_PUSHING.lock().unwrap().left = false;
                        if !ALT_KEY_PUSHING.lock().unwrap().right {
                            if !SHORTCUT_KEY_PUSHED.load(Ordering::Relaxed) {
                                match ime::disable() {
                                    Ok(_) => keyboard::send_vk_ff(),
                                    Err(e) => eprintln!("ime disable failed: {e}"),
                                }
                            }
                            SHORTCUT_KEY_PUSHED.store(false, Ordering::Relaxed);
                        }
                    }
                    0xa5 => {
                        ALT_KEY_PUSHING.lock().unwrap().right = false;
                        if !ALT_KEY_PUSHING.lock().unwrap().left {
                            if !SHORTCUT_KEY_PUSHED.load(Ordering::Relaxed) {
                                match ime::enable() {
                                    Ok(_) => keyboard::send_vk_ff(),
                                    Err(e) => eprintln!("ime enable failed: {e}"),
                                }
                            }
                            SHORTCUT_KEY_PUSHED.store(false, Ordering::Relaxed);
                        }
                    }
                    _ => (),
                }
            }
            WM_SYSKEYDOWN => {
                let key_code = unsafe { *(lparam.0 as *const u16) };
                match key_code {
                    0xa4 => {
                        ALT_KEY_PUSHING.lock().unwrap().left = true;
                    }
                    0xa5 => {
                        ALT_KEY_PUSHING.lock().unwrap().right = true;
                    }
                    _ => {
                        SHORTCUT_KEY_PUSHED.store(true, Ordering::Relaxed);
                    }
                }
            }
            _ => (),
        }
    }
    unsafe { CallNextHookEx(HHOOK::default(), ncode, wparam, lparam) }
}
