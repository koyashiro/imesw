use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};

use once_cell::sync::Lazy;
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

use crate::{
    config::{ConfigManager, ConfigManagerImpl},
    ime::{ImeActivator, ImeActivatorImpl},
    keyboard::{Keyboard, KeyboardImpl},
};

static GLOBAL_HHOOK: RwLock<Option<HHOOK>> = RwLock::new(None);
static ACTIVATE_KEY_PUSHING: AtomicBool = AtomicBool::new(false);
static DEACTIVATE_KEY_PUSHING: AtomicBool = AtomicBool::new(false);
static OTHER_KEY_PUSHED: AtomicBool = AtomicBool::new(false);

static CONFIG_MANAGER: Lazy<RwLock<Arc<RwLock<dyn ConfigManager>>>> =
    Lazy::new(|| RwLock::new(Arc::new(RwLock::new(ConfigManagerImpl::default()))));
static IME_ACTIVATOR: Lazy<RwLock<Box<dyn ImeActivator>>> =
    Lazy::new(|| RwLock::new(Box::new(ImeActivatorImpl)));
static KEYBOARD: Lazy<RwLock<Box<dyn Keyboard>>> =
    Lazy::new(|| RwLock::new(Box::new(KeyboardImpl)));

pub extern "system" fn callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let config_manager = CONFIG_MANAGER.read().unwrap();
    let config_manager = config_manager.read().unwrap();
    let config = config_manager.get_config();

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
                DEACTIVATE_KEY_PUSHING.store(false, Ordering::Relaxed);

                if !ACTIVATE_KEY_PUSHING.load(Ordering::Relaxed) {
                    if !OTHER_KEY_PUSHED.load(Ordering::Relaxed) {
                        IME_ACTIVATOR.read().unwrap().activate().unwrap();
                    }

                    OTHER_KEY_PUSHED.store(false, Ordering::Relaxed);
                }

                KEYBOARD.read().unwrap().send_vk_none().unwrap();
            } else if vk == config.deactivate_key.as_virtual_key() {
                ACTIVATE_KEY_PUSHING.store(false, Ordering::Relaxed);

                if !DEACTIVATE_KEY_PUSHING.load(Ordering::Relaxed) {
                    if !OTHER_KEY_PUSHED.load(Ordering::Relaxed) {
                        IME_ACTIVATOR.read().unwrap().deactivate().unwrap();
                    }

                    OTHER_KEY_PUSHED.store(false, Ordering::Relaxed);
                }

                KEYBOARD.read().unwrap().send_vk_none().unwrap();
            }
        }
        WM_SYSKEYDOWN => {
            let vk = unsafe { *(lparam.0 as *const VIRTUAL_KEY) };

            if vk == config.activate_key.as_virtual_key() {
                DEACTIVATE_KEY_PUSHING.store(true, Ordering::Relaxed);
                if ACTIVATE_KEY_PUSHING.load(Ordering::Relaxed) {
                    OTHER_KEY_PUSHED.store(true, Ordering::Relaxed);
                }
            } else if vk == config.deactivate_key.as_virtual_key() {
                ACTIVATE_KEY_PUSHING.store(true, Ordering::Relaxed);
                if DEACTIVATE_KEY_PUSHING.load(Ordering::Relaxed) {
                    OTHER_KEY_PUSHED.store(true, Ordering::Relaxed);
                }
            } else {
                OTHER_KEY_PUSHED.store(true, Ordering::Relaxed);
            }
        }
        _ => (),
    }

    unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) }
}

pub fn init(
    config_manager: Arc<RwLock<dyn ConfigManager>>,
    ime_activator: Box<dyn ImeActivator>,
    keyboard: Box<dyn Keyboard>,
) -> anyhow::Result<()> {
    set_global_hook_if_needed()?;

    *CONFIG_MANAGER
        .write()
        .map_err(|e| anyhow::anyhow!("failed to write CONFIG_MANAGER: {e}"))? = config_manager;

    *IME_ACTIVATOR
        .write()
        .map_err(|e| anyhow::anyhow!("failed to write IME_ACTIVATOR: {e}"))? = ime_activator;

    *KEYBOARD
        .write()
        .map_err(|e| anyhow::anyhow!("failed to write KEYBOARD: {e}"))? = keyboard;

    Ok(())
}

fn set_global_hook_if_needed() -> anyhow::Result<()> {
    let mut global_hook = GLOBAL_HHOOK
        .write()
        .map_err(|e| anyhow::anyhow!("failed to write GLOBAL_HHOOK: {e}"))?;

    if global_hook.is_some() {
        return Ok(());
    }

    let hhook =
        unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), HINSTANCE::default(), 0) }?;

    *global_hook = Some(hhook);

    Ok(())
}
