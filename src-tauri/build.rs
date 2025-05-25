fn main() {
    let microsoft_client_id = match std::env::var("MICROSOFT_CLIENT_ID") {
        Ok(id) => id,
        Err(_) => {
            panic!("MICROSOFT_CLIENT_ID is not defined");
        }
    };

    if microsoft_client_id.is_empty() {
        panic!("MICROSOFT_CLIENT_ID is not defined");
    }

    tauri_build::build()
}
