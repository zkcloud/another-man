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
pub struct RequestDependency {
    pub id: String,
    pub request_id: String,
    pub depends_on_request_id: String,
    pub created_at: i64,
}

// Export format (Postman-like)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedCollection {
    pub info: ExportInfo,
    pub item: Vec<ExportItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportInfo {
    pub name: String,
    pub schema: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportItem {
    pub name: String,
    pub request: ExportRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub method: String,
    pub header: Vec<ExportHeader>,
    pub url: ExportUrl,
    pub body: Option<ExportBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportUrl {
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportBody {
    pub mode: String,
    pub raw: String,
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
    
    // Export/Import operations
    pub fn export_collection(&self, collection_id: &str) -> Result<ExportedCollection> {
        // Get collection info
        let collection: Collection = self.conn.query_row(
            "SELECT id, name, description, parent_id, sort_order, created_at, updated_at FROM collections WHERE id = ?1",
            [collection_id],
            |row| Ok(Collection {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            }),
        )?;
        
        // Get requests
        let requests = self.get_requests_by_collection(collection_id)?;
        
        // Build export format
        let items: Vec<ExportItem> = requests.into_iter().map(|req| {
            let headers: Vec<crate::models::Header> = 
                serde_json::from_str(&req.headers).unwrap_or_default();
            
            let export_headers: Vec<ExportHeader> = headers
                .into_iter()
                .filter(|h| h.enabled)
                .map(|h| ExportHeader { key: h.key, value: h.value })
                .collect();
            
            ExportItem {
                name: req.name,
                request: ExportRequest {
                    method: req.method,
                    header: export_headers,
                    url: ExportUrl { raw: req.url },
                    body: req.body.map(|b| ExportBody { mode: "raw".to_string(), raw: b }),
                },
            }
        }).collect();
        
        Ok(ExportedCollection {
            info: ExportInfo {
                name: collection.name,
                schema: "https://schema.getpostman.com/json/collection/v2.1.0/collection.json".to_string(),
            },
            item: items,
        })
    }
    
    pub fn import_collection(&self, imported: &ExportedCollection) -> Result<Collection> {
        let now = chrono::Utc::now().timestamp();
        let collection_id = uuid::Uuid::new_v4().to_string();
        
        // Create collection
        self.conn.execute(
            "INSERT INTO collections (id, name, description, parent_id, sort_order, created_at, updated_at)
             VALUES (?1, ?2, NULL, NULL, 0, ?3, ?3)",
            params![&collection_id, &imported.info.name, now],
        )?;
        
        // Import items
        for item in &imported.item {
            let request_id = uuid::Uuid::new_v4().to_string();
            let headers: Vec<crate::models::Header> = item.request.header
                .iter()
                .map(|h| crate::models::Header {
                    key: h.key.clone(),
                    value: h.value.clone(),
                    enabled: true,
                })
                .collect();
            let headers_json = serde_json::to_string(&headers).unwrap_or_default();
            
            self.conn.execute(
                "INSERT INTO requests (id, collection_id, name, method, url, headers, body, sort_order, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8, ?8)",
                params![
                    &request_id,
                    &collection_id,
                    &item.name,
                    &item.request.method,
                    &item.request.url.raw,
                    &headers_json,
                    item.request.body.as_ref().map(|b| b.raw.clone()),
                    now
                ],
            )?;
        }
        
        Ok(Collection {
            id: collection_id,
            name: imported.info.name.clone(),
            description: None,
            parent_id: None,
            sort_order: 0,
            created_at: now,
            updated_at: now,
        })
    }
    
    // Request dependency operations
    pub fn add_request_dependency(&self, request_id: &str, depends_on_request_id: &str) -> Result<RequestDependency> {
        let now = chrono::Utc::now().timestamp();
        let id = uuid::Uuid::new_v4().to_string();
        
        self.conn.execute(
            "INSERT INTO request_dependencies (id, request_id, depends_on_request_id, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![&id, request_id, depends_on_request_id, now],
        )?;
        
        Ok(RequestDependency {
            id,
            request_id: request_id.to_string(),
            depends_on_request_id: depends_on_request_id.to_string(),
            created_at: now,
        })
    }
    
    pub fn get_request_dependencies(&self, request_id: &str) -> Result<Vec<RequestDependency>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, request_id, depends_on_request_id, created_at
             FROM request_dependencies WHERE request_id = ?1 ORDER BY created_at"
        )?;
        
        let deps = stmt.query_map([request_id], |row| {
            Ok(RequestDependency {
                id: row.get(0)?,
                request_id: row.get(1)?,
                depends_on_request_id: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?.collect::<Result<Vec<_>>>()?;
        
        Ok(deps)
    }
    
    pub fn delete_request_dependency(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM request_dependencies WHERE id = ?1", [id])?;
        Ok(())
    }
    
    /// Get requests in dependency order using topological sort
    pub fn get_requests_in_order(&self, collection_id: &str) -> Result<Vec<SavedRequest>> {
        use std::collections::{HashMap, HashSet};
        
        let requests = self.get_requests_by_collection(collection_id)?;
        
        // Build dependency graph
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        
        for req in &requests {
            in_degree.insert(req.id.clone(), 0);
            graph.insert(req.id.clone(), Vec::new());
        }
        
        // Build edges from dependencies
        for req in &requests {
            let deps = self.get_request_dependencies(&req.id)?;
            for dep in deps {
                if let Some(edges) = graph.get_mut(&dep.depends_on_request_id) {
                    edges.push(req.id.clone());
                }
                *in_degree.get_mut(&req.id).unwrap() += 1;
            }
        }
        
        // Kahn's algorithm for topological sort
        let mut queue: Vec<String> = in_degree.iter()
            .filter(|(_, &count)| count == 0)
            .map(|(id, _)| id.clone())
            .collect();
        
        let mut result: Vec<SavedRequest> = Vec::new();
        let request_map: HashMap<String, SavedRequest> = requests.into_iter()
            .map(|r| (r.id.clone(), r))
            .collect();
        
        while let Some(id) = queue.pop() {
            if let Some(req) = request_map.get(&id) {
                result.push(req.clone());
            }
            
            if let Some(neighbors) = graph.get(&id) {
                for neighbor in neighbors {
                    let count = in_degree.get_mut(neighbor).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        queue.push(neighbor.clone());
                    }
                }
            }
        }
        
        Ok(result)
    }
}