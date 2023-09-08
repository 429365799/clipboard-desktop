use crate::structs::states::AppState;
use tauri::{Runtime, State};

#[tauri::command]
pub(crate) async fn toggle_main_window<R: Runtime>(
    window: tauri::Window<R>,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    if let Ok(mut main_window_visibility) = app_state.main_window_visibility.lock() {
        let visible = *main_window_visibility;
        *main_window_visibility = !visible;

        let err_fn = |e| format!("Failed to hide main window. error code: {}", e);
        if visible {
            window.hide().map_err(err_fn)?;
        } else {
            window.show().map_err(err_fn)?;
            window.set_focus().map_err(err_fn)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub(crate) async fn show_main_window<R: Runtime>(
    window: tauri::Window<R>,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    match window.show() {
        Ok(()) => {
            if let Ok(mut main_window_visibility) = app_state.main_window_visibility.lock() {
                *main_window_visibility = true;
            }
            Ok(())
        }
        Err(e) => Err(format!("Hide window error: {}", e)),
    }
}

#[tauri::command]
pub(crate) async fn hide_main_window<R: Runtime>(
    window: tauri::Window<R>,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    match window.hide() {
        Ok(()) => {
            if let Ok(mut main_window_visibility) = app_state.main_window_visibility.lock() {
                *main_window_visibility = false;
            }
            Ok(())
        }
        Err(e) => Err(format!("Hide window error: {}", e)),
    }
}
