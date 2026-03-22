mod models;
mod core;
mod db;

use models::{HttpRequest, HttpResponse, ResponseError};
use core::HttpClient;
use db::{Database, HistoryEntry, Collection, SavedRequest, CreateCollectionInput, CreateRequestInput};
use std::sync::Mutex;
use tauri::Manager;

// Global database instance
struct AppState {
    db: Mutex<Option<Database>>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn send_http_request(
    request: HttpRequest,
    state: tauri::State<'_, AppState>
) -> Result<HttpResponse, ResponseError> {
    let client = HttpClient::new();
    let response = client.send(request.clone()).await;
    
    // Save to history
    if let Ok(ref resp) = response {
        if let Ok(mut db_guard) = state.db.lock() {
            if let Some(ref db) = *db_guard {
                let _ = db.save_history(&request, Some(resp));
                // Keep only last 1000 entries
                let _ = db.delete_old_history(1000);
            }
        }
    }
    
    response
}

#[tauri::command]
fn get_history(
    limit: i64,
    state: tauri::State<'_, AppState>
) -> Result<Vec<HistoryEntry>, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.get_history(limit).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { db: Mutex::new(None) })
        .setup(|app| {
            // Initialize database
            let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
            
            let db = Database::new(app_dir).expect("Failed to initialize database");
            
            if let Ok(mut db_guard) = app.state::<AppState>().db.lock() {
                *db_guard = Some(db);
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            send_http_request,
            get_history,
            create_collection,
            get_collections,
            update_collection,
            delete_collection,
            create_saved_request,
            get_requests_by_collection,
            delete_saved_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}