use tauri::{AppHandle, command, Runtime, State, Window};

use crate::{config::InnerConfig, MyState, Result};

#[command]
pub(crate) async fn get_config<R: Runtime>(
  _app: AppHandle<R>,
  _window: Window<R>,
  state: State<'_, MyState>,
) -> Result<InnerConfig> {
  Ok(state.0.get_config())
}
