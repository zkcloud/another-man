use rusqlite::{Connection, params, OptionalExtension};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub collection_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub id: String,
    pub suite_id: String,
    pub request_id: Option<String>,
    pub name: String,
    pub order_index: i32,
    pub pre_script: Option<String>,
    pub test_script: Option<String>,
    pub enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRun {
    pub id: String,
    pub suite_id: String,
    pub status: TestRunStatus,
    pub total_tests: i32,
    pub passed: i32,
    pub failed: i32,
    pub skipped: i32,
    pub duration_ms: i64,
    pub started_at: i64,
    pub completed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub id: String,
    pub run_id: String,
    pub test_case_id: String,
    pub status: TestStatus,
    pub assertion_results: String,
    pub error_message: Option<String>,
    pub duration_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestRunStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionResult {
    pub name: String,
    pub passed: bool,
    pub expected: String,
    pub actual: String,
    pub error: Option<String>,
}

pub struct TestDatabase<'a> {
    conn: &'a Connection,
}

impl<'a> TestDatabase<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn init_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS test_suites (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                collection_id TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS test_cases (
                id TEXT PRIMARY KEY,
                suite_id TEXT NOT NULL,
                request_id TEXT,
                name TEXT NOT NULL,
                order_index INTEGER NOT NULL DEFAULT 0,
                pre_script TEXT,
                test_script TEXT,
                enabled INTEGER NOT NULL DEFAULT 1,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (suite_id) REFERENCES test_suites(id) ON DELETE CASCADE
            );
            CREATE TABLE IF NOT EXISTS test_runs (
                id TEXT PRIMARY KEY,
                suite_id TEXT NOT NULL,
                status TEXT NOT NULL,
                total_tests INTEGER NOT NULL,
                passed INTEGER NOT NULL,
                failed INTEGER NOT NULL,
                skipped INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL,
                started_at INTEGER NOT NULL,
                completed_at INTEGER
            );
            CREATE TABLE IF NOT EXISTS test_results (
                id TEXT PRIMARY KEY,
                run_id TEXT NOT NULL,
                test_case_id TEXT NOT NULL,
                status TEXT NOT NULL,
                assertion_results TEXT NOT NULL,
                error_message TEXT,
                duration_ms INTEGER NOT NULL,
                FOREIGN KEY (run_id) REFERENCES test_runs(id) ON DELETE CASCADE
            );"
        )?;
        Ok(())
    }

    // Test Suite CRUD
    pub fn create_suite(&self, name: &str, description: Option<&str>, collection_id: Option<&str>) -> Result<TestSuite> {
        let now = chrono::Utc::now().timestamp();
        let id = uuid::Uuid::new_v4().to_string();
        
        self.conn.execute(
            "INSERT INTO test_suites (id, name, description, collection_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![&id, name, description, collection_id, now],
        )?;
        
        Ok(TestSuite {
            id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            collection_id: collection_id.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_suites(&self) -> Result<Vec<TestSuite>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, collection_id, created_at, updated_at
             FROM test_suites ORDER BY created_at DESC"
        )?;
        
        let suites = stmt.query_map([], |row| {
            Ok(TestSuite {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                collection_id: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(suites)
    }

    pub fn delete_suite(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM test_suites WHERE id = ?1", [id])?;
        Ok(())
    }

    // Test Case CRUD
    pub fn create_case(&self, suite_id: &str, name: &str, request_id: Option<&str>) -> Result<TestCase> {
        let now = chrono::Utc::now().timestamp();
        let id = uuid::Uuid::new_v4().to_string();
        
        let max_order: i32 = self.conn.query_row(
            "SELECT COALESCE(MAX(order_index), -1) FROM test_cases WHERE suite_id = ?1",
            [suite_id],
            |row| row.get(0),
        ).unwrap_or(-1);
        
        self.conn.execute(
            "INSERT INTO test_cases (id, suite_id, request_id, name, order_index, enabled, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, ?6)",
            params![&id, suite_id, request_id, name, max_order + 1, now],
        )?;
        
        Ok(TestCase {
            id,
            suite_id: suite_id.to_string(),
            request_id: request_id.map(|s| s.to_string()),
            name: name.to_string(),
            order_index: max_order + 1,
            pre_script: None,
            test_script: None,
            enabled: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_cases_by_suite(&self, suite_id: &str) -> Result<Vec<TestCase>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, suite_id, request_id, name, order_index, pre_script, test_script, enabled, created_at, updated_at
             FROM test_cases WHERE suite_id = ?1 ORDER BY order_index"
        )?;
        
        let cases = stmt.query_map([suite_id], |row| {
            Ok(TestCase {
                id: row.get(0)?,
                suite_id: row.get(1)?,
                request_id: row.get(2)?,
                name: row.get(3)?,
                order_index: row.get(4)?,
                pre_script: row.get(5)?,
                test_script: row.get(6)?,
                enabled: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(cases)
    }

    pub fn update_case_script(&self, id: &str, pre_script: Option<&str>, test_script: Option<&str>) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        self.conn.execute(
            "UPDATE test_cases SET pre_script = ?1, test_script = ?2, updated_at = ?3 WHERE id = ?4",
            params![pre_script, test_script, now, id],
        )?;
        Ok(())
    }

    pub fn delete_case(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM test_cases WHERE id = ?1", [id])?;
        Ok(())
    }

    // Test Runs
    pub fn create_run(&self, suite_id: &str, total_tests: i32) -> Result<TestRun> {
        let now = chrono::Utc::now().timestamp();
        let id = uuid::Uuid::new_v4().to_string();
        
        self.conn.execute(
            "INSERT INTO test_runs (id, suite_id, status, total_tests, passed, failed, skipped, duration_ms, started_at)
             VALUES (?1, ?2, 'running', ?3, 0, 0, 0, 0, ?4)",
            params![&id, suite_id, total_tests, now],
        )?;
        
        Ok(TestRun {
            id,
            suite_id: suite_id.to_string(),
            status: TestRunStatus::Running,
            total_tests,
            passed: 0,
            failed: 0,
            skipped: 0,
            duration_ms: 0,
            started_at: now,
            completed_at: None,
        })
    }

    pub fn complete_run(&self, id: &str, passed: i32, failed: i32, skipped: i32, duration_ms: i64) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let status = if failed > 0 { "failed" } else { "completed" };
        
        self.conn.execute(
            "UPDATE test_runs SET status = ?1, passed = ?2, failed = ?3, skipped = ?4, duration_ms = ?5, completed_at = ?6 WHERE id = ?7",
            params![status, passed, failed, skipped, duration_ms, now, id],
        )?;
        Ok(())
    }

    pub fn get_runs_by_suite(&self, suite_id: &str) -> Result<Vec<TestRun>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, suite_id, status, total_tests, passed, failed, skipped, duration_ms, started_at, completed_at
             FROM test_runs WHERE suite_id = ?1 ORDER BY started_at DESC LIMIT 50"
        )?;
        
        let runs = stmt.query_map([suite_id], |row| {
            let status_str: String = row.get(2)?;
            let status = match status_str.as_str() {
                "running" => TestRunStatus::Running,
                "completed" => TestRunStatus::Completed,
                "failed" => TestRunStatus::Failed,
                _ => TestRunStatus::Cancelled,
            };
            
            Ok(TestRun {
                id: row.get(0)?,
                suite_id: row.get(1)?,
                status,
                total_tests: row.get(3)?,
                passed: row.get(4)?,
                failed: row.get(5)?,
                skipped: row.get(6)?,
                duration_ms: row.get(7)?,
                started_at: row.get(8)?,
                completed_at: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(runs)
    }

    // Test Results
    pub fn save_result(&self, run_id: &str, test_case_id: &str, status: TestStatus, assertions: &[AssertionResult], duration_ms: i64) -> Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let status_str = match status {
            TestStatus::Passed => "passed",
            TestStatus::Failed => "failed",
            TestStatus::Skipped => "skipped",
        };
        let assertions_json = serde_json::to_string(assertions)?;
        let error = assertions.iter().find(|a| !a.passed).and_then(|a| a.error.clone());
        
        self.conn.execute(
            "INSERT INTO test_results (id, run_id, test_case_id, status, assertion_results, error_message, duration_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![&id, run_id, test_case_id, status_str, &assertions_json, error, duration_ms],
        )?;
        Ok(())
    }
}
