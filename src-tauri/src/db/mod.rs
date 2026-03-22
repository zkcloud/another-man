use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(app_dir: PathBuf) -> Result<Self> {
        let db_path = app_dir.join("another-man.db");
        let conn = Connection::open(db_path)?;
        
        let db = Self { conn };
        db.init_tables()?;
        
        Ok(db)
    }
    
    fn init_tables(&self) -> Result<()> {
        // History table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id TEXT PRIMARY KEY,
                method TEXT NOT NULL,
                url TEXT NOT NULL,
                headers TEXT,
                body TEXT,
                response_status INTEGER,
                response_body TEXT,
                time_ms INTEGER,
                created_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        // Collections table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS collections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                parent_id TEXT,
                sort_order INTEGER DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        // Requests table (saved requests)
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS requests (
                id TEXT PRIMARY KEY,
                collection_id TEXT,
                name TEXT NOT NULL,
                method TEXT NOT NULL,
                url TEXT NOT NULL,
                headers TEXT,
                body TEXT,
                sort_order INTEGER DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (collection_id) REFERENCES collections(id)
            )",
            [],
        )?;
        
        // Environments table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS environments (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                variables TEXT,
                is_global INTEGER DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        Ok(())
    }
    
    pub fn get_conn(&self) -> &Connection {
        &self.conn
    }
}

// History operations
pub mod history;
pub mod collections;
pub mod environments;
pub use history::*;
pub use collections::*;
pub use environments::*;