use std::fmt::Debug;

use anyhow::Context as _;
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::{
        Input::Ime::{ImmGetDefaultIMEWnd, IMC_SETOPENSTATUS},
        WindowsAndMessaging::{GetGUIThreadInfo, SendMessageA, GUITHREADINFO, WM_IME_CONTROL},
    },
};

pub trait ImeActivator: Debug + Send + Sync + 'static {
    fn activate(&self) -> anyhow::Result<()>;
    fn deactivate(&self) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct ImeActivatorImpl;

impl ImeActivator for ImeActivatorImpl {
    fn activate(&self) -> anyhow::Result<()> {
        set_ime_status(true)
    }

    fn deactivate(&self) -> anyhow::Result<()> {
        set_ime_status(false)
    }
}

fn set_ime_status(status: bool) -> anyhow::Result<()> {
    let mut gui_thread_info = GUITHREADINFO {
        cbSize: std::mem::size_of::<GUITHREADINFO>() as _,
        ..Default::default()
    };

    unsafe { GetGUIThreadInfo(0, &mut gui_thread_info) }
        .context("failed to call GetGUIThreadInfo")?;

    let hwnd = unsafe { ImmGetDefaultIMEWnd(gui_thread_info.hwndFocus) };
    if hwnd.0 == 0 {
        return Err(anyhow::anyhow!("failed to call ImmGetDefaultIMEWnd"));
    }

    let lresult = unsafe {
        SendMessageA(
            hwnd,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS as _),
            LPARAM(status as _),
        )
    };
    if lresult.0 != 0 {
        return Err(anyhow::anyhow!("failed to call SendMessageA: {lresult:?}"));
    }

    Ok(())
}
