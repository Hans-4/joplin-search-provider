use rusqlite::{Connection as SqliteConnection, Result};
use std::collections::HashMap;

pub struct Sql {
    conn: SqliteConnection,
    _db_path: String,
}
impl Sql {
    pub fn conn() -> Result<Self> {
        let _db_path = std::env::var("JOPLIN_DB_PATH").unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            format!("{}/.config/joplin-desktop/database.sqlite", home)
        });
        let conn = SqliteConnection::open(&_db_path)?;
        Ok(Sql { conn, _db_path })
    }

    pub fn get_results(&self, arguments: Vec<String>) -> Result<Vec<String>> {
        let mut result = Vec::new();

        if arguments.is_empty() {
            return Ok(result);
        }

        let conditions: Vec<String> = arguments
            .iter()
            .map(|arg| format!("(title LIKE '%{}%' OR body LIKE '%{}%')", arg, arg))
            .collect();

        let query = format!(
            "SELECT DISTINCT id FROM notes WHERE {}",
            conditions.join(" AND ")
        );

        let mut statement = self.conn.prepare(&query)?;
        let rows = statement.query_map([], |row| row.get::<_, String>(0))?;

        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }

    pub fn get_metas(&self, ids: Vec<String>) -> Result<Vec<HashMap<String, String>>> {
        let mut result = Vec::new();
        let icon_str = "net.cozic.joplin_desktop".to_string();

        for id in ids {
            let mut statement = self
                .conn
                .prepare("SELECT title, SUBSTRING(body, 1, 100) FROM notes WHERE id = ?")?;

            let rows = statement.query_map([&id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            for row in rows {
                let (title, body_snippet) = row?;
                let mut dict = HashMap::new();
                dict.insert("id".to_string(), id.trim().to_string());
                dict.insert("name".to_string(), title);
                dict.insert("description".to_string(), body_snippet);
                dict.insert("icon".to_string(), icon_str.clone());

                result.push(dict);
            }
        }
        Ok(result)
    }
}
