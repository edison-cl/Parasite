use actix_web::web;
use chrono::Local;
use json::{self, object, JsonValue};
use once_cell::sync::{Lazy, OnceCell};
use rand::Rng;
use serde_json::{json, Value};
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::hash::Hash;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::JoinHandle;
use std::{thread, time};
#[path = "config.rs"]
mod config;
#[path = "utils.rs"]
mod utils;

pub struct Node {
    // FOLLOWER CANDIDATE LEADER
    pub id:String,
    pub address:String,
    pub role: Mutex<String>,
    pub wait_for_leader: i64,
    pub last_leader_beat: Mutex<i64>,
    pub term: Mutex<i64>,
    pub state:Mutex<String>,
    pub version:Mutex<i64>
}

impl Node {
    pub fn new() -> Node {
        match utils::id_generator() {
            Some(id) => {
                let ip = utils::ip_get().unwrap();
                let port = utils::parse_args().port;
                Node {
                    id,
                    address: format!("{}:{}",ip,port),
                    role: Mutex::new(String::from("follower")),
                    wait_for_leader: rand::thread_rng().gen_range(300..700),
                    last_leader_beat: Mutex::new(Local::now().timestamp()),
                    term: Mutex::new(0),
                    version:Mutex::new(0),
                    state:Mutex::new(String::from("on")),
                }
            },
            None => panic!("cannot get uid"),
        }
    }
}

pub fn node_data() -> &'static Mutex<Value> {
    static INSTANCE: OnceCell<Mutex<Value>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .open(&config::global_data::node_path())
            .unwrap();
        Mutex::new((serde_json::from_reader(f)).unwrap())
    })
}

pub fn listen_leader_beat(node: web::Data<Node>) {
    println!("begin listen, role {}", node.role.lock().unwrap());
    loop {
        if *node.role.lock().unwrap() == String::from("follower") {
        } else if *node.role.lock().unwrap() == String::from("leader") {
            break;
        } else if *node.role.lock().unwrap() == String::from("candidate") {
            break;
        }
        let now = Local::now().timestamp();
        let last_beat = node.last_leader_beat.lock().unwrap();
        let wait_for_leader = node.wait_for_leader;
        if now > *last_beat + wait_for_leader / 1000 {
            // TIMEOUT
            println!("timeout");
        }
        thread::sleep(time::Duration::from_millis(150));
    }
}

pub fn cluster_init() {
    if !fs::metadata(Path::new(&config::global_data::data_path())).is_ok() {
        fs::create_dir(&config::global_data::data_path()).unwrap();
        let mut f = fs::File::create(&config::global_data::node_path()).unwrap();
        let ip = utils::ip_get().unwrap();
        let id = utils::id_generator().unwrap();
        let config = utils::parse_args();
        let node_local: Value = json!({
            "address" : format!("{}:{}",ip,config.port),
            "state" : "on",
            "role" : "folower"
        });
        let mut data = json!({});
        data.as_object_mut().unwrap().insert(id, node_local);
        f.write(format!("{}", data.to_string()).as_bytes()).unwrap();
        node_data().lock().unwrap();
    }
}

pub fn cluster_add(id: &str, address: &str, state: &str, role: &str) {
    let mut cluster_json = node_data().lock().unwrap();
    let node_new = json!({
        "address" : address,
        "state" : state,
        "role" : role
    });

    cluster_json
        .as_object_mut()
        .unwrap()
        .insert(id.to_string(), node_new);
    // .unwrap();  加了报错
}

pub fn cluster_del(id: &str) {
    let mut cluster_json = node_data().lock().unwrap();
    cluster_json.as_object_mut().unwrap().remove(id);
}

pub fn cluster_input_device() {
    let mut last_md5 = String::from("");
    loop {
        let cluster_json = node_data().lock().unwrap();
        let content = format!("{}", cluster_json.to_string());
        let this_md5 = utils::md5(&content);
        if this_md5 != last_md5 {
            last_md5 = this_md5;
            let mut f = fs::OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(&config::global_data::node_path())
                .unwrap();
            f.write_all(content.as_bytes()).unwrap();
        }
        thread::sleep(time::Duration::from_millis(500));
    }
}

pub mod edit {
    use super::*;
    pub fn state(id: &str, state: &str) {
        if state != "del" && state != "on" && state != "off" {
            panic!("invaid argusment{{state}}: {}", &state)
        }
        let mut cluster_json = node_data().lock().unwrap();
        cluster_json.as_object_mut().unwrap()[id]["state"] = Value::String(state.to_string());
    }
    pub fn role(id: &str, role: &str) {
        if role != "follower" && role != "leader" && role != "candidate" {
            panic!("invaid argusment{{role}}: {}", &role)
        }
        let mut cluster_json = node_data().lock().unwrap();
        cluster_json.as_object_mut().unwrap()[id]["role"] = Value::String(role.to_string());
    }
}

pub fn sync_node_information_v1(){

}
#[cfg(test)]
mod tests {
    use super::*;
    pub fn run_test() {
        let node_web_data = web::Data::new(Node::new());
        {
            let node = node_web_data.clone();
            std::thread::spawn(|| listen_leader_beat(node));
        }
        thread::sleep(time::Duration::from_secs(2));
        {
            let node = node_web_data.clone();
            let mut role = node.role.lock().unwrap();
            *role = String::from("LEADER");
        }
        thread::sleep(time::Duration::from_secs(2));
        {
            let node = node_web_data.clone();
            let mut role = node.role.lock().unwrap();
            *role = String::from("FOLLOWER");
        }
        {
            let node = node_web_data.clone();
            println!("start twice listen");
            std::thread::spawn(|| listen_leader_beat(node));
        }
        thread::sleep(time::Duration::from_secs(2));
        {
            let node = node_web_data.clone();
            let mut role = node.role.lock().unwrap();
            *role = String::from("LEADER");
        }
        thread::sleep(time::Duration::from_secs(5));
        // h.join().unwrap();
        // h2.join().unwrap();
    }
}
