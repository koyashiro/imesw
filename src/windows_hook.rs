use std::sync::atomic::{AtomicBool, Ordering};

use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_LMENU, VK_RMENU},
    UI::WindowsAndMessaging::{CallNextHookEx, HC_ACTION, HHOOK, WM_KEYUP, WM_SYSKEYDOWN},
};

use crate::{ime, keyboard};

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
