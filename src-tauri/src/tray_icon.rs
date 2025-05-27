use tauri::{
    menu::{MenuBuilder, MenuEvent, MenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewWindow,
};
use thiserror::Error;

use crate::MAIN_WINDOW_LABEL;

#[derive(Error, Debug)]
pub enum TrayIconSetupError {
    #[error("failed to create tray icon: {0}")]
    CreateTrayIconError(#[from] tauri::Error),
}

fn menu_handler(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "quit" => {
            app.exit(0);
        }
        _ => {
            log::warn!("unhandled menu item {:?} called", event.id);
        }
    }
}
fn show_window(window: WebviewWindow) {
    match window.is_visible() {
        Ok(true) => (),
        Ok(false) => {
            if let Err(e) = window.show() {
                log::error!("failed to show {MAIN_WINDOW_LABEL} window: {:?}", e);
            }

            if let Err(e) = window.set_focus() {
                log::error!("failed to focus {MAIN_WINDOW_LABEL} window: {:?}", e);
            }
        }
        Err(e) => {
            log::error!("failed to check if window is visible: {:?}", e);
        }
    }
}

#[allow(clippy::single_match)]
fn tray_icon_handler(icon: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click { .. } => {
            if let Some(window) = icon.app_handle().get_webview_window(MAIN_WINDOW_LABEL) {
                show_window(window);
            } else {
                log::error!("failed to retrieve {MAIN_WINDOW_LABEL} window");
            }
        }
        _ => (),
    }
}

pub fn setup(app: &mut tauri::App) -> Result<(), TrayIconSetupError> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = MenuBuilder::new(app).item(&quit_i).build()?;

    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(menu_handler)
        .on_tray_icon_event(tray_icon_handler)
        .build(app)?;

    Ok(())
}
