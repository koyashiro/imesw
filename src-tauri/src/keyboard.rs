use std::{fmt::Debug, mem};

use serde::{Deserialize, Serialize};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, VK__none_, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY,
    KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_CONTROL, VK_LMENU, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_SHIFT,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Key {
    #[serde(rename = "left_alt")]
    LeftAlt,

    #[serde(rename = "right_alt")]
    RightAlt,

    #[serde(rename = "left_shift")]
    LeftShift,

    #[serde(rename = "right_shift")]
    RightShift,

    #[serde(rename = "left_ctrl")]
    LeftCtrl,

    #[serde(rename = "right_ctrl")]
    RightCtrl,
}

impl Key {
    pub fn as_virtual_key(&self) -> VIRTUAL_KEY {
        match self {
            Key::LeftAlt => VK_LMENU,
            Key::RightAlt => VK_RMENU,
            Key::LeftShift => VK_SHIFT,
            Key::RightShift => VK_RSHIFT,
            Key::LeftCtrl => VK_CONTROL,
            Key::RightCtrl => VK_RCONTROL,
        }
    }
}

pub trait Keyboard: Debug + Send + Sync + 'static {
    fn send_vk_none(&self) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct KeyboardImpl;

impl Keyboard for KeyboardImpl {
    fn send_vk_none(&self) -> anyhow::Result<()> {
        unsafe {
            SendInput(
                &[
                    INPUT {
                        r#type: INPUT_KEYBOARD,
                        Anonymous: INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: VK__none_,
                                wScan: 0,
                                dwFlags: KEYEVENTF_EXTENDEDKEY,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    },
                    INPUT {
                        r#type: INPUT_KEYBOARD,
                        Anonymous: INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: VK__none_,
                                wScan: 0,
                                dwFlags: KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    },
                ],
                mem::size_of::<INPUT>() as _,
            );
        }

        Ok(())
    }
}
