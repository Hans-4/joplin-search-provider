mod sql;

use zbus::{Connection, interface};

struct MyInterface {
}

#[interface(name = "org.gnome.Shell.SearchProvider2")]
impl MyInterface {
    fn get_initial_result_set(&mut self, argument: Vec<String>) {

    }
    fn get_subsearch_result_set(&mut self, argument: Vec<String>) {

    }
    fn get_result_metas(&mut self, argument: Vec<String>) {

    }
    fn activate_result(&mut self, id: &str, terms: Vec<String>, timestamp: i64) {

    }
    fn launch_search(&mut self, argument: Vec<String>, timestamp: i64) {

    }
}

fn main() {
}