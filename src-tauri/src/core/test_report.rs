use serde::{Serialize, Deserialize};
use crate::db::test_suites::{TestRun, TestRunStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    pub summary: TestSummary,
    pub suites: Vec<SuiteReport>,
    pub generated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    pub total_suites: i32,
    pub total_tests: i32,
    pub passed: i32,
    pub failed: i32,
    pub skipped: i32,
    pub duration_ms: i64,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteReport {
    pub suite_id: String,
    pub suite_name: String,
    pub tests: Vec<TestResultReport>,
    pub summary: TestSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResultReport {
    pub test_id: String,
    pub test_name: String,
    pub status: String,
    pub duration_ms: i64,
    pub error_message: Option<String>,
}

pub struct TestReportGenerator;

impl TestReportGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_html(&self, report: &TestReport) -> Result<String, String> {
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Another Man 测试报告</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #f5f5f5; padding: 20px; }}
        .container {{ max-width: 1200px; margin: 0 auto; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }}
        .header {{ background: #1a1a2e; color: white; padding: 24px; border-radius: 8px 8px 0 0; }}
        .header h1 {{ font-size: 24px; margin-bottom: 8px; }}
        .header .meta {{ color: #888; font-size: 14px; }}
        .summary {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 16px; padding: 24px; background: #f8f9fa; border-bottom: 1px solid #e0e0e0; }}
        .summary-card {{ background: white; padding: 16px; border-radius: 8px; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }}
        .summary-card .number {{ font-size: 32px; font-weight: bold; margin-bottom: 4px; }}
        .summary-card .label {{ font-size: 12px; color: #666; }}
        .summary-card.passed .number {{ color: #28a745; }}
        .summary-card.failed .number {{ color: #dc3545; }}
        .summary-card.skipped .number {{ color: #6c757d; }}
        .summary-card.rate .number {{ color: #2196f3; }}
        .suite {{ border-bottom: 1px solid #e0e0e0; padding: 16px 24px; }}
        .suite:last-child {{ border-bottom: none; }}
        .suite-header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }}
        .suite-header h2 {{ font-size: 16px; font-weight: 600; }}
        .test-item {{ display: flex; align-items: center; gap: 16px; padding: 12px; background: #f8f9fa; border-radius: 4px; margin-bottom: 8px; }}
        .test-item:last-child {{ margin-bottom: 0; }}
        .test-status {{ width: 80px; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-weight: 500; text-align: center; }}
        .test-status.passed {{ background: #d4edda; color: #155724; }}
        .test-status.failed {{ background: #f8d7da; color: #721c24; }}
        .test-name {{ flex: 1; font-weight: 500; }}
        .test-duration {{ color: #666; font-size: 13px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Another Man 测试报告</h1>
            <div class="meta">生成时间: {}</div>
        </div>
        <div class="summary">
            <div class="summary-card passed">
                <div class="number">{}</div>
                <div class="label">通过</div>
            </div>
            <div class="summary-card failed">
                <div class="number">{}</div>
                <div class="label">失败</div>
            </div>
            <div class="summary-card skipped">
                <div class="number">{}</div>
                <div class="label">跳过</div>
            </div>
            <div class="summary-card rate">
                <div class="number">{:.1}%</div>
                <div class="label">成功率</div>
            </div>
        </div>
        {}
    </div>
</body>
</html>"#,
            self.format_timestamp(report.generated_at),
            report.summary.passed,
            report.summary.failed,
            report.summary.skipped,
            report.summary.success_rate,
            self.generate_suites_html(&report.suites)
        );
        Ok(html)
    }

    pub fn generate_json(&self, report: &TestReport) -> Result<String, String> {
        serde_json::to_string_pretty(report).map_err(|e| e.to_string())
    }

    fn format_timestamp(&self, timestamp: i64) -> String {
        use chrono::DateTime;
        let dt = DateTime::from_timestamp(timestamp, 0)
            .unwrap_or_else(|| DateTime::UNIX_EPOCH);
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn get_status_text<'a>(&self, status: &'a str) -> &'a str {
        match status {
            "passed" => "通过",
            "failed" => "失败",
            _ => status,
        }
    }

    fn generate_suites_html(&self, suites: &[SuiteReport]) -> String {
        suites.iter().map(|suite| {
            format!(
                r#"<div class="suite">
                    <div class="suite-header">
                        <h2>{}</h2>
                    </div>
                    {}
                </div>"#,
                suite.suite_name,
                suite.tests.iter().map(|t| {
                    let status_text = self.get_status_text(&t.status);
                    format!(
                        r#"<div class="test-item">
                            <span class="test-status {}">{}</span>
                            <span class="test-name">{}</span>
                            <span class="test-duration">{}ms</span>
                        </div>"#,
                        t.status,
                        status_text,
                        t.test_name,
                        t.duration_ms
                    )
                }).collect::<Vec<_>>().join("\n")
            )
        }).collect::<Vec<_>>().join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_json_report() {
        let report = TestReport {
            summary: TestSummary {
                total_suites: 1,
                total_tests: 2,
                passed: 1,
                failed: 1,
                skipped: 0,
                duration_ms: 1000,
                success_rate: 50.0,
            },
            suites: vec![
                SuiteReport {
                    suite_id: "s1".to_string(),
                    suite_name: "Test Suite 1".to_string(),
                    tests: vec![
                        TestResultReport {
                            test_id: "t1".to_string(),
                            test_name: "Test 1".to_string(),
                            status: "passed".to_string(),
                            duration_ms: 500,
                            error_message: None,
                        },
                        TestResultReport {
                            test_id: "t2".to_string(),
                            test_name: "Test 2".to_string(),
                            status: "failed".to_string(),
                            duration_ms: 500,
                            error_message: Some("Error".to_string()),
                        },
                    ],
                    summary: TestSummary {
                        total_suites: 1,
                        total_tests: 2,
                        passed: 1,
                        failed: 1,
                        skipped: 0,
                        duration_ms: 1000,
                        success_rate: 50.0,
                    },
                }
            ],
            generated_at: 1704067200,
        };

        let generator = TestReportGenerator::new();
        
        // Test JSON generation
        let json = generator.generate_json(&report).unwrap();
        assert!(json.contains("\"passed\":"));
        assert!(json.contains("\"failed\":"));
        
        // Test HTML generation
        let html = generator.generate_html(&report).unwrap();
        assert!(html.contains("Another Man 测试报告"));
        assert!(html.contains("Test Suite 1"));
    }
}
