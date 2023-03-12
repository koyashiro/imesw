use std::mem;

use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::{
        Input::Ime::{ImmGetDefaultIMEWnd, IMC_SETOPENSTATUS},
        WindowsAndMessaging::{GetGUIThreadInfo, SendMessageA, GUITHREADINFO, WM_IME_CONTROL},
    },
};

pub fn enable() -> windows::core::Result<()> {
    set_ime(true)
}

pub fn disable() -> windows::core::Result<()> {
    set_ime(false)
}

fn set_ime(status: bool) -> windows::core::Result<()> {
    let mut gti = GUITHREADINFO {
        cbSize: mem::size_of::<GUITHREADINFO>() as _,
        ..Default::default()
    };
    unsafe { GetGUIThreadInfo(0, &mut gti) }.ok()?;
    let hwnd = unsafe { ImmGetDefaultIMEWnd(gti.hwndFocus) };
    unsafe {
        SendMessageA(
            hwnd,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS as _),
            LPARAM(status as _),
        );
    }

    Ok(())
}
