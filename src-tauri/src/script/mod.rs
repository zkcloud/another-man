use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub mod assertions;
pub use assertions::{AssertionEngine, Assertion};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptExecutionResult {
    pub environment_changes: HashMap<String, String>,
    pub test_results: Vec<TestResult>,
    pub logs: Vec<String>,
}

pub struct ScriptEngine;

impl ScriptEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn execute_pre_request(
        &self,
        script: &str,
        env_vars: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, String> {
        let mut changes = HashMap::new();
        
        // Simple line-by-line parser for pm.environment.set()
        for line in script.lines() {
            let line = line.trim();
            if line.starts_with("pm.environment.set(") {
                if let Some(content) = line.strip_prefix("pm.environment.set(") {
                    if let Some(content) = content.strip_suffix(");") {
                        let parts: Vec<&str> = content.splitn(2, ',').collect();
                        if parts.len() == 2 {
                            let key = parts[0].trim().trim_matches('"').trim_matches('\'');
                            let value = parts[1].trim().trim_matches('"').trim_matches('\'');
                            changes.insert(key.to_string(), value.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(changes)
    }

    pub fn execute_test(
        &self,
        script: &str,
        response: crate::models::HttpResponse,
        _env_vars: HashMap<String, String>,
    ) -> Result<ScriptExecutionResult, String> {
        let mut results = ScriptExecutionResult {
            environment_changes: HashMap::new(),
            test_results: Vec::new(),
            logs: Vec::new(),
        };

        // Parse pm.test() calls
        for line in script.lines() {
            let line = line.trim();
            if line.starts_with("pm.test(") {
                if let Some(content) = line.strip_prefix("pm.test(") {
                    if let Some(content) = content.split("),").next() {
                        let test_name = content.trim().trim_matches('"').trim_matches('\'');
                        
                        // Simple test evaluation
                        let passed = if line.contains("pm.response.to.have.status") {
                            if let Some(status_str) = line.split("status(").nth(1) {
                                if let Some(status_str) = status_str.split(")").next() {
                                    if let Ok(expected) = status_str.parse::<u16>() {
                                        response.status == expected
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            true // Default pass for other tests
                        };

                        results.test_results.push(TestResult {
                            name: test_name.to_string(),
                            passed,
                            error: if passed { None } else { Some("Test failed".to_string()) },
                        });
                    }
                }
            }
        }

        Ok(results)
    }

    pub fn validate_script(&self, script: &str) -> Result<(), String> {
        // Basic validation - check for common syntax errors
        let open_parens = script.matches('(').count();
        let close_parens = script.matches(')').count();
        if open_parens != close_parens {
            return Err("Unbalanced parentheses".to_string());
        }
        
        let open_braces = script.matches('{').count();
        let close_braces = script.matches('}').count();
        if open_braces != close_braces {
            return Err("Unbalanced braces".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_request_script() {
        let engine = ScriptEngine::new();
        let script = r#"
            pm.environment.set("timestamp", "1234567890");
            pm.environment.set("random", "abc123");
        "#;
        
        let result = engine.execute_pre_request(script, HashMap::new());
        assert!(result.is_ok());
        
        let changes = result.unwrap();
        assert_eq!(changes.get("timestamp"), Some(&"1234567890".to_string()));
        assert_eq!(changes.get("random"), Some(&"abc123".to_string()));
    }

    #[test]
    fn test_validate_script() {
        let engine = ScriptEngine::new();
        
        // Valid script
        assert!(engine.validate_script("pm.environment.set('x', 'y');").is_ok());
        
        // Invalid script - unbalanced parentheses
        assert!(engine.validate_script("pm.environment.set('x', 'y';").is_err());
    }
}