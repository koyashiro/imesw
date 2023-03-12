use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA, UnhookWindowsHookEx,
        HC_ACTION, HHOOK, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
    },
};

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
            WM_KEYDOWN => {
                let key_code = unsafe { *(lparam.0 as *const u16) };
                println!("KEYDOWN: {key_code:#02x}");
            }
            WM_KEYUP => {
                let key_code = unsafe { *(lparam.0 as *const u16) };
                println!("KEYUP: {key_code:#02x}");
            }
            WM_SYSKEYDOWN => {
                let key_code = unsafe { *(lparam.0 as *const u16) };
                println!("SYSKEYDOWN: {key_code:#02x}");
            }
            WM_SYSKEYUP => {
                let key_code = unsafe { *(lparam.0 as *const u16) };
                println!("SYSKEYUP: {key_code:#02x}");
            }
            _ => (),
        }
    }
    unsafe { CallNextHookEx(HHOOK::default(), ncode, wparam, lparam) }
}
