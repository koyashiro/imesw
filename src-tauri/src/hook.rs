use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
};

use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::VIRTUAL_KEY,
        WindowsAndMessaging::{
            CallNextHookEx, SetWindowsHookExA, HC_ACTION, HHOOK, WH_KEYBOARD_LL, WM_KEYUP,
            WM_SYSKEYDOWN,
        },
    },
};

use crate::{config::ConfigManager, ime::ImeActivator, keyboard::Keyboard};

static GLOBAL_HHOOK: RwLock<Option<HHOOK>> = RwLock::new(None);
static GLOBAL_CALLBACK_STATE: RwLock<Option<CallbackState>> = RwLock::new(None);

#[derive(Debug)]
struct CallbackState {
    activate_key_pushing: AtomicBool,
    deactivate_key_pushing: AtomicBool,
    other_key_pushed: AtomicBool,
    config_manager: Arc<dyn ConfigManager>,
    ime_activator: Box<dyn ImeActivator>,
    keyboard: Box<dyn Keyboard>,
}

pub fn init(
    config_manager: Arc<dyn ConfigManager>,
    ime_activator: Box<dyn ImeActivator>,
    keyboard: Box<dyn Keyboard>,
) -> anyhow::Result<()> {
    set_global_hook_if_needed()?;

    let state = CallbackState {
        activate_key_pushing: AtomicBool::new(false),
        deactivate_key_pushing: AtomicBool::new(false),
        other_key_pushed: AtomicBool::new(false),
        config_manager,
        ime_activator,
        keyboard,
    };

    *GLOBAL_CALLBACK_STATE.write().map_err(into_anyhow_error)? = Some(state);

    Ok(())
}

fn set_global_hook_if_needed() -> anyhow::Result<()> {
    let mut global_hook = GLOBAL_HHOOK.write().map_err(into_anyhow_error)?;

    if global_hook.is_some() {
        return Ok(());
    }

    let hhook =
        unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), HINSTANCE::default(), 0) }?;

    *global_hook = Some(hhook);

    Ok(())
}

extern "system" fn callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let state = GLOBAL_CALLBACK_STATE.read().unwrap();
    let state = match state.as_ref() {
        Some(s) => s,
        None => {
            return unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) };
        }
    };

    let config = state.config_manager.config().unwrap();

    if !config.is_running {
        return unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) };
    }

    if code as u32 != HC_ACTION {
        return unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) };
    }

    match wparam.0 as _ {
        WM_KEYUP => {
            let vk = unsafe { *(lparam.0 as *const VIRTUAL_KEY) };

            if vk == config.activate_key.as_virtual_key() {
                state.deactivate_key_pushing.store(false, Ordering::SeqCst);

                if !state.activate_key_pushing.load(Ordering::SeqCst) {
                    if !state.other_key_pushed.load(Ordering::SeqCst) {
                        state.ime_activator.activate().unwrap();
                    }

                    state.other_key_pushed.store(false, Ordering::SeqCst);
                }

                state.keyboard.send_vk_none().unwrap();
            } else if vk == config.deactivate_key.as_virtual_key() {
                state.activate_key_pushing.store(false, Ordering::SeqCst);

                if !state.deactivate_key_pushing.load(Ordering::SeqCst) {
                    if !state.other_key_pushed.load(Ordering::SeqCst) {
                        state.ime_activator.deactivate().unwrap();
                    }

                    state.other_key_pushed.store(false, Ordering::SeqCst);
                }

                state.keyboard.send_vk_none().unwrap();
            }
        }
        WM_SYSKEYDOWN => {
            let vk = unsafe { *(lparam.0 as *const VIRTUAL_KEY) };

            if vk == config.activate_key.as_virtual_key() {
                state.deactivate_key_pushing.store(true, Ordering::SeqCst);
                if state.activate_key_pushing.load(Ordering::SeqCst) {
                    state.other_key_pushed.store(true, Ordering::SeqCst);
                }
            } else if vk == config.deactivate_key.as_virtual_key() {
                state.activate_key_pushing.store(true, Ordering::SeqCst);
                if state.deactivate_key_pushing.load(Ordering::SeqCst) {
                    state.other_key_pushed.store(true, Ordering::SeqCst);
                }
            } else {
                state.other_key_pushed.store(true, Ordering::SeqCst);
            }
        }
        _ => (),
    }

    unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) }
}

fn into_anyhow_error(error: impl Error) -> anyhow::Error {
    anyhow::anyhow!(error.to_string())
}
