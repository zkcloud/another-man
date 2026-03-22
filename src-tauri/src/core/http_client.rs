use crate::models::{HttpMethod, HttpRequest, HttpResponse, RequestBody, ResponseError, ErrorKind};
use std::time::Instant;

pub struct HttpClient;

impl HttpClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn send(&self, request: HttpRequest) -> Result<HttpResponse, ResponseError> {
        let start = Instant::now();
        
        let client = reqwest::Client::new();
        
        let method = match request.method {
            HttpMethod::GET => reqwest::Method::GET,
            HttpMethod::POST => reqwest::Method::POST,
            HttpMethod::PUT => reqwest::Method::PUT,
            HttpMethod::DELETE => reqwest::Method::DELETE,
            HttpMethod::PATCH => reqwest::Method::PATCH,
            HttpMethod::HEAD => reqwest::Method::HEAD,
            HttpMethod::OPTIONS => reqwest::Method::OPTIONS,
        };
        
        let mut req_builder = client.request(method, &request.url);
        
        // Add headers
        for header in &request.headers {
            if header.enabled {
                req_builder = req_builder.header(&header.key, &header.value);
            }
        }
        
        // Add query params
        if !request.query_params.is_empty() {
            let query: Vec<(String, String)> = request.query_params
                .iter()
                .filter(|p| p.enabled)
                .map(|p| (p.key.clone(), p.value.clone()))
                .collect();
            req_builder = req_builder.query(&query);
        }
        
        // Add body
        match &request.body {
            Some(RequestBody::Json(body)) => {
                req_builder = req_builder.header("Content-Type", "application/json");
                req_builder = req_builder.body(body.clone());
            }
            Some(RequestBody::Text(body)) => {
                req_builder = req_builder.body(body.clone());
            }
            Some(RequestBody::FormData(data)) => {
                let form: Vec<(String, String)> = data.clone();
                req_builder = req_builder.form(&form);
            }
            _ => {}
        }
        
        // Send request
        let response = match req_builder.send().await {
            Ok(resp) => resp,
            Err(e) => {
                return Err(ResponseError {
                    message: format!("Request failed: {}", e),
                    kind: ErrorKind::Network,
                });
            }
        };
        
        let status = response.status();
        let status_text = status.canonical_reason().unwrap_or("Unknown").to_string();
        
        // Get headers
        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(k, v)| {
                let value = v.to_str().unwrap_or("").to_string();
                (k.to_string(), value)
            })
            .collect();
        
        // Get body
        let body = match response.text().await {
            Ok(text) => text,
            Err(e) => {
                return Err(ResponseError {
                    message: format!("Failed to read response body: {}", e),
                    kind: ErrorKind::Network,
                });
            }
        };
        
        let time_ms = start.elapsed().as_millis() as u64;
        let size_bytes = body.len();
        
        Ok(HttpResponse {
            status: status.as_u16(),
            status_text,
            headers,
            body,
            time_ms,
            size_bytes,
        })
    }
}