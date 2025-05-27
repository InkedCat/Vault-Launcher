pub mod login;
pub mod microsoft;
pub mod xbox;

const MICROSOFT_OAUTH_API_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0";
const XBOX_LIVE_URL: &str = "https://user.auth.xboxlive.com/";

const MICROSOFT_CLIENT_ID: &str = env!("MICROSOFT_CLIENT_ID", "MICROSOFT_CLIENT_ID is not defined");
const REDIRECT_URI: &str = "vault-launcher://account/login/callback";

fn handle_login_callback(app_handle: tauri::AppHandle, url: tauri::Url) {
    match url.query() {
        Some(query) => {
            let query = query.to_string();
            tauri::async_runtime::spawn(async move {
                let _ = login::handle_callback(&app_handle, &query).await;
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
