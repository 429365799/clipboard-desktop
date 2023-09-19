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

    pub fn set_main_window_visibility(&self, visible: bool) {
        if let Ok(mut visibility) = self.main_window_visibility.lock() {
            *visibility = visible;
        }
    }

    pub fn put_clipboard_data(&self, data: MyClipboardData) {
        if let Ok(mut clipboard_list) = self.clipboard_list.lock() {
            match (*clipboard_list).last() {
                Some(first) => {
                    if !first.eq(&data) {
                        (*clipboard_list).push(data);
                    }
                }
                None => (*clipboard_list).push(data),
            }
            (*clipboard_list).sort_by(|a, b| a.comp(b))
        }
    }
}
