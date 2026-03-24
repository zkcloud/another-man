use clap::{Parser, Subcommand, Args};
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[command(name = "another-man")]
#[command(about = "HTTP Client CLI - Another Man")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a collection
    Run(RunArgs),
    /// Generate a report
    Report(ReportArgs),
    /// Import a collection
    Import(ImportArgs),
    /// Export a collection
    Export(ExportArgs),
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct RunArgs {
    /// Collection ID or name
    pub collection: String,
    /// Environment ID or name
    #[arg(short, long)]
    pub environment: Option<String>,
    /// Number of iterations
    #[arg(short, long, default_value = "1")]
    pub iterations: u32,
    /// Delay between requests in milliseconds
    #[arg(long, default_value = "0")]
    pub delay: u64,
    /// Stop on first error
    #[arg(short, long)]
    pub stop_on_error: bool,
    /// Output format (json, html)
    #[arg(short, long, default_value = "json")]
    pub output: String,
    /// Output file
    #[arg(short, long)]
    pub output_file: Option<String>,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct ReportArgs {
    /// Suite ID or name
    pub suite: String,
    /// Report format (json, html)
    #[arg(short, long, default_value = "html")]
    pub format: String,
    /// Output file
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct ImportArgs {
    /// File to import
    pub file: String,
    /// Collection name
    #[arg(short, long)]
    pub name: Option<String>,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct ExportArgs {
    /// Collection ID or name
    pub collection: String,
    /// Output file
    pub output: String,
}

/// CLI result for JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct CliResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub exit_code: i32,
}

impl CliResult {
    pub fn success(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: None,
            exit_code: 0,
        }
    }

    pub fn success_with_data(message: &str, data: serde_json::Value) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
            exit_code: 0,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
            exit_code: 1,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| r#"{"error": "JSON serialization failed"}"#.to_string())
    }

    pub fn print_json(&self) {
        println!("{}", self.to_json());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_result_success() {
        let result = CliResult::success("Operation completed");
        assert!(result.success);
        assert_eq!(result.exit_code, 0);
    }

    #[test]
    fn test_cli_result_error() {
        let result = CliResult::error("Something went wrong");
        assert!(!result.success);
        assert_eq!(result.exit_code, 1);
    }

    #[test]
    fn test_cli_result_with_data() {
        let data = serde_json::json!({"id": "123", "name": "test"});
        let result = CliResult::success_with_data("Done", data.clone());
        assert!(result.success);
        assert!(result.data.is_some());
    }
}
