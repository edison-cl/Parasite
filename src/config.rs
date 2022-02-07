use once_cell::sync::Lazy;
use std::{collections::HashMap, process::exit};
use super::*;

pub static GLOBAL_DATA: Lazy<HashMap<&str, String>> = Lazy::new(|| {
    let data_path = ".data".to_string();
    let node_path = data_path.clone() + "/.node";
    let mut m = HashMap::new();
    
    let port = parse_port();
    
    m.insert("data_path", data_path);
    m.insert("node_path", node_path);
    m.insert("port",port);
    m.insert("version","0.1.0".to_string());
    m
});

pub struct global_data {}
impl global_data {
    pub fn data_path() -> String {
        GLOBAL_DATA.get("data_path").unwrap().clone().to_string()
    }
    pub fn node_path() -> String {
        GLOBAL_DATA.get("node_path").unwrap().clone().to_string()
    }
    pub fn port() -> String {
        GLOBAL_DATA.get("port").unwrap().clone().to_string()
    }
    pub fn version() -> String {
        GLOBAL_DATA.get("version").unwrap().clone().to_string()
    }
}


fn parse_port() -> String{
    let mut port_index = 0;
    let arguments = std::env::args();
    let mut port_flag = false;
    for argument in arguments{
        if argument == "-p".to_string(){
            port_flag = true;
            break
        }
        port_index += 1;
    }
    if port_index > 1 && port_flag {
        let args:Vec<String> = std::env::args().collect();
        match args.get(port_index + 1){
            Some(port) => {
                let port_str = port.to_string();
                let port_int:u32 = port_str.parse().unwrap_or_else(|_|{
                    utils::ColorPrint::redln(format!("❌invaild argument: port => {}",port_str));
                    exit(0)
                });
                port_int.to_string()
            },
            None => {
                utils::ColorPrint::redln("❌invaild argument: port => None".to_string());
                exit(0)
            }
        }
    }else{
        "8000".to_string()
    }
}