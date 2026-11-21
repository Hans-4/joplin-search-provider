use rusqlite::{Connection as SqliteConnection, Result};

struct Sql {
    conn: SqliteConnection,
    db_path: String,
}
impl Sql {
    fn conn(&mut self) -> Result<()> {
        self.db_path = "/home/hannes/.config/joplin-desktop/database.sqlite".to_string();
        self.conn = SqliteConnection::open(&self.db_path)?;
        Ok(())
    }
}