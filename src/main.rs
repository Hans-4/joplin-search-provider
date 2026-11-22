mod sql;

use std::collections::HashMap;
use std::sync::{Arc};
use std::time::Duration;
use rusqlite::Result;
use zbus::{Connection, interface};
use sql::Sql;
use tokio::sync::Mutex;
use zbus::zvariant::Value;

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
        println!("Calling get_initial_result_with: {argument:?}");
        let sql = self.sql.lock().await;
        sql.get_id(argument).unwrap_or_default()
    }
    async fn get_subsearch_result_set(&mut self, previous_results: Vec<String>, arguments: Vec<String>) -> Vec<String> {
        println!("Calling get_subsearch_result_set: {arguments:?}");
        let sql = self.sql.lock().await;
        let result = sql.get_results(arguments).unwrap_or_default();
        println!("Recived: {result:?}",);
        result
    }

    async fn get_result_metas(&mut self, argument: Vec<String>) -> Vec<HashMap<String, Value<'static>>> {
        println!("Calling get_result_metas: {argument:?}");
        let sql = self.sql.lock().await;
        let result = sql.get_metas(argument).unwrap_or_default();
        println!("Recived: {result:?}",);
        result.into_iter().map(|dict| {
            dict.into_iter().map(|(key, val)| {
                (key, Value::from(val))
            }).collect()
        }).collect()
    }
    async fn activate_result(&mut self, id: &str, argument: Vec<String>, timestamp: i64) {
        println!("Calling activate_result: {argument:?}");
    }
    async fn launch_search(&mut self, argument: Vec<String>, timestamp: i64) {
        println!("Calling launch_search: {argument:?}");
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