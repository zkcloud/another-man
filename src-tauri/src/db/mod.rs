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
        
        // Initialize test suite tables
        let test_db = test_suites::TestDatabase::new(&self.conn);
        if let Err(e) = test_db.init_tables() {
            return Err(rusqlite::Error::InvalidParameterName(e.to_string()));
        }
        
        // Request dependencies table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS request_dependencies (
                id TEXT PRIMARY KEY,
                request_id TEXT NOT NULL,
                depends_on_request_id TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                FOREIGN KEY (request_id) REFERENCES requests(id) ON DELETE CASCADE,
                FOREIGN KEY (depends_on_request_id) REFERENCES requests(id) ON DELETE CASCADE,
                UNIQUE(request_id, depends_on_request_id)
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
pub mod test_suites;
pub use history::*;
pub use collections::*;
pub use environments::*;
pub use test_suites::*;