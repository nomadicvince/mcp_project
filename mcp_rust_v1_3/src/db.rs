use std::collections::HashMap;
use rusqlite::{Connection, params};

pub fn init_db() {
    let conn = Connection::open("memory.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memory (
            user_id TEXT PRIMARY KEY,
            data TEXT
        )", [],
    ).unwrap();
}

pub fn save_memory(user_id: &str, memory: &HashMap<String, Vec<String>>) {
    let conn = Connection::open("memory.db").unwrap();
    let data = serde_json::to_string(memory).unwrap();
    conn.execute("REPLACE INTO memory (user_id, data) VALUES (?1, ?2)", params![user_id, data]).unwrap();
}

pub fn load_memory(user_id: &str) -> HashMap<String, Vec<String>> {
    let conn = Connection::open("memory.db").unwrap();
    let mut stmt = conn.prepare("SELECT data FROM memory WHERE user_id = ?1").unwrap();
    let result: Result<String, _> = stmt.query_row(params![user_id], |row| row.get(0));
    match result {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}