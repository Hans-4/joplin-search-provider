mod sql;

use std::sync::{Arc};
use std::time::Duration;
use rusqlite::Result;
use zbus::{Connection, interface};
use sql::Sql;
use tokio::sync::Mutex;

struct MyInterface {
    sql: Arc<Mutex<Sql>>,
}

impl MyInterface {
    pub fn new(sql: Sql) -> Self {
        MyInterface {
            sql: Arc::new(Mutex::new(sql)),
        }
    }
}

#[interface(name = "org.gnome.Shell.SearchProvider2")]
impl MyInterface {
    async fn get_initial_result_set(&mut self, argument: Vec<String>) -> Vec<String> {
        let sql = self.sql.lock().await;
        sql.get_id(argument).unwrap_or_default()
    }
    async fn get_subsearch_result_set(&mut self, argument: Vec<String>) {

    }
    async fn get_result_metas(&mut self, argument: Vec<String>) {

    }
    async fn activate_result(&mut self, id: &str, terms: Vec<String>, timestamp: i64) {

    }
    async fn launch_search(&mut self, argument: Vec<String>, timestamp: i64) {

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = Sql::conn()?;
    let my_interface = MyInterface::new(sql);
    let connection = Connection::session().await?;
    connection.request_name("io.github.hans4.JoplinSearchProvider").await?;
    connection
        .object_server()
        .at("/io/github/hans4/JoplinSearchProvider", my_interface)
        .await?;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}