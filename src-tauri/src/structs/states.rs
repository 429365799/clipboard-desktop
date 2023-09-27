use std::{ops::Deref, sync::Mutex};

use super::clipboard::MyClipboardData;

#[derive(Debug)]
pub(crate) struct AppState {
    pub main_window_visibility: Mutex<bool>,
    pub clipboard_list: Mutex<Vec<MyClipboardData>>,
    pub commiting_select: Mutex<bool>,
}

impl AppState {
    pub fn default() -> AppState {
        AppState {
            main_window_visibility: Default::default(),
            clipboard_list: Mutex::new(vec![]),
            commiting_select: Mutex::new(false),
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

    pub fn is_commiting_select(&self) -> Result<bool, ()> {
        if let Ok(commiting_select) = self.commiting_select.lock() {
            return Ok(*commiting_select);
        }
        return Err(());
    }

    pub fn set_commiting_select(&self, value: bool) {
        if let Ok(mut commiting_select) = self.commiting_select.lock() {
            *commiting_select = value;
        }
    }
}
