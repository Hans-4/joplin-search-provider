use std::collections::HashMap;
use rusqlite::{Connection as SqliteConnection, Result};
use gio::prelude::*;
use zbus::Str;

pub struct Sql {
    conn: SqliteConnection,
    _db_path: String,
}
impl Sql {
    pub fn conn() -> Result<Self> {
        let _db_path = "/home/hannes/.config/joplin-desktop/database.sqlite".to_string();
        let conn = SqliteConnection::open(&_db_path)?;
        Ok(Sql { conn, _db_path })
    }
    pub fn get_id(&self, arguments: Vec<String>) -> Result<Vec<String>> {
        let mut result = Vec::new();
        for argument in arguments {
            let mut statement = self.conn.prepare(
                "SELECT id FROM notes WHERE title LIKE ? OR body LIKE ?",
            )?;
            let search_pattern = format!("%{}%", argument);
            let rows = statement.query_map(
                [search_pattern.clone(), search_pattern],
                |row| row.get::<_, String>(0)
            )?;
            for row in rows {
                result.push(row?);
            }
        }
        Ok(result)
    }
    pub fn get_metas(&self, ids: Vec<String>) -> Result<Vec<HashMap<String, String>>> {
        let mut result = Vec::new();
        let icon_str = "net.cozic.joplin_desktop".to_string();
        for id in ids {
            let mut statement = self.conn.prepare(
                "SELECT title FROM notes WHERE id = ?",
            )?;
            let rows = statement.query_map([&id], |row| row.get::<_, String>(0))?;
            for row in rows {
                let mut dict = HashMap::new();
                dict.insert("id".to_string(), id.trim().to_string());
                dict.insert("title".to_string(), row?);
                dict.insert("icon".to_string(), icon_str.clone());
                result.push(dict);
            }
        }
        Ok(result)
    }
}