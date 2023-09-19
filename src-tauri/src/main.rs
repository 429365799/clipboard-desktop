// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::structs::clipboard::MyClipboardData;
use crate::structs::states::AppState;
use tauri::{
    App, CustomMenuItem, Manager, Pixel, Position, Size, State, SystemTray, SystemTrayEvent,
    SystemTrayMenu, WindowEvent,
};

use clipboardrs::api::read_clipboard_data;
use clipboardrs::listener::ClipboardListen;

mod commands {
    pub(crate) mod get_clipboard_list;
    pub(crate) mod show_in_folder;
    pub(crate) mod window_visible;
}

mod structs {
    pub(crate) mod clipboard;
    pub(crate) mod states;
}

fn init_app(app: &App) {
    let app_handle = app.app_handle();
    ClipboardListen::run(move || {
        if let Ok(data) = read_clipboard_data() {
            let app_state = app_handle.state::<AppState>();
            app_state.put_clipboard_data(MyClipboardData::new(data));
            app_handle.emit_all("CLIPBOARD_UPDATE", ()).unwrap();
        }
    });
}

fn init_window_style(app: &App) {
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
    }

    main_window.set_skip_taskbar(true).unwrap();
    main_window.hide().unwrap();
}

fn init_window_event(app: &App) {
    let main_window = app.get_window("main").unwrap();

    main_window.on_window_event(|event: &WindowEvent| match event {
        WindowEvent::Focused(is_focus) => {
            if !is_focus {

                // event.window();
                // main_window.hide().unwrap();
            }
        }
        _ => (),
    });
}

fn init_tray_menus(app: &App) {
    let main_window = app.get_window("main").unwrap();

    SystemTray::new()
        .with_menu(
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("open", "打开"))
                .add_item(CustomMenuItem::new("settings", "设置"))
                .add_item(CustomMenuItem::new("quit", "退出")),
        )
        .on_event(move |event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let app_state: State<'_, AppState> = main_window.state();

                if let Ok(window_visible) = main_window.is_visible() {
                    if window_visible {
                        main_window.hide().unwrap();
                    } else {
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }

                    app_state.set_main_window_visibility(!window_visible);
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let app_state: State<'_, AppState> = main_window.state();

                match id.as_str() {
                    "open" => {
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        app_state.set_main_window_visibility(true);
                    }
                    "settings" => {}
                    "quit" => {}
                    _ => (),
                }
            }
            _ => (),
        })
        .build(app)
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            init_app(&app);
            init_window_style(&app);
            init_window_event(&app);
            init_tray_menus(&app);

            println!("Success setup application!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::get_clipboard_list::get_clipboard_list,
            crate::commands::show_in_folder::show_in_folder,
            crate::commands::window_visible::toggle_main_window,
            crate::commands::window_visible::show_main_window,
            crate::commands::window_visible::hide_main_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
