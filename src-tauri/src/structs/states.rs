use std::sync::Mutex;

use super::clipboard::MyClipboardData;

#[derive(Debug)]
pub(crate) struct AppState {
    pub main_window_visibility: Mutex<bool>,
    pub clipboard_list: Mutex<Vec<MyClipboardData>>,
}

impl AppState {
    pub fn default() -> AppState {
        AppState {
            main_window_visibility: Default::default(),
            clipboard_list: Mutex::new(vec![]),
        }
    }
}
