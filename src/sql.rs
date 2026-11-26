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
        if arguments.is_empty() {
            return Ok(Vec::new());
        }

        let mut note_conditions = Vec::new();
        let mut tag_conditions = Vec::new();
        let mut folder_conditions = Vec::new();

        for arg in &arguments {
            note_conditions.push(format!("(title LIKE '%{}%' OR body LIKE '%{}%')", arg, arg));
            tag_conditions.push(format!("(title LIKE '%{}%')", arg));
            folder_conditions.push(format!("(title LIKE '%{}%')", arg));
        }

        let queries = vec![
            format!(
                "SELECT DISTINCT id FROM tags WHERE {}",
                tag_conditions.join(" AND ")
            ),
            format!(
                "SELECT DISTINCT id FROM folders WHERE {}",
                folder_conditions.join(" AND ")
            ),
            format!(
                "SELECT DISTINCT id FROM notes WHERE {}",
                note_conditions.join(" AND ")
            ),
        ];

        let mut result = Vec::new();
        for query in queries {
            let mut stmt = self.conn.prepare(&query)?;
            for row in stmt.query_map([], |row| row.get::<_, String>(0))? {
                result.push(row?);
            }
        }
        Ok(result)
    }

    pub fn get_metas(&self, ids: Vec<String>) -> Result<Vec<HashMap<String, String>>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let icon_str = "net.cozic.joplin_desktop".to_string();
        let placeholders = vec!["?"; ids.len()].join(", ");
        let mut result = Vec::new();

        let tag_query = format!(
            "SELECT tags.id, tags.title, GROUP_CONCAT(notes.title, ', ')
             FROM tags
             LEFT JOIN note_tags ON tags.id = note_tags.tag_id
             LEFT JOIN notes ON note_tags.note_id = notes.id
             WHERE tags.id IN ({})
             GROUP BY tags.id",
            placeholders
        );

        let note_query = format!(
            "SELECT id, title, SUBSTRING(body, 1, 100)
            FROM notes
            WHERE id IN ({})",
            placeholders
        );

        let folder_query = format!(
            "SELECT folders.id, folders.title, GROUP_CONCAT(notes.title, ', ')
             FROM folders
             LEFT JOIN notes ON notes.parent_id = folders.id
             WHERE folders.id IN ({})
             GROUP BY folders.id",
            placeholders
        );

        self.append_metadata_rows(&tag_query, &ids, &mut result, &icon_str)?;
        self.append_metadata_rows(&folder_query, &ids, &mut result, &icon_str)?;
        self.append_metadata_rows(&note_query, &ids, &mut result, &icon_str)?;

        Ok(result)
    }

    fn append_metadata_rows(
        &self,
        query: &str,
        ids: &[String],
        result: &mut Vec<HashMap<String, String>>,
        icon: &str,
    ) -> Result<()> {
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map(
            rusqlite::params_from_iter(ids.iter()),
            |row: &rusqlite::Row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                ))
            },
        )?;

        for row in rows {
            let (id, name, description) = row?;
            let mut metadata = HashMap::new();
            metadata.insert("id".to_string(), id);
            metadata.insert("name".to_string(), name);
            metadata.insert("description".to_string(), description.unwrap_or_default());
            metadata.insert("icon".to_string(), icon.to_string());
            result.push(metadata);
        }

        Ok(())
    }

    pub fn get_type(&self, id: &str) -> Result<Option<String>> {
        let tables = ["notes", "tags", "folders"];
        for table in tables.iter() {
            let query = format!("SELECT 1 FROM {} WHERE id = ?", table);
            let mut stmt = self.conn.prepare(&query)?;

            if stmt.exists([id])? {
                println!("Found ID in table: {}", table);
                return Ok(Some(table.to_string()));
            }
        }

        Ok(None)
    }
}
