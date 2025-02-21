mod commands;
mod tray_icon;

use tauri::{Window, WindowEvent};

pub static MAIN_WINDOW_LABEL: &str = "main";

fn handle_window_event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            if let Err(e) = window.hide() {
                eprintln!("Failed to minimize window: {:?}", e);
            }
            api.prevent_close();
        }
        _ => {}
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_window_event(handle_window_event)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            tray_icon::enable_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
