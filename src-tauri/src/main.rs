// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::structs::states::AppState;
use tauri::{
    CustomMenuItem, Manager, Pixel, Position, Size, SystemTray, SystemTrayEvent, SystemTrayMenu,
    WindowEvent,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod commands {
    pub(crate) mod get_clipboard_list;
    pub(crate) mod show_in_folder;
    pub(crate) mod window_visible;
}

mod structs {
    pub(crate) mod clipboard;
    pub(crate) mod states;
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
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
            }

            main_window.set_skip_taskbar(true).unwrap();

            // main_window.on_window_event(|event: &WindowEvent| match event {
            //     WindowEvent::Focused(is_focus) => {
            //         if !is_focus {

            //             // event.window();
            //             // main_window.hide().unwrap();
            //         }
            //     }
            //     _ => (),
            // });

            SystemTray::new()
                .with_menu(
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("quit", "Quit"))
                        .add_item(CustomMenuItem::new("open", "Open")),
                )
                .on_event(|event| match event {
                    SystemTrayEvent::LeftClick {
                        tray_id,
                        position,
                        size,
                        ..
                    } => {}
                    // SystemTrayEvent::MenuItemClick { tray_id, id, .. } => todo!(),
                    // SystemTrayEvent::RightClick {
                    //     tray_id,
                    //     position,
                    //     size,
                    //     ..
                    // } => todo!(),
                    // SystemTrayEvent::DoubleClick {
                    //     tray_id,
                    //     position,
                    //     size,
                    //     ..
                    // } => todo!(),
                    _ => todo!(),
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::get_clipboard_list::get_clipboard_list,
            crate::commands::show_in_folder::show_in_folder,
            crate::commands::window_visible::toggle_main_window,
            crate::commands::window_visible::show_main_window,
            crate::commands::window_visible::hide_main_window,
        ])
        //     main_window.hide().unwrap();
        //     println!("Success setup application!");
        //     Ok(())
        // })
        // // .on_window_event(|event: GlobalWindowEvent| match event.event() {
        // //     WindowEvent::Focused(is_focus) => {
        // //         let w = event.window();
        // //         println!("window focus {}", is_focus,);
        // //         if w.label() == "main" && !*is_focus {
        // //             w.hide().unwrap();
        // //             let app_state: State<'_, AppState> = w.state();
        // //             if let Ok(mut main_window_visibility) = app_state.main_window_visibility.lock()
        // //             {
        // //                 *main_window_visibility = false;
        // //             };
        // //         }
        // //     }
        // //     _ => (),
        // // })
        // .invoke_handler(tauri::generate_handler![
        //     show_in_folder,
        //     get_clipboard_list,
        //     show_main_window,
        //     hide_main_window,
        //     toggle_main_window
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
