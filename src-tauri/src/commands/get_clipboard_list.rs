
use clipboardrs::api::read_clipboard_data;

use crate::structs::clipboard::MyClipboardData;

#[tauri::command]
pub(crate) fn get_clipboard_list() -> Vec<MyClipboardData> {
    let mut list = vec![];
    if let Ok(data) = read_clipboard_data() {
        list.push(MyClipboardData::new(data));
    }

    list
}
