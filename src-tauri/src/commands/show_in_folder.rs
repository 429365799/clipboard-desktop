use std::process::Command;

#[tauri::command]
pub(crate) fn show_in_folder(path: &str) {
    Command::new("explorer")
        .args(["/select,", &path]) // The comma after select is not a typo
        .spawn()
        .unwrap();
}