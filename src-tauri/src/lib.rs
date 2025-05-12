mod auth;
mod tray_icon;

use tauri::{AppHandle, Manager, Window, WindowEvent};
use tauri_plugin_deep_link::DeepLinkExt;

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

async fn setup(app: &AppHandle) -> Result<(), tauri_plugin_deep_link::Error> {
    #[cfg(any(windows, target_os = "linux"))]
    {
        app.deep_link().register_all()?;
    }
    app.deep_link().on_open_url(|event| {
        println!("Open URL: {:?}", event.urls());
    });
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main")
                       .expect("no main window")
                       .set_focus();
        }));
    }

    builder
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .on_window_event(handle_window_event)
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            tauri::async_runtime::block_on(setup(app.handle()))?;
            tray_icon::enable_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
