use super::Database;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub variables: HashMap<String, String>,
    pub is_global: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEnvironmentInput {
    pub name: String,
    pub variables: HashMap<String, String>,
    pub is_global: bool,
}

impl Database {
    pub fn create_environment(&self, input: CreateEnvironmentInput) -> Result<Environment> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let variables_json = serde_json::to_string(&input.variables).unwrap_or_default();
        
        // If this is global, unset other globals
        if input.is_global {
            self.conn.execute("UPDATE environments SET is_global = 0", [])?;
        }
        
        self.conn.execute(
            "INSERT INTO environments (id, name, variables, is_global, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![&id, &input.name, &variables_json, input.is_global as i32, now],
        )?;
        
        Ok(Environment {
            id,
            name: input.name,
            variables: input.variables,
            is_global: input.is_global,
            created_at: now,
            updated_at: now,
        })
    }
    
    pub fn get_environments(&self) -> Result<Vec<Environment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, variables, is_global, created_at, updated_at
             FROM environments
             ORDER BY name"
        )?;
        
        let envs = stmt.query_map([], |row| {
            let variables_json: String = row.get(2)?;
            let variables: HashMap<String, String> = 
                serde_json::from_str(&variables_json).unwrap_or_default();
            
            Ok(Environment {
                id: row.get(0)?,
                name: row.get(1)?,
                variables,
                is_global: row.get::<_, i32>(3)? == 1,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        
        envs.collect()
    }
    
    pub fn get_environment(&self, id: &str) -> Result<Option<Environment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, variables, is_global, created_at, updated_at
             FROM environments
             WHERE id = ?1"
        )?;
        
        let mut envs = stmt.query_map([id], |row| {
            let variables_json: String = row.get(2)?;
            let variables: HashMap<String, String> = 
                serde_json::from_str(&variables_json).unwrap_or_default();
            
            Ok(Environment {
                id: row.get(0)?,
                name: row.get(1)?,
                variables,
                is_global: row.get::<_, i32>(3)? == 1,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        
        Ok(envs.next().transpose()?)
    }
    
    pub fn update_environment(
        &self, 
        id: &str, 
        name: &str, 
        variables: &HashMap<String, String>,
        is_global: bool
    ) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let variables_json = serde_json::to_string(variables).unwrap_or_default();
        
        // If this is global, unset other globals
        if is_global {
            self.conn.execute("UPDATE environments SET is_global = 0 WHERE id != ?1", [id])?;
        }
        
        self.conn.execute(
            "UPDATE environments SET name = ?1, variables = ?2, is_global = ?3, updated_at = ?4 WHERE id = ?5",
            params![name, &variables_json, is_global as i32, now, id],
        )?;
        Ok(())
    }
    
    pub fn delete_environment(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM environments WHERE id = ?1", [id])?;
        Ok(())
    }
    
    pub fn get_active_environment(&self) -> Result<Option<Environment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, variables, is_global, created_at, updated_at
             FROM environments
             WHERE is_global = 1
             LIMIT 1"
        )?;
        
        let mut envs = stmt.query_map([], |row| {
            let variables_json: String = row.get(2)?;
            let variables: HashMap<String, String> = 
                serde_json::from_str(&variables_json).unwrap_or_default();
            
            Ok(Environment {
                id: row.get(0)?,
                name: row.get(1)?,
                variables,
                is_global: true,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        
        Ok(envs.next().transpose()?)
    }
}