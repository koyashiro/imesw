mod ime;
mod keyboard;
mod windows_hook;

use windows::Win32::{
    Foundation::{HINSTANCE, HWND},
    UI::WindowsAndMessaging::{
        DispatchMessageA, GetMessageA, SetWindowsHookExA, UnhookWindowsHookEx, MSG, WH_KEYBOARD_LL,
    },
};

use windows_hook::callback;

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
