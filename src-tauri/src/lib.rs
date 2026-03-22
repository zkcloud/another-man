mod models;
mod core;

use models::{HttpRequest, HttpResponse, ResponseError};
use core::HttpClient;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn send_http_request(request: HttpRequest) -> Result<HttpResponse, ResponseError> {
    let client = HttpClient::new();
    client.send(request).await
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            send_http_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}