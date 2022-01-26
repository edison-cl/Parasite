use once_cell::sync::Lazy;

use std::{collections::HashMap, sync::Mutex};

pub static GLOBAL_DATA: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let data_path = ".data".to_string();
    let node_path = data_path.clone() + "/.node";
    let mut m = HashMap::new();
    m.insert("data_path".to_string(), data_path);
    m.insert("node_path".to_string(), node_path);
    Mutex::new(m)
});

// pub fn _global_data() -> &'static Mutex<HashMap<String, String>> {
//     static INSTANCE: OnceCell<Mutex<HashMap<String, String>>> = OnceCell::new();
//     let data_path = ".data".to_string();
//     let node_path = data_path.clone() + "/.node";
//     INSTANCE.get_or_init(|| {
//         let mut m = HashMap::new();
//         m.insert("data_path".to_string(), data_path);
//         m.insert("node_path".to_string(), node_path);
//         Mutex::new(m)
//     })
// }

pub struct global_data {}
impl global_data {
    pub fn data_path() -> String {
        let data = GLOBAL_DATA.lock().unwrap();
        data.get("data_path").unwrap().clone()
    }

    pub fn node_path() -> String {
        let data = GLOBAL_DATA.lock().unwrap();
        data.get("node_path").unwrap().clone()
    }
}
