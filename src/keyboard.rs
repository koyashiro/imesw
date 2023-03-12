use std::mem;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP,
    VIRTUAL_KEY,
};

pub const VK_FF: VIRTUAL_KEY = VIRTUAL_KEY(255u16);

pub fn send_vk_ff() {
    unsafe {
        SendInput(
            &[
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_FF,
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
                            wVk: VK_FF,
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
}
