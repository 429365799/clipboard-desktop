use tauri::State;

use crate::structs::{clipboard::MyClipboardData, states::AppState};

#[tauri::command]
pub(crate) fn get_clipboard_list(app_state: State<'_, AppState>,) -> Vec<MyClipboardData> {
    if let Ok(data) = app_state.clipboard_list.lock() {
        return (*data).to_vec();
    }
    vec![]
}
