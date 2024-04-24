use tauri::{command, AppHandle, Runtime, State, Window};

use crate::{config::InnerConfig, database::Record, history, MyState, Result};

#[command]
pub(crate) async fn get_config<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, MyState>,
) -> Result<InnerConfig> {
    Ok(state.0.get_config())
}

#[command]
pub(crate) async fn read_history<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, MyState>,
    name: String,
    start: u64,
    end: u64,
) -> Result<Vec<Record>> {
    let map = history::get_database_map();
    let mut result = vec![];
    let database_path_list = map.get(&name.as_str()).unwrap_or(&vec![]).clone();
    for path in database_path_list {
        let mut list = state.0.read(&name, path, start, end)?;
        result.append(&mut list);
    }
    Ok(result)
}

#[command]
pub(crate) async fn set_config<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, MyState>,
    config: InnerConfig
) -> Result<()> {
    state.0.set_config(config);
    Ok(())
}