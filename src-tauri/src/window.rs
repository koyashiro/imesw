use tauri::{
    GlobalWindowEvent, PhysicalSize,
    WindowEvent::{CloseRequested, Resized},
};

pub fn window_event(e: GlobalWindowEvent) {
    let (e, w) = (e.event(), e.window());
    match e {
        CloseRequested { api, .. } => {
            w.hide().expect("Failed to hide the window");
            api.prevent_close();
        }
        Resized(PhysicalSize {
            width: 0,
            height: 0,
        }) => {
            w.hide().expect("Failed to hide the window");
        }
        _ => (),
    };
}
