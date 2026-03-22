use super::Database;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub sort_order: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedRequest {
    pub id: String,
    pub collection_id: Option<String>,
    pub name: String,
    pub method: String,
    pub url: String,
    pub headers: String,
    pub body: Option<String>,
    pub sort_order: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCollectionInput {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestInput {
    pub collection_id: Option<String>,
    pub name: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<crate::models::Header>,
    pub body: Option<String>,
}

impl Database {
    // Collection operations
    pub fn create_collection(&self, input: CreateCollectionInput) -> Result<Collection> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        
        self.conn.execute(
            "INSERT INTO collections (id, name, description, parent_id, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, 0, ?5, ?5)",
            params![&id, &input.name, &input.description, &input.parent_id, now],
        )?;
        
        Ok(Collection {
            id,
            name: input.name,
            description: input.description,
            parent_id: input.parent_id,
            sort_order: 0,
            created_at: now,
            updated_at: now,
        })
    }
    
    pub fn get_collections(&self) -> Result<Vec<Collection>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, parent_id, sort_order, created_at, updated_at
             FROM collections
             ORDER BY sort_order, name"
        )?;
        
        let collections = stmt.query_map([], |row| {
            Ok(Collection {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        
        collections.collect()
    }
    
    pub fn update_collection(&self, id: &str, name: &str, description: Option<&str>) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        self.conn.execute(
            "UPDATE collections SET name = ?1, description = ?2, updated_at = ?3 WHERE id = ?4",
            params![name, description, now, id],
        )?;
        Ok(())
    }
    
    pub fn delete_collection(&self, id: &str) -> Result<()> {
        // Delete all requests in this collection first
        self.conn.execute("DELETE FROM requests WHERE collection_id = ?1", [id])?;
        // Delete the collection
        self.conn.execute("DELETE FROM collections WHERE id = ?1", [id])?;
        Ok(())
    }
    
    // Saved Request operations
    pub fn create_request(&self, input: CreateRequestInput) -> Result<SavedRequest> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let headers_json = serde_json::to_string(&input.headers).unwrap_or_default();
        
        self.conn.execute(
            "INSERT INTO requests (id, collection_id, name, method, url, headers, body, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8, ?8)",
            params![&id, &input.collection_id, &input.name, &input.method, &input.url, &headers_json, &input.body, now],
        )?;
        
        Ok(SavedRequest {
            id,
            collection_id: input.collection_id,
            name: input.name,
            method: input.method,
            url: input.url,
            headers: headers_json,
            body: input.body,
            sort_order: 0,
            created_at: now,
            updated_at: now,
        })
    }
    
    pub fn get_requests_by_collection(&self, collection_id: &str) -> Result<Vec<SavedRequest>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, collection_id, name, method, url, headers, body, sort_order, created_at, updated_at
             FROM requests
             WHERE collection_id = ?1
             ORDER BY sort_order, name"
        )?;
        
        let requests = stmt.query_map([collection_id], |row| {
            Ok(SavedRequest {
                id: row.get(0)?,
                collection_id: row.get(1)?,
                name: row.get(2)?,
                method: row.get(3)?,
                url: row.get(4)?,
                headers: row.get(5)?,
                body: row.get(6)?,
                sort_order: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        
        requests.collect()
    }
    
    pub fn update_request(&self, id: &str, name: &str, url: &str, method: &str, headers: &str, body: Option<&str>) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        self.conn.execute(
            "UPDATE requests SET name = ?1, url = ?2, method = ?3, headers = ?4, body = ?5, updated_at = ?6 WHERE id = ?7",
            params![name, url, method, headers, body, now, id],
        )?;
        Ok(())
    }
    
    pub fn delete_request(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM requests WHERE id = ?1", [id])?;
        Ok(())
    }
}