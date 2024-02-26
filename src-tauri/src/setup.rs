use std::sync::{Arc, RwLock};

use tauri::{App, Manager};

use crate::{
    hook,
    ime_manager::ImeManagerImpl,
    keyboard::KeyboardImpl,
    setting::{SettingReader, SettingReaderImpl, SettingWriter},
    tray::{SystemTrayUpdater, SystemTrayUpdaterImpl, IS_RUNNING_CUSTOM_MENU_ITEM_ID},
};

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let setting_reader = SettingReaderImpl;
    let setting = setting_reader.read_from_file().unwrap_or_default();

    let system_tray_updater: Arc<dyn SystemTrayUpdater> = Arc::new(SystemTrayUpdaterImpl::new({
        app.tray_handle().get_item(IS_RUNNING_CUSTOM_MENU_ITEM_ID)
    }));
    system_tray_updater.update(&setting)?;

    let setting = Arc::new(RwLock::new(setting));

    let setting_writer: Arc<dyn SettingWriter> = Arc::new(SettingReaderImpl);

    app.manage(setting.clone());
    app.manage(system_tray_updater);
    app.manage(setting_writer);

    hook::init(setting, Box::new(ImeManagerImpl), Box::new(KeyboardImpl))?;

    Ok(())
}
