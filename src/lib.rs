use std::sync::Mutex;

use history::History;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod config;
mod error;
mod history;
mod models;
mod shared;
mod database;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::ShionHistory;
#[cfg(mobile)]
use mobile::ShionHistory;

// #[derive(Default)]
struct MyState(History);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the shion-history APIs.
pub trait ShionHistoryExt<R: Runtime> {
    fn shion_history(&self) -> &ShionHistory<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ShionHistoryExt<R> for T {
    fn shion_history(&self) -> &ShionHistory<R> {
        self.state::<ShionHistory<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("shion-history")
        .invoke_handler(tauri::generate_handler![commands::get_config, commands::read_history])
        .setup(|app, api| {
            #[cfg(mobile)]
            let shion_history = mobile::init(app, api)?;
            #[cfg(desktop)]
            let shion_history = desktop::init(app, api)?;
            app.manage(shion_history);

            let base = app.app_handle().path().app_data_dir().unwrap();
            let history = History::new(base);
            app.manage(MyState(history));
            Ok(())
        })
        .build()
}
