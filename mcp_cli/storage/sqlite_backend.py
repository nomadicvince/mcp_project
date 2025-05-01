import sqlite3
import json
from pathlib import Path

DB_PATH = Path(__file__).parent / "memory.db"

def init_db():
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("""
        CREATE TABLE IF NOT EXISTS memory (
            user_id TEXT PRIMARY KEY,
            data TEXT
        )
    """)
    conn.commit()
    conn.close()

def save_context(user_id, memory_dict):
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    json_data = json.dumps(memory_dict)
    c.execute("REPLACE INTO memory (user_id, data) VALUES (?, ?)", (user_id, json_data))
    conn.commit()
    conn.close()

def load_context(user_id):
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT data FROM memory WHERE user_id = ?", (user_id,))
    row = c.fetchone()
    conn.close()
    if row:
        return json.loads(row[0])
    return {}