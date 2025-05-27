mod account;
mod deeplink;
mod states;
mod tray_icon;
mod utils;

use tauri::{Manager, Window, WindowEvent, Wry};

use account::microsoft;

pub static MAIN_WINDOW_LABEL: &str = "main";

#[allow(clippy::single_match)]
fn handle_window_event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => match window.hide() {
            Ok(_) => {
                api.prevent_close();
            }
            Err(e) => {
                log::error!("failed to minimize window: {:?}", e);
            }
        },
        _ => {}
    }
}

fn setup_plugins(builder: tauri::Builder<Wry>) -> tauri::Builder<Wry> {
    builder
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
}

pub fn register_commands(builder: tauri::Builder<Wry>) -> tauri::Builder<Wry> {
    builder.invoke_handler(tauri::generate_handler!(microsoft::open_microsoft_oauth))
}

pub fn run() {
    let mut builder = tauri::Builder::default();

    builder = setup_plugins(builder);
    builder = register_commands(builder);

    builder
        .on_window_event(handle_window_event)
        .setup(|app| {
            deeplink::setup(app.handle())?;
            states::create_states(app.handle());
            tray_icon::setup(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
