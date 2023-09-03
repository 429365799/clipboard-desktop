// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::process::Command;

use clipboardrs::api::{read_clipboard_data, write_clipboard_data, ClipboardData, ClipboardFile};
use serde::{ser::SerializeStruct, Serialize};
use tauri::{
    api::path::cache_dir, CustomMenuItem, Manager, Pixel, Position, Size, SystemTray,
    SystemTrayMenu
};
use uuid::Uuid;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

struct MyFile(ClipboardFile);

impl Serialize for MyFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("MyFile", 2)?;
        s.serialize_field("path", &self.0.path)?;
        s.serialize_field("size", &self.0.size)?;
        s.end()
    }
}

struct MyClipboardData {
    len: usize,
    text: Option<String>,
    html: Option<String>,
    image: Option<String>,
    files: Option<Vec<MyFile>>,
}

impl MyClipboardData {
    fn new(raw_data: ClipboardData) -> MyClipboardData {
        let mut len: usize = 0;
        let mut data = MyClipboardData {
            len: 0,
            text: None,
            html: None,
            image: None,
            files: None,
        };

        if let Some(val) = raw_data.text {
            len += 1;
            data.text = Some(val);
        }
        if let Some(val) = raw_data.html {
            len += 1;
            data.html = Some(val);
        }
        if let Some(val) = raw_data.image {
            if let Some(p) = cache_dir() {
                let parent_dir = p.join("clipboard-tauri");
                let path = parent_dir.join(format!("{}.png", Uuid::new_v4()));
                if let Ok(()) = fs::create_dir_all(parent_dir) {
                    if let Some(path) = path.to_str() {
                        if let Ok(()) = val.save(path) {
                            len += 1;
                            data.image = Some(path.to_string());
                        }
                    }
                }
            }
        }
        if let Some(val) = raw_data.files {
            let mut files = vec![];
            for f in val {
                files.push(MyFile(f));
            }
            data.files = Some(files);
        }

        data.len = len;

        data
    }
}

impl Serialize for MyClipboardData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("MyClipboardData", self.len)?;
        if let Some(text) = &self.text {
            s.serialize_field("text", text)?;
        }
        if let Some(html) = &self.html {
            s.serialize_field("html", html)?;
        }
        if let Some(img) = &self.image {
            s.serialize_field("image", img)?;
        }
        if let Some(files) = &self.files {
            s.serialize_field("files", files)?;
        }
        s.end()
    }
}

#[tauri::command]
fn get_clipboard_list() -> Vec<MyClipboardData> {
    let mut list = vec![];
    if let Ok(data) = read_clipboard_data() {
        list.push(MyClipboardData::new(data));
    }

    list
}

#[tauri::command]
fn show_in_folder(path: &str) {
    Command::new("explorer")
        .args(["/select,", &path]) // The comma after select is not a typo
        .spawn()
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            if let Ok(Some(monitor)) = main_window.current_monitor() {
                let size = monitor.size();

                main_window
                    .set_position(Position::Physical(tauri::PhysicalPosition {
                        x: 0,
                        y: size.height.cast::<i32>() - 300,
                    }))
                    .expect("Set window position failed!");
                main_window
                    .set_size(Size::Physical(tauri::PhysicalSize {
                        width: size.width,
                        height: 300,
                    }))
                    .expect("Set window size failed!");

                println!("Success setup application!");
            }

            main_window.set_skip_taskbar(true).unwrap();

            SystemTray::new()
                .with_menu(
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("quit", "Quit"))
                        .add_item(CustomMenuItem::new("open", "Open")),
                )
                .on_event(|event| {

                })
                .build(app)?;

            app.global_shortcut_manager().register("Ctrl+Shift+V", || {
                println!("！！！");
            }).expect("快捷键注册失败");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![show_in_folder, get_clipboard_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
