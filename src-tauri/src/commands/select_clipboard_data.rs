use clipboardrs::api::write_clipboard_data;
use tauri::{AppHandle, Manager, State};

use crate::structs::states::AppState;

#[tauri::command]
pub(crate) fn select_clipboard_data(app_state: State<'_, AppState>, key: String, app: AppHandle) {
    if let Ok(data) = app_state.clipboard_list.lock() {
        (*data)
            .iter()
            .find(|item| *item.get_key() == key)
            .map(|item| {
                app_state.set_commiting_select(true);
                write_clipboard_data(item.to_clipboard_data(), true).unwrap();

                app.get_window("main").unwrap().hide().unwrap();
                app_state.set_main_window_visibility(false);
            });
    }
}
