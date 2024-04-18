const COMMANDS: &[&str] = &["get_config", "read_history", "set_config"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
