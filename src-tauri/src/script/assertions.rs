use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Assertion {
    pub name: String,
    pub passed: bool,
    pub expected: String,
    pub actual: String,
    pub error: Option<String>,
}

pub struct AssertionEngine;

impl AssertionEngine {
    pub fn new() -> Self {
        Self
    }

    // Status code assertions
    pub fn status_code(&self, actual: u16, expected: u16) -> Assertion {
        let passed = actual == expected;
        Assertion {
            name: format!("Status code is {}", expected),
            passed,
            expected: expected.to_string(),
            actual: actual.to_string(),
            error: if passed { None } else { Some(format!("Expected status {}, got {}", expected, actual)) },
        }
    }

    pub fn status_success(&self, actual: u16) -> Assertion {
        let passed = actual >= 200 && actual < 300;
        Assertion {
            name: "Status code is 2xx".to_string(),
            passed,
            expected: "200-299".to_string(),
            actual: actual.to_string(),
            error: if passed { None } else { Some(format!("Expected success status (2xx), got {}", actual)) },
        }
    }

    pub fn status_code_in_range(&self, actual: u16, min: u16, max: u16) -> Assertion {
        let passed = actual >= min && actual <= max;
        Assertion {
            name: format!("Status code is between {} and {}", min, max),
            passed,
            expected: format!("{}-{}", min, max),
            actual: actual.to_string(),
            error: if passed { None } else { Some(format!("Expected status in range {}-{}, got {}", min, max, actual)) },
        }
    }

    // Header assertions
    pub fn header_exists(&self, headers: &HashMap<String, String>, name: &str) -> Assertion {
        let passed = headers.contains_key(&name.to_lowercase());
        Assertion {
            name: format!("Header '{}' exists", name),
            passed,
            expected: name.to_string(),
            actual: if passed { "exists".to_string() } else { "missing".to_string() },
            error: if passed { None } else { Some(format!("Header '{}' not found", name)) },
        }
    }

    pub fn header_equals(&self, headers: &HashMap<String, String>, name: &str, expected: &str) -> Assertion {
        let actual_value = headers.get(&name.to_lowercase()).cloned().unwrap_or_default();
        let passed = actual_value == expected;
        Assertion {
            name: format!("Header '{}' equals '{}'", name, expected),
            passed,
            expected: expected.to_string(),
            actual: actual_value.clone(),
            error: if passed { None } else { Some(format!("Header '{}' expected '{}', got '{}'", name, expected, actual_value)) },
        }
    }

    pub fn header_contains(&self, headers: &HashMap<String, String>, name: &str, substring: &str) -> Assertion {
        let actual_value = headers.get(&name.to_lowercase()).cloned().unwrap_or_default();
        let passed = actual_value.contains(substring);
        Assertion {
            name: format!("Header '{}' contains '{}'", name, substring),
            passed,
            expected: format!("contains '{}'", substring),
            actual: actual_value,
            error: if passed { None } else { Some(format!("Header '{}' does not contain '{}'", name, substring)) },
        }
    }

    // Body assertions
    pub fn body_contains(&self, body: &str, substring: &str) -> Assertion {
        let passed = body.contains(substring);
        Assertion {
            name: format!("Body contains '{}'", substring),
            passed,
            expected: format!("contains '{}'", substring),
            actual: if body.len() > 100 { format!("{}...", &body[..100]) } else { body.to_string() },
            error: if passed { None } else { Some(format!("Body does not contain '{}'", substring)) },
        }
    }

    pub fn body_not_empty(&self, body: &str) -> Assertion {
        let passed = !body.is_empty();
        Assertion {
            name: "Body is not empty".to_string(),
            passed,
            expected: "non-empty".to_string(),
            actual: if body.is_empty() { "empty".to_string() } else { format!("{} bytes", body.len()) },
            error: if passed { None } else { Some("Body is empty".to_string()) },
        }
    }

    // JSON assertions
    pub fn json_path_exists(&self, json: &Value, path: &str) -> Assertion {
        let value = self.get_json_path(json, path);
        let passed = value.is_some() && !value.unwrap().is_null();
        Assertion {
            name: format!("JSON path '{}' exists", path),
            passed,
            expected: path.to_string(),
            actual: if passed { "exists".to_string() } else { "missing".to_string() },
            error: if passed { None } else { Some(format!("JSON path '{}' not found", path)) },
        }
    }

    pub fn json_path_equals(&self, json: &Value, path: &str, expected: &str) -> Assertion {
        let actual_opt = self.get_json_path(json, path).map(|v| v.to_string());
        let actual_value = actual_opt.unwrap_or_default();
        let passed = actual_value == expected;
        Assertion {
            name: format!("JSON path '{}' equals '{}'", path, expected),
            passed,
            expected: expected.to_string(),
            actual: actual_value.clone(),
            error: if passed { None } else { Some(format!("JSON path '{}' expected '{}', got '{}'", path, expected, actual_value)) },
        }
    }

    pub fn json_is_object(&self, json: &Value) -> Assertion {
        let passed = json.is_object();
        Assertion {
            name: "Response is a JSON object".to_string(),
            passed,
            expected: "object".to_string(),
            actual: self.get_json_type(json),
            error: if passed { None } else { Some("Response is not a JSON object".to_string()) },
        }
    }

    pub fn json_is_array(&self, json: &Value) -> Assertion {
        let passed = json.is_array();
        Assertion {
            name: "Response is a JSON array".to_string(),
            passed,
            expected: "array".to_string(),
            actual: self.get_json_type(json),
            error: if passed { None } else { Some("Response is not a JSON array".to_string()) },
        }
    }

    // Time assertions
    pub fn response_time_less_than(&self, actual_ms: i64, expected_ms: i64) -> Assertion {
        let passed = actual_ms < expected_ms;
        Assertion {
            name: format!("Response time is less than {}ms", expected_ms),
            passed,
            expected: format!("<{}ms", expected_ms),
            actual: format!("{}ms", actual_ms),
            error: if passed { None } else { Some(format!("Response time {}ms exceeds limit of {}ms", actual_ms, expected_ms)) },
        }
    }

    // Helper methods
    fn get_json_path(&self, json: &Value, path: &str) -> Option<Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = json;
        
        for part in parts {
            if let Ok(index) = part.parse::<usize>() {
                if let Some(arr) = current.as_array() {
                    current = arr.get(index)?;
                } else {
                    return None;
                }
            } else {
                current = current.get(part)?;
            }
        }
        
        Some(current.clone())
    }

    fn get_json_type(&self, json: &Value) -> String {
        match json {
            Value::Null => "null".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::Number(_) => "number".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::Object(_) => "object".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_assertion() {
        let engine = AssertionEngine::new();
        
        // Pass case
        let result = engine.status_code(200, 200);
        assert!(result.passed);
        
        // Fail case
        let result = engine.status_code(404, 200);
        assert!(!result.passed);
    }

    #[test]
    fn test_status_success_assertion() {
        let engine = AssertionEngine::new();
        
        assert!(engine.status_success(200).passed);
        assert!(engine.status_success(201).passed);
        assert!(!engine.status_success(404).passed);
    }

    #[test]
    fn test_header_assertions() {
        let engine = AssertionEngine::new();
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        
        assert!(engine.header_exists(&headers, "content-type").passed);
        assert!(!engine.header_exists(&headers, "x-custom").passed);
        
        assert!(engine.header_equals(&headers, "content-type", "application/json").passed);
        assert!(!engine.header_equals(&headers, "content-type", "text/html").passed);
    }

    #[test]
    fn test_body_assertions() {
        let engine = AssertionEngine::new();
        
        assert!(engine.body_contains("Hello World", "World").passed);
        assert!(!engine.body_contains("Hello World", "Foo").passed);
        
        assert!(engine.body_not_empty("content").passed);
        assert!(!engine.body_not_empty("").passed);
    }

    #[test]
    fn test_json_path_assertions() {
        let engine = AssertionEngine::new();
        let json: Value = serde_json::from_str(r#"{"data": {"name": "John"}}"#).unwrap();
        
        assert!(engine.json_path_exists(&json, "data.name").passed);
        assert!(!engine.json_path_exists(&json, "data.nonexistent").passed);
        
        assert!(engine.json_is_object(&json).passed);
        assert!(engine.json_is_object(&json["data"]).passed);  // data.name is a string, but data is an object
    }

    #[test]
    fn test_response_time_assertion() {
        let engine = AssertionEngine::new();
        
        assert!(engine.response_time_less_than(100, 200).passed);
        assert!(!engine.response_time_less_than(300, 200).passed);
    }
}
