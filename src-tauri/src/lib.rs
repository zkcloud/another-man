mod models;
mod core;
mod db;
mod script;

use models::{HttpRequest, HttpResponse, ResponseError};
use core::{HttpClient, TestReportGenerator, TestReport, TestSummary, SuiteReport, TestResultReport, VariableExtractor, VariableExtraction};
use db::{
    Database, HistoryEntry, Collection, SavedRequest, 
    CreateCollectionInput, CreateRequestInput, 
    Environment, CreateEnvironmentInput, ExportedCollection,
    TestSuite, TestCase, TestDatabase
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

// File operations
#[tauri::command]
async fn download_file(
    url: String,
    save_path: String,
) -> Result<String, String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to download: {}", e))?;
    
    let bytes = response.bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    std::fs::write(&save_path, &bytes)
        .map_err(|e| format!("Failed to save file: {}", e))?;
    
    Ok(save_path)
}

#[tauri::command]
fn select_file() -> Result<Option<String>, String> {
    // This would use tauri-plugin-dialog in a real implementation
    // For now, return None
    Ok(None)
}

// Test Suite commands
#[tauri::command]
fn create_test_suite(
    name: String,
    description: Option<String>,
    collection_id: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<TestSuite, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            let test_db = TestDatabase::new(db.get_conn());
            return test_db.create_suite(&name, description.as_deref(), collection_id.as_deref())
                .map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn get_test_suites(state: tauri::State<'_, AppState>) -> Result<Vec<TestSuite>, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            let test_db = TestDatabase::new(db.get_conn());
            return test_db.get_suites().map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn delete_test_suite(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            let test_db = TestDatabase::new(db.get_conn());
            return test_db.delete_suite(&id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

// Test Case commands
#[tauri::command]
fn create_test_case(
    suite_id: String,
    name: String,
    request_id: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<TestCase, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            let test_db = TestDatabase::new(db.get_conn());
            return test_db.create_case(&suite_id, &name, request_id.as_deref())
                .map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn get_test_cases(
    suite_id: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<TestCase>, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            let test_db = TestDatabase::new(db.get_conn());
            return test_db.get_cases_by_suite(&suite_id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn update_test_case_script(
    id: String,
    pre_script: Option<String>,
    test_script: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            let test_db = TestDatabase::new(db.get_conn());
            return test_db.update_case_script(&id, pre_script.as_deref(), test_script.as_deref())
                .map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

#[tauri::command]
fn delete_test_case(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            let test_db = TestDatabase::new(db.get_conn());
            return test_db.delete_case(&id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

// Test Run commands
#[tauri::command]
fn get_test_runs(
    suite_id: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<crate::db::test_suites::TestRun>, String> {
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            let test_db = TestDatabase::new(db.get_conn());
            return test_db.get_runs_by_suite(&suite_id).map_err(|e| e.to_string());
        }
    }
    Err("Database not initialized".to_string())
}

// Test Report command
#[tauri::command]
fn generate_test_report(
    suite_id: String,
    format: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    use crate::db::test_suites::TestDatabase;
    
    if let Ok(db_guard) = state.db.lock() {
        if let Some(ref db) = *db_guard {
            let test_db = TestDatabase::new(db.get_conn());
            
            // Get suite info
            let suites = test_db.get_suites().map_err(|e| e.to_string())?;
            let suite = suites.iter().find(|s| s.id == suite_id)
                .ok_or_else(|| "Suite not found".to_string())?;
            
            // Get runs
            let runs = test_db.get_runs_by_suite(&suite_id).map_err(|e| e.to_string())?;
            
            // Build report
            let mut total_tests = 0i32;
            let mut total_passed = 0i32;
            let mut total_failed = 0i32;
            let mut total_skipped = 0i32;
            let mut total_duration = 0i64;
            
            for run in runs.iter().take(10) {
                total_tests += run.total_tests;
                total_passed += run.passed;
                total_failed += run.failed;
                total_skipped += run.skipped;
                total_duration += run.duration_ms;
            }
            
            let success_rate = if total_tests > 0 {
                (total_passed as f64 / total_tests as f64) * 100.0
            } else {
                0.0
            };
            
            let report = TestReport {
                summary: TestSummary {
                    total_suites: 1,
                    total_tests,
                    passed: total_passed,
                    failed: total_failed,
                    skipped: total_skipped,
                    duration_ms: total_duration,
                    success_rate,
                },
                suites: vec![
                    SuiteReport {
                        suite_id: suite_id.clone(),
                        suite_name: suite.name.clone(),
                        tests: vec![],
                        summary: TestSummary {
                            total_suites: 1,
                            total_tests,
                            passed: total_passed,
                            failed: total_failed,
                            skipped: total_skipped,
                            duration_ms: total_duration,
                            success_rate,
                        },
                    }
                ],
                generated_at: chrono::Utc::now().timestamp(),
            };
            
            let generator = TestReportGenerator::new();
            match format.as_str() {
                "html" => generator.generate_html(&report),
                "json" => generator.generate_json(&report),
                _ => Err("Unsupported format".to_string()),
            }
        } else {
            Err("Database not initialized".to_string())
        }
    } else {
        Err("Database not initialized".to_string())
    }
}

// Variable extraction commands
#[tauri::command]
fn extract_variables(
    body: String,
    headers: std::collections::HashMap<String, String>,
    extractions: Vec<String>,
) -> Result<std::collections::HashMap<String, String>, String> {
    let extractor = VariableExtractor::new();
    
    let parsed_extractions: Vec<VariableExtraction> = extractions
        .iter()
        .filter_map(|e| extractor.parse_extraction(e))
        .collect();
    
    Ok(extractor.extract_variables(&body, &headers, &parsed_extractions))
}

#[tauri::command]
fn extract_json_path(body: String, path: String) -> Result<Option<String>, String> {
    let json: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    
    let extractor = VariableExtractor::new();
    Ok(extractor.extract_json_path(&json, &path))
}

#[tauri::command]
fn extract_regex(body: String, pattern: String, group: usize) -> Result<Option<String>, String> {
    let extractor = VariableExtractor::new();
    Ok(extractor.extract_regex(&body, &pattern, group))
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
            validate_script,
            download_file,
            select_file,
            create_test_suite,
            get_test_suites,
            delete_test_suite,
            create_test_case,
            get_test_cases,
            update_test_case_script,
            delete_test_case,
            get_test_runs,
            generate_test_report,
            extract_variables,
            extract_json_path,
            extract_regex
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
