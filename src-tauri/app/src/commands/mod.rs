use feather_core::common::greet;

#[tauri::command]
pub fn greet(name: &str) -> String {
    greet::greet_user(name)
}
