use super::Database;
use crate::models::{HttpRequest, HttpResponse};
use rusqlite::{params, Result};
use serde_json;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub method: String,
    pub url: String,
    pub headers: String,
    pub body: Option<String>,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub time_ms: Option<i32>,
    pub created_at: i64,
}

impl Database {
    pub fn save_history(&self, request: &HttpRequest, response: Option<&HttpResponse>) -> Result<()> {
        let headers_json = serde_json::to_string(&request.headers).unwrap_or_default();
        let body = request.body.as_ref().map(|b| match b {
            crate::models::RequestBody::Json(s) => s.clone(),
            crate::models::RequestBody::Text(s) => s.clone(),
            crate::models::RequestBody::FormData(data) => serde_json::to_string(data).unwrap_or_default(),
            _ => String::new(),
        });
        
        let (status, resp_body, time_ms) = match response {
            Some(r) => (Some(r.status as i32), Some(r.body.clone()), Some(r.time_ms as i32)),
            None => (None, None, None),
        };
        
        let now = chrono::Utc::now().timestamp();
        
        self.conn.execute(
            "INSERT INTO history (id, method, url, headers, body, response_status, response_body, time_ms, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(id) DO UPDATE SET
             response_status = excluded.response_status,
             response_body = excluded.response_body,
             time_ms = excluded.time_ms",
            params![
                &request.id,
                request.method.to_string(),
                &request.url,
                headers_json,
                body,
                status,
                resp_body,
                time_ms,
                now
            ],
        )?;
        
        Ok(())
    }
    
    pub fn get_history(&self, limit: i64) -> Result<Vec<HistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, method, url, headers, body, response_status, response_body, time_ms, created_at
             FROM history
             ORDER BY created_at DESC
             LIMIT ?1"
        )?;
        
        let entries = stmt.query_map([limit], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                method: row.get(1)?,
                url: row.get(2)?,
                headers: row.get(3)?,
                body: row.get(4)?,
                response_status: row.get(5)?,
                response_body: row.get(6)?,
                time_ms: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?;
        
        entries.collect()
    }
    
    pub fn delete_old_history(&self, keep_count: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM history WHERE id NOT IN (
                SELECT id FROM history ORDER BY created_at DESC LIMIT ?1
            )",
            [keep_count],
        )?;
        Ok(())
    }
}