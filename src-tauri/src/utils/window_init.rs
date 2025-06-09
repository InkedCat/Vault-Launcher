use std::sync::OnceLock;

use lazy_static::lazy_static;

lazy_static! {
    static ref WINDOW_INITIALIZED: OnceLock<bool> = OnceLock::new();
}

#[tauri::command]
pub fn init_window(webview_window: tauri::WebviewWindow) {
    if WINDOW_INITIALIZED.get().is_some_and(|value| *value) {
        return;
    }

    if let Err(e) = webview_window.show() {
        log::error!("failed to show window: {:?}", e);
    }
    if let Err(e) = webview_window.set_focus() {
        log::error!("failed to set focus: {:?}", e);
    }

    WINDOW_INITIALIZED
        .set(true)
        .expect("WINDOW_INITIALIZED should not be set");
}
