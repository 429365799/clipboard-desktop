// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, Manager, Pixel, Position, Size, SystemTray,
    SystemTrayMenu
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod commands {
    pub(crate) mod get_clipboard_list;
    pub(crate) mod show_in_folder;
}

mod structs {
    pub(crate) mod clipboard;
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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::get_clipboard_list::get_clipboard_list,
            crate::commands::show_in_folder::show_in_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
