use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

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

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::ShionHistory;
#[cfg(mobile)]
use mobile::ShionHistory;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

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
        .invoke_handler(tauri::generate_handler![commands::execute])
        .setup(|app, api| {
            #[cfg(mobile)]
            let shion_history = mobile::init(app, api)?;
            #[cfg(desktop)]
            let shion_history = desktop::init(app, api)?;
            app.manage(shion_history);

            // manage state so it is accessible by the commands
            app.manage(MyState::default());
            Ok(())
        })
        .build()
}
