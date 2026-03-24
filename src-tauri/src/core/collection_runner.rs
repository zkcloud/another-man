use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::models::{HttpRequest, HttpResponse};
use crate::core::HttpClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunConfig {
    pub collection_id: String,
    pub environment_id: Option<String>,
    pub iteration_count: u32,
    pub delay_ms: u64,
    pub stop_on_error: bool,
    pub concurrent_requests: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResult {
    pub run_id: String,
    pub status: RunStatus,
    pub total_requests: u32,
    pub completed_requests: u32,
    pub failed_requests: u32,
    pub results: Vec<RequestResult>,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub duration_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestStatus {
    Pending,
    Running,
    Success,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestResult {
    pub request_id: String,
    pub request_name: String,
    pub status: RequestStatus,
    pub response: Option<HttpResponse>,
    pub error: Option<String>,
    pub duration_ms: i64,
    pub started_at: i64,
}

pub struct CollectionRunner;

impl CollectionRunner {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_collection(
        &self,
        config: RunConfig,
        requests: Vec<(String, String, HttpRequest)>,
        env_vars: HashMap<String, String>,
    ) -> Result<RunResult, String> {
        let started_at = chrono::Utc::now().timestamp_millis();
        let run_id = uuid::Uuid::new_v4().to_string();
        
        let total_requests = requests.len() as u32 * config.iteration_count;
        let mut all_results = Vec::new();
        let mut total_completed = 0u32;
        let mut total_failed = 0u32;

        for _iteration in 0..config.iteration_count {
            let iteration_results = self.run_sequential(&requests, &env_vars, config.delay_ms).await?;
            
            for result in iteration_results {
                match result.status {
                    RequestStatus::Success => total_completed += 1,
                    RequestStatus::Failed => total_failed += 1,
                    _ => {}
                }
                all_results.push(result);
            }
            
            if config.stop_on_error && total_failed > 0 {
                break;
            }
        }

        let completed_at = chrono::Utc::now().timestamp_millis();
        let duration_ms = completed_at - started_at;

        Ok(RunResult {
            run_id,
            status: if total_failed > 0 { RunStatus::Failed } else { RunStatus::Completed },
            total_requests,
            completed_requests: total_completed,
            failed_requests: total_failed,
            results: all_results,
            started_at: started_at / 1000,
            completed_at: Some(completed_at / 1000),
            duration_ms,
        })
    }

    async fn run_sequential(
        &self,
        requests: &[(String, String, HttpRequest)],
        env_vars: &HashMap<String, String>,
        delay_ms: u64,
    ) -> Result<Vec<RequestResult>, String> {
        let http_client = HttpClient::new();
        let mut results = Vec::new();
        
        for (req_id, req_name, request) in requests {
            let req_start = chrono::Utc::now().timestamp_millis();
            let resolved = Self::resolve_request(request, env_vars);
            let response_result = http_client.send(resolved).await;
            
            let (status, response, error) = match response_result {
                Ok(resp) => (RequestStatus::Success, Some(resp), None),
                Err(e) => (RequestStatus::Failed, None, Some(format!("{:?}", e))),
            };
            
            let duration_ms = chrono::Utc::now().timestamp_millis() - req_start;
            
            results.push(RequestResult {
                request_id: req_id.clone(),
                request_name: req_name.clone(),
                status,
                response,
                error,
                duration_ms,
                started_at: req_start / 1000,
            });
            
            if delay_ms > 0 {
                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            }
        }
        
        Ok(results)
    }

    fn resolve_request(request: &HttpRequest, vars: &HashMap<String, String>) -> HttpRequest {
        let mut resolved = request.clone();
        resolved.url = Self::resolve_variables(&request.url, vars);
        resolved
    }

    fn resolve_variables(text: &str, vars: &HashMap<String, String>) -> String {
        let mut result = text.to_string();
        for (key, value) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_variables() {
        let vars = HashMap::from([
            ("baseUrl".to_string(), "https://api.example.com".to_string()),
            ("token".to_string(), "abc123".to_string()),
        ]);
        
        let result = CollectionRunner::resolve_variables("{{baseUrl}}/users", &vars);
        assert_eq!(result, "https://api.example.com/users");
        
        let result = CollectionRunner::resolve_variables("Bearer {{token}}", &vars);
        assert_eq!(result, "Bearer abc123");
    }

    #[test]
    fn test_run_config_creation() {
        let config = RunConfig {
            collection_id: "col-1".to_string(),
            environment_id: None,
            iteration_count: 1,
            delay_ms: 0,
            stop_on_error: false,
            concurrent_requests: 1,
        };
        
        assert_eq!(config.iteration_count, 1);
        assert_eq!(config.concurrent_requests, 1);
    }

    #[test]
    fn test_request_status_variants() {
        assert!(matches!(RequestStatus::Pending, RequestStatus::Pending));
        assert!(matches!(RequestStatus::Success, RequestStatus::Success));
        assert!(matches!(RequestStatus::Failed, RequestStatus::Failed));
    }

    #[test]
    fn test_run_status_variants() {
        assert!(matches!(RunStatus::Running, RunStatus::Running));
        assert!(matches!(RunStatus::Completed, RunStatus::Completed));
        assert!(matches!(RunStatus::Failed, RunStatus::Failed));
    }
}
