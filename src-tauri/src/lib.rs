mod models;
mod core;
mod db;
mod script;

use models::{HttpRequest, HttpResponse, ResponseError};
use core::HttpClient;
use db::{
    Database, HistoryEntry, Collection, SavedRequest, 
    CreateCollectionInput, CreateRequestInput, 
    Environment, CreateEnvironmentInput, ExportedCollection
};
use script::{ScriptEngine, ScriptExecutionResult};
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
        if let Ok(db_guard) = state.db.lock() {
            if let Some(ref db) = *db_guard {
                let _ = db.save_history(&request, Some(resp));
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

// Collection commands
#[tauri::command]
fn create_collection(
    input: CreateCollectionInput,
    state: tauri::State<'_, AppState>
) -> Result<Collection, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.create_collection(input).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn get_collections(state: tauri::State<'_, AppState>) -> Result<Vec<Collection>, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.get_collections().map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn update_collection(
    id: String,
    name: String,
    description: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.update_collection(&id, &name, description.as_deref()).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn delete_collection(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.delete_collection(&id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

// Saved Request commands
#[tauri::command]
fn create_saved_request(
    input: CreateRequestInput,
    state: tauri::State<'_, AppState>
) -> Result<SavedRequest, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.create_request(input).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn get_requests_by_collection(
    collection_id: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<SavedRequest>, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.get_requests_by_collection(&collection_id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn delete_saved_request(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.delete_request(&id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

// Environment commands
#[tauri::command]
fn create_environment(
    input: CreateEnvironmentInput,
    state: tauri::State<'_, AppState>
) -> Result<Environment, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.create_environment(input).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn get_environments(state: tauri::State<'_, AppState>) -> Result<Vec<Environment>, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.get_environments().map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn get_environment(id: String, state: tauri::State<'_, AppState>) -> Result<Option<Environment>, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.get_environment(&id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn update_environment(
    id: String,
    name: String,
    variables: std::collections::HashMap<String, String>,
    is_global: bool,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.update_environment(&id, &name, &variables, is_global).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn delete_environment(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.delete_environment(&id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn get_active_environment(state: tauri::State<'_, AppState>) -> Result<Option<Environment>, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.get_active_environment().map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

// Export/Import commands
#[tauri::command]
fn export_collection(
    collection_id: String,
    state: tauri::State<'_, AppState>
) -> Result<ExportedCollection, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.export_collection(&collection_id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn import_collection(
    imported: ExportedCollection,
    state: tauri::State<'_, AppState>
) -> Result<Collection, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            return db.import_collection(&imported).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

// Script commands
#[tauri::command]
fn execute_pre_request_script(
    script: String,
    env_id: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<std::collections::HashMap<String, String>, String> {
    let engine = ScriptEngine::new();
    
    // Get current environment variables
    let env_vars = if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            if let Some(env_id) = env_id {
                db.get_environment(&env_id)
                    .map(|e| e.map(|env| env.variables).unwrap_or_default())
                    .unwrap_or_default()
            } else {
                db.get_active_environment()
                    .map(|e| e.map(|env| env.variables).unwrap_or_default())
                    .unwrap_or_default()
            }
        } else {
            std::collections::HashMap::new()
        }
    } else {
        std::collections::HashMap::new()
    };
    
    engine.execute_pre_request(&script, env_vars)
}

#[tauri::command]
fn execute_test_script(
    script: String,
    response: HttpResponse,
    env_id: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<ScriptExecutionResult, String> {
    let engine = ScriptEngine::new();
    
    // Get current environment variables
    let env_vars = if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            if let Some(env_id) = env_id {
                db.get_environment(&env_id)
                    .map(|e| e.map(|env| env.variables).unwrap_or_default())
                    .unwrap_or_default()
            } else {
                db.get_active_environment()
                    .map(|e| e.map(|env| env.variables).unwrap_or_default())
                    .unwrap_or_default()
            }
        } else {
            std::collections::HashMap::new()
        }
    } else {
        std::collections::HashMap::new()
    };
    
    engine.execute_test(&script, response, env_vars)
}

#[tauri::command]
fn validate_script(script: String) -> Result<(), String> {
    let engine = ScriptEngine::new();
    engine.validate_script(&script)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
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
            delete_saved_request,
            create_environment,
            get_environments,
            get_environment,
            update_environment,
            delete_environment,
            get_active_environment,
            export_collection,
            import_collection,
            execute_pre_request_script,
            execute_test_script,
            validate_script
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
