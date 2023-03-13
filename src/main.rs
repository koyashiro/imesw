mod ime;
mod keyboard;
mod watcher;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::watcher::Watcher;

fn main() {
    let mut watcher = Watcher::new();
    watcher.start();

    let running = Arc::new(AtomicBool::new(true));
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            watcher.stop().unwrap();
            r.store(false, Ordering::SeqCst)
        })
        .expect("Error setting Ctrl-C handler");
    }

    while running.load(Ordering::SeqCst) {}
}
