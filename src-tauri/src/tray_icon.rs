use tauri::{
    menu::{MenuBuilder, MenuEvent, MenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewWindow,
};

use crate::MAIN_WINDOW_LABEL;

fn menu_handler(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "quit" => {
            println!("Quit menu item clicked");
            app.exit(0);
        }
        _ => {
            println!("Menu item {:?} not handled", event.id);
        }
    }
}

fn tray_icon_handler(icon: &TrayIcon, event: TrayIconEvent) -> () {
    fn toggle_window(window: WebviewWindow) -> () {
        match window.is_visible() {
            Ok(true) => {}
            Ok(false) => {
                if let Err(e) = window.show() {
                    eprintln!("Failed to show window: {:?}", e);
                }

                if let Err(e) = window.set_focus() {
                    eprintln!("Failed to focus window: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to check if window is visible: {:?}", e);
            }
        }
    }

    match event {
        TrayIconEvent::Click { .. } => {
            if let Some(window) = icon.app_handle().get_webview_window(MAIN_WINDOW_LABEL) {
                toggle_window(window);
            } else {
                eprintln!("Failed to get window");
            }
        }
        _ => (),
    }
}

pub fn enable_tray(app: &mut tauri::App) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = MenuBuilder::new(app).item(&quit_i).build().unwrap();

    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(menu_handler)
        .on_tray_icon_event(tray_icon_handler)
        .build(app)?;

    return Ok(());
}
