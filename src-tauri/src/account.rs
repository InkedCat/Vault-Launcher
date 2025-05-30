pub mod microsoft;

fn handle_login_callback(app_handle: tauri::AppHandle, url: tauri::Url) {
    match url.query() {
        Some(query) => {
            let query = query.to_string();
            tauri::async_runtime::spawn(async move {
                let _ = microsoft::handle_callback(&app_handle, &query).await;
            });
        }
        None => log::error!("No query parameters found"),
    }
}

pub fn handle_deep_link(app_handle: &tauri::AppHandle, url: tauri::Url) {
    let app_handle = app_handle.clone();
    match url.path() {
        "/login/callback" => handle_login_callback(app_handle, url),
        _ => log::error!("Path not found"),
    }
}
