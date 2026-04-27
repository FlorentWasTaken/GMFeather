use feather_core::use_cases;

#[tauri::command]
pub fn greet(name: &str) -> String {
    use_cases::greet_user(name)
}
