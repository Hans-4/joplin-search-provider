use rusqlite::{Connection as SqliteConnection, Result};

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
}