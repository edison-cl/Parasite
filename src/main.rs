use std::collections::HashMap;
use std::fs;
use std::io;
use std::slice::SliceIndex;
use std::thread::spawn;
use std::{thread, time};

use json::JsonValue;

#[path = "server/engine.rs"]
mod engine;

#[path = "utils.rs"]
mod utils;

#[path = "cluster.rs"]
mod cluster;

#[path = "config.rs"]
mod config;

// #[actix_rt::main]
// async fn main() -> io::Result<()>{
//     engine::server_run().await
// }

fn main() {
    // cluster::cluster_init();
    // cluster::cluster_add("123456789", "10.8.8.216:8000", "off", "follower");
    // cluster::cluster_del("123456789");
    // let data = cluster::node("f698d5e33138b3d538c9e0ae10f6f4c0")[""];
    // let  data = cluster::node_data().lock().unwrap();
    let h = spawn(||cluster::cluster_input_device());
    thread::sleep(time::Duration::from_secs(2));
    cluster::edit::role("f698d5e33138b3d538c9e0ae10f6f4c0", "candidate");
    thread::sleep(time::Duration::from_secs(5));
    cluster::cluster_add("4399", "10.8.8.237:4399", "on", "leader");
    h.join().unwrap();
    // println!("{:#?}", data);

}
