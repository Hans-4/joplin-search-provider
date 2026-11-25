mod sql;

use rusqlite::Result;
use sql::Sql;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use zbus::Variant;
use zbus::{Connection, interface};

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
        sql.get_results(argument).unwrap_or_default()
    }

    async fn get_subsearch_result_set(
        &mut self,
        _previous_results: Vec<String>,
        arguments: Vec<String>,
    ) -> Vec<String> {
        let sql = self.sql.lock().await;
        sql.get_results(arguments).unwrap_or_default()
    }

    async fn get_result_metas(
        &mut self,
        argument: Vec<String>,
    ) -> Vec<HashMap<String, Variant<'_>>> {
        let sql = self.sql.lock().await;
        let result = sql.get_metas(argument).unwrap_or_default();

        result
            .into_iter()
            .map(|dict| {
                dict.into_iter()
                    .map(|(key, val)| (key, Variant::from(val)))
                    .collect()
            })
            .collect()
    }

    async fn activate_result(&mut self, id: &str, _terms: Vec<String>, _timestamp: u32) {
        std::process::Command::new("xdg-open")
            .arg(format!("joplin://x-callback-url/openNote?id={}", id))
            .spawn()
            .ok();
    }

    async fn launch_search(&mut self, _terms: Vec<String>, _timestamp: u32) {
        std::process::Command::new("xdg-open")
            .arg("joplin://")
            .spawn()
            .ok();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = Sql::conn()?;
    let my_interface = MyInterface::new(sql);
    let connection = Connection::session().await?;
    connection
        .request_name("io.github.hans4.JoplinSearchProvider")
        .await?;
    connection
        .object_server()
        .at("/io/github/hans4/JoplinSearchProvider", my_interface)
        .await?;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
