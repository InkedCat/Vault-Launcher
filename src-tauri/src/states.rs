use std::sync::Mutex;

use tauri::Manager;

#[derive(Debug, Clone)]
pub struct AuthData {
    pub state: Option<String>,
    pub code_verifier: Option<String>,
}

pub fn create_states(app_handle: &tauri::AppHandle) {
    let auth_data = Mutex::new(AuthData {
        state: None,
        code_verifier: None,
    });

    app_handle.manage(auth_data);
}
