use crate::utils::ColorPrint;

use super::*;
use async_std::task::sleep;
use chrono::Local;
use once_cell::sync::OnceCell;
use rand::Rng;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::sync::Mutex;
use std::time;
#[derive(Serialize)]
pub struct Node {
    // FOLLOWER CANDIDATE LEADER
    pub wait_for_leader: i64,
    pub last_leader_beat: Mutex<i64>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            wait_for_leader: rand::thread_rng().gen_range(300..700),
            last_leader_beat: Mutex::new(Local::now().timestamp()),
        }
    }
}

pub fn node_struct() -> &'static Node {
    static NODE_JSON: OnceCell<Node> = OnceCell::new();
    NODE_JSON.get_or_init(|| {
        println!("NODE_JSON init");
        Node::new()
    })
}

pub fn cluster_json() -> &'static Mutex<Value> {
    static CLUSTER_JSON: OnceCell<Mutex<Value>> = OnceCell::new();
    CLUSTER_JSON.get_or_init(|| {
        println!("CLUSTER_JSON init");
        let f = fs::OpenOptions::new()
            .read(true)
            .open(&config::global_data::node_path())
            .unwrap();
        Mutex::new((serde_json::from_reader(f)).unwrap())
    })
}
pub fn cluster_init() {
    if !fs::metadata(Path::new(&config::global_data::data_path())).is_ok() {
        fs::create_dir(&config::global_data::data_path()).unwrap();
        let mut f = fs::File::create(&config::global_data::node_path()).unwrap();
        let ip = utils::ip_get().unwrap();
        let id = utils::id_generator().unwrap();
        let node_local: Value = json!({
            "address" : format!("{}:{}",ip,config::global_data::port()),
            "state" : "on",
            "role" : "follower",
            "version":config::global_data::version(),
            "term":0,
            "nv":0,
        });
        let mut data = json!({});
        data.as_object_mut().unwrap().insert(id, node_local);
        f.write(format!("{}", data.to_string()).as_bytes()).unwrap();
    }
    println!("init--------");
    cluster_json();
    node_struct();
    println!("init--------");
}

pub async fn listen_leader_beat() {
    if cluster_json().lock().unwrap().as_object().unwrap().len() < 3 {
        return;
    }
    global_state::STATE
        .lock()
        .unwrap()
        .insert("listen_leader_beat", true);
    let mut timeout_count = 0;
    loop {
        let node = node_struct();
        let id = utils::id_generator().unwrap();
        let now = Local::now().timestamp();
        let last_beat = node.last_leader_beat.lock().unwrap().to_owned();
        let wait_for_leader = node.wait_for_leader;
        if now > last_beat + wait_for_leader / 1000 {
            // TIMEOUT
            timeout_count += 1;
            ColorPrint::redln(format!("leader beat timeout - {}", timeout_count));
            if timeout_count == 3 {
                edit::role(id.as_str(), "candidate");
                ColorPrint::redln(format!("leader beat timeout - over"));
                global_state::STATE
                    .lock()
                    .unwrap()
                    .insert("listen_leader_beat", false);
                break;
            }
        } else {
            timeout_count = 0
        }
        drop(node);
        sleep(time::Duration::from_millis(300)).await;
    }
}

pub fn cluster_add(address: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("http://{}/api/node/info", address);
    let resp = reqwest::blocking::get(url)?.text()?;
    let obj = json::parse(resp.as_str())?;
    let mut cluster_json = cluster_json().lock().unwrap();
    let node_new = json!({
        "address":address,
        "state":obj["state"].to_string(),
        "role" : obj["role"].to_string(),
        "term" : obj["term"].as_i64().unwrap(),
        "nv" : obj["nv"].as_i64().unwrap(),
        "version":obj["version"].to_string(),
    });

    cluster_json
        .as_object_mut()
        .unwrap()
        .insert(obj["id"].to_string(), node_new);
    // .unwrap();  加了报错
    Ok(())
}

pub async fn cluster_input_device() {
    let mut last_md5 = String::from("");
    loop {
        let mut this_md5 = String::new();
        let mut content = String::new();
        {
            let cluster_json = cluster_json().lock().unwrap();
            content = format!("{}", cluster_json.to_string());
            this_md5 = utils::md5(&content);
        }
        if this_md5 != last_md5 {
            last_md5 = this_md5;
            let mut f = fs::OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(&config::global_data::node_path())
                .unwrap();
            f.write_all(content.as_bytes()).unwrap();
            sync_node().await;
        }

        sleep(time::Duration::from_millis(500)).await;
        // sleep(time::Duration::from_secs(2)).await;
    }
}

pub mod edit {

    use std::process::exit;

    use serde_json;

    use crate::utils::ColorPrint;

    use super::*;
    pub fn state(id: &str, state: &str) {
        if state != "del" && state != "on" && state != "off" {
            ColorPrint::redln(format!("❌invaid argusment{{state}}: {}", &state));
            exit(0);
        }
        let mut cluster_json = cluster_json().lock().unwrap();
        cluster_json.as_object_mut().unwrap()[id]["state"] = Value::String(state.to_string());
        let nv = json!(
            cluster_json.as_object_mut().unwrap()[id]
                .get("nv")
                .unwrap()
                .as_i64()
                .unwrap()
                + 1
        );
        cluster_json.as_object_mut().unwrap()[id]["nv"] = serde_json::from_value(nv).unwrap();
    }
    pub fn role(id: &str, role: &str) {
        if role != "follower" && role != "leader" && role != "candidate" {
            ColorPrint::redln(format!("❌invaid argusment{{role}}: {}", &role));
            exit(0);
        }
        let mut cluster_json = cluster_json().lock().unwrap();
        cluster_json.as_object_mut().unwrap()[id]["role"] = Value::String(role.to_string());
        let nv = json!(
            cluster_json.as_object_mut().unwrap()[id]
                .get("nv")
                .unwrap()
                .as_i64()
                .unwrap()
                + 1
        );
        cluster_json.as_object_mut().unwrap()[id]["nv"] = serde_json::from_value(nv).unwrap();
    }
}

pub async fn sync_node() {
    let cluster = cluster_json().lock().unwrap();
    let local_id = utils::id_generator().unwrap();
    let local_node = cluster.as_object().unwrap().get(&local_id).unwrap();
    let cluster_length = cluster.as_object().unwrap().len();
    let local_role = local_node
        .as_object()
        .unwrap()
        .get("role")
        .unwrap()
        .to_string();
    drop(cluster);
    if local_role == "leader" {
        sync_node_v2().await;
    } else {
        if cluster_length < 3 {
            match sync_node_v1().await {
                Ok(_) => {}
                Err(err) => {
                    ColorPrint::redln(format!("add node failed, err: {}", err.to_string()));
                    exit(0);
                }
            }
        }
    }
}

pub async fn sync_node_v1() -> Result<(), Box<dyn Error>> {
    let cluster = cluster_json().lock().unwrap();
    let content = cluster.to_string();
    let local_cluster = for (id, node) in cluster.as_object().unwrap().iter() {
        if node.get("state").unwrap().to_string() != "on".to_string() {
            continue;
        }
        if id.to_owned() == utils::id_generator().unwrap() {
            continue;
        }
        let url = format!(
            "http://{}/api/node/sync/v1",
            node.get("address").unwrap().to_string()
        );
        let mut data = HashMap::new();
        data.insert("content", &content);
        let client = reqwest::Client::new();
        let result = client
            .post(url)
            .form(&data)
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?;
        if result.get("state").unwrap().to_string() != "ok".to_string() {
            let object = result
                .get("object")
                .unwrap()
                .as_object()
                .unwrap()
                .to_owned();
            let mut local_cluster = cluster.clone();
            for (id, node) in object.iter() {
                match local_cluster.as_object().unwrap().get(id) {
                    Some(_) => {}
                    None => {
                        local_cluster
                            .as_object_mut()
                            .unwrap()
                            .insert(id.to_owned(), node.to_owned());
                        continue;
                    }
                }
                // let remote_nv = node
                //     .as_object()
                //     .unwrap()
                //     .get("nv")
                //     .unwrap()
                //     .as_i64()
                //     .unwrap();
                // let local_nv = local_cluster
                //     .as_object()
                //     .unwrap()
                //     .get(id)
                //     .unwrap()
                //     .as_object()
                //     .unwrap()
                //     .get("nv")
                //     .unwrap()
                //     .as_i64()
                //     .unwrap();
                // if remote_nv > local_nv {
                //     local_cluster
                //         .as_object_mut()
                //         .unwrap()
                //         .insert(id.to_owned(), node.to_owned());
                // }
            }
        }
    };
    Ok(())
}

pub async fn sync_node_v2() {}
