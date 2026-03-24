use serde_json::Value;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Extractor {
    JsonPath { path: String },
    Regex { pattern: String, group: usize },
    Header { name: String },
}

#[derive(Debug, Clone)]
pub struct VariableExtraction {
    pub name: String,
    pub extractor: Extractor,
}

pub struct VariableExtractor;

impl VariableExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract value from JSON using dot notation path
    /// Examples: "data.user.name", "items.0.id", "meta.total"
    pub fn extract_json_path(&self, json: &Value, path: &str) -> Option<String> {
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

        Some(self.value_to_string(current))
    }

    /// Extract value using regex pattern
    /// pattern: regex pattern with capture groups
    /// group: which capture group to return (0 = full match)
    pub fn extract_regex(&self, text: &str, pattern: &str, group: usize) -> Option<String> {
        let regex = Regex::new(pattern).ok()?;
        if let Some(captures) = regex.captures(text) {
            return captures.get(group).map(|m| m.as_str().to_string());
        }
        None
    }

    /// Extract value from HTTP headers
    pub fn extract_header(&self, headers: &HashMap<String, String>, name: &str) -> Option<String> {
        headers.get(&name.to_lowercase()).cloned()
    }

    /// Extract multiple variables from response
    pub fn extract_variables(
        &self,
        body: &str,
        headers: &HashMap<String, String>,
        extractions: &[VariableExtraction],
    ) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        let json_body: Option<Value> = serde_json::from_str(body).ok();

        for extraction in extractions {
            let value = match &extraction.extractor {
                Extractor::JsonPath { path } => {
                    json_body.as_ref().and_then(|json| self.extract_json_path(json, path))
                }
                Extractor::Regex { pattern, group } => {
                    self.extract_regex(body, pattern, *group)
                }
                Extractor::Header { name } => {
                    self.extract_header(headers, name)
                }
            };

            if let Some(val) = value {
                variables.insert(extraction.name.clone(), val);
            }
        }

        variables
    }

    /// Parse extraction definition from string format
    /// Format: "variableName:jsonPath:data.user.id" or "variableName:regex:pattern:group"
    pub fn parse_extraction(&self, definition: &str) -> Option<VariableExtraction> {
        let parts: Vec<&str> = definition.split(':').collect();
        
        if parts.len() < 3 {
            return None;
        }

        let name = parts[0].to_string();
        let extractor_type = parts[1];
        let config = parts[2];

        let extractor = match extractor_type {
            "jsonPath" => Extractor::JsonPath {
                path: config.to_string(),
            },
            "regex" => {
                let group = parts.get(3).and_then(|g| g.parse().ok()).unwrap_or(0);
                Extractor::Regex {
                    pattern: config.to_string(),
                    group,
                }
            }
            "header" => Extractor::Header {
                name: config.to_string(),
            },
            _ => return None,
        };

        Some(VariableExtraction { name, extractor })
    }

    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_path() {
        let extractor = VariableExtractor::new();
        let json: Value = serde_json::from_str(r#"{
            "data": {
                "user": {
                    "name": "John",
                    "id": 123
                },
                "items": [{"id": "a"}, {"id": "b"}]
            }
        }"#).unwrap();

        assert_eq!(
            extractor.extract_json_path(&json, "data.user.name"),
            Some("John".to_string())
        );
        assert_eq!(
            extractor.extract_json_path(&json, "data.user.id"),
            Some("123".to_string())
        );
        assert_eq!(
            extractor.extract_json_path(&json, "data.items.0.id"),
            Some("a".to_string())
        );
        assert_eq!(
            extractor.extract_json_path(&json, "data.items.1.id"),
            Some("b".to_string())
        );
    }

    #[test]
    fn test_extract_regex() {
        let extractor = VariableExtractor::new();
        let text = r#"{"token": "abc123", "user": "john"}"#;

        let result = extractor.extract_regex(text, r#""token":\s*"([^"]+)""#, 1).unwrap();
        assert_eq!(result, "abc123");

        let result = extractor.extract_regex(text, r#""user":\s*"([^"]+)""#, 1).unwrap();
        assert_eq!(result, "john");
    }

    #[test]
    fn test_extract_header() {
        let extractor = VariableExtractor::new();
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("x-request-id".to_string(), "abc-123".to_string());

        assert_eq!(
            extractor.extract_header(&headers, "Content-Type"),
            Some("application/json".to_string())
        );
        assert_eq!(
            extractor.extract_header(&headers, "x-request-id"),
            Some("abc-123".to_string())
        );
    }

    #[test]
    fn test_parse_extraction() {
        let extractor = VariableExtractor::new();

        let ext = extractor.parse_extraction("userId:jsonPath:data.user.id").unwrap();
        assert_eq!(ext.name, "userId");
        match ext.extractor {
            Extractor::JsonPath { path } => assert_eq!(path, "data.user.id"),
            _ => panic!("Expected JsonPath extractor"),
        }

        let ext = extractor.parse_extraction("token:regex:testpattern:1").unwrap();
        assert_eq!(ext.name, "token");
        match ext.extractor {
            Extractor::Regex { pattern, group } => {
                assert_eq!(pattern, "testpattern");
                assert_eq!(group, 1);
            }
            _ => panic!("Expected Regex extractor"),
        }

        let ext = extractor.parse_extraction("auth:header:Authorization").unwrap();
        assert_eq!(ext.name, "auth");
        match ext.extractor {
            Extractor::Header { name } => assert_eq!(name, "Authorization"),
            _ => panic!("Expected Header extractor"),
        }
    }

    #[test]
    fn test_extract_variables() {
        let extractor = VariableExtractor::new();
        let body = r#"{"user": {"id": 123, "name": "John"}}"#;
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        
        let extractions = vec![
            VariableExtraction {
                name: "userId".to_string(),
                extractor: Extractor::JsonPath { path: "user.id".to_string() },
            },
            VariableExtraction {
                name: "userName".to_string(),
                extractor: Extractor::JsonPath { path: "user.name".to_string() },
            },
        ];
        
        let vars = extractor.extract_variables(body, &headers, &extractions);
        
        assert_eq!(vars.get("userId"), Some(&"123".to_string()));
        assert_eq!(vars.get("userName"), Some(&"John".to_string()));
    }
}
