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
    pub id: String,
    pub address: String,
    pub role: Mutex<String>,
    pub wait_for_leader: i64,
    pub last_leader_beat: Mutex<i64>,
    pub term: Mutex<i64>,
    pub state: Mutex<String>,
    pub version: Mutex<i64>,
}

impl Node {
    pub fn new() -> Node {
        match utils::id_generator() {
            Some(id) => {
                let ip = utils::ip_get().unwrap();
                let port = utils::parse_args().port;
                Node {
                    id,
                    address: format!("{}:{}", ip, port),
                    role: Mutex::new(String::from("follower")),
                    wait_for_leader: rand::thread_rng().gen_range(300..700),
                    last_leader_beat: Mutex::new(Local::now().timestamp()),
                    term: Mutex::new(0),
                    version: Mutex::new(0),
                    state: Mutex::new(String::from("on")),
                }
            }
            None => panic!("cannot get uid"),
        }
    }
}

pub fn node_struct() -> &'static Node {
    static nodeJson: OnceCell<Node> = OnceCell::new();
    nodeJson.get_or_init(|| Node::new())
}

pub fn cluster_json() -> &'static Mutex<Value> {
    static clusterJson: OnceCell<Mutex<Value>> = OnceCell::new();
    clusterJson.get_or_init(|| {
        println!("clusterJson init");
        let f = fs::OpenOptions::new()
            .read(true)
            .open(&config::global_data::node_path())
            .unwrap();
        Mutex::new((serde_json::from_reader(f)).unwrap())
    })
}

pub fn listen_leader_beat() {
    loop {
        // println!("{}",cluster_json().lock().unwrap().to_string());
        let node = node_struct();
        let id = utils::id_generator().unwrap();
        let role = node.role.lock().unwrap().to_string();
        if role == String::from("follower") {
        } else if role == String::from("leader") {
            // break;
        } else if role == String::from("candidate") {
            // break;
        }
        let now = Local::now().timestamp();
        let last_beat = node.last_leader_beat.lock().unwrap().to_owned();
        let wait_for_leader = node.wait_for_leader;
        if now > last_beat + wait_for_leader / 1000 {
            // TIMEOUT
            *node.role.lock().unwrap() = "candidate".to_string();
            edit::role(id.as_str(), "candidate");
            println!("leader beat timeout");
        } else {
            println!("get leader beat");
        }
        drop(node);
        thread::sleep(time::Duration::from_millis(300));
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
            "role" : "follower"
        });
        let mut data = json!({});
        data.as_object_mut().unwrap().insert(id, node_local);
        f.write(format!("{}", data.to_string()).as_bytes()).unwrap();
        cluster_json();
    }
}

pub fn cluster_add(id: &str, address: &str, state: &str, role: &str) {
    let mut cluster_json = cluster_json().lock().unwrap();
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
    let mut cluster_json = cluster_json().lock().unwrap();
    cluster_json.as_object_mut().unwrap().remove(id);
}

pub fn cluster_input_device() {
    let mut last_md5 = String::from("");
    loop {
        let cluster_json = cluster_json().lock().unwrap();
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
        drop(cluster_json);
        thread::sleep(time::Duration::from_millis(500));
    }
}

pub mod edit {
    use super::*;
    pub fn state(id: &str, state: &str) {
        if state != "del" && state != "on" && state != "off" {
            panic!("invaid argusment{{state}}: {}", &state)
        }
        let mut cluster_json = cluster_json().lock().unwrap();
        cluster_json.as_object_mut().unwrap()[id]["state"] = Value::String(state.to_string());
    }
    pub fn role(id: &str, role: &str) {
        if role != "follower" && role != "leader" && role != "candidate" {
            panic!("invaid argusment{{role}}: {}", &role)
        }
        let mut cluster_json = cluster_json().lock().unwrap();
        cluster_json.as_object_mut().unwrap()[id]["role"] = Value::String(role.to_string());
        println!("listen time out, role change candidate. {} - {}",cluster_json.as_object_mut().unwrap()[id]["role"],role);
    }
}

pub fn sync_node_information_v1() {}
