use tauri::AppHandle;
use tauri_plugin_deep_link::{DeepLinkExt, OpenUrlEvent};
use thiserror::Error;

use crate::account;

#[derive(Error, Debug)]
pub enum DeeplinkSetupError {
    #[error("Failed to register deep link scheme")]
    RegisterDeepLinkSchemeError(#[from] tauri_plugin_deep_link::Error),
}

pub static DEEPLINK_SCHEME: &str = "vault-launcher";

fn handle_deep_link(event: OpenUrlEvent, app_handle: &AppHandle) {
    let urls: Vec<tauri::Url> = event.urls().into_iter().collect();

    for url in urls {
        if url.scheme() != DEEPLINK_SCHEME {
            log::warn!("Unsupported deep link scheme used: {}", url.scheme());
            continue;
        }

        if let Some(host) = url.host() {
            match host.to_string().as_str() {
                "account" => {
                    account::handle_deep_link(app_handle, url);
                }
                _ => (),
            }
        }
    }
}

pub fn setup(app_handle: &AppHandle) -> Result<(), DeeplinkSetupError> {
    #[cfg(any(windows, target_os = "linux"))]
    {
        app_handle.deep_link().register_all()?;
    }

    let app_handle_clone = app_handle.clone();
    app_handle
        .deep_link()
        .on_open_url(move |event| handle_deep_link(event, &app_handle_clone));

    Ok(())
}
