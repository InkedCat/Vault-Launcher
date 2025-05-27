use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let microsoft_client_id =
        std::env::var("MICROSOFT_CLIENT_ID").expect("MICROSOFT_CLIENT_ID is not defined");

    if microsoft_client_id.is_empty() {
        panic!("MICROSOFT_CLIENT_ID is not defined");
    }

    println!(
        "cargo:rustc-env=MICROSOFT_CLIENT_ID={}",
        microsoft_client_id
    );

    tauri_build::build()
}
