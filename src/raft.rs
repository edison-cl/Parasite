use actix_web::web;
use chrono::Local;
use rand::Rng;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{thread, time};

pub struct Node {
    // FOLLOWER CANDIDATE LEADER
    pub ROLE: Mutex<String>,
    pub WAIT_FOR_LEADER: i64,
    pub LAST_BEAT: Mutex<i64>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            ROLE: Mutex::new(String::from("FOLLOWER")),
            WAIT_FOR_LEADER: rand::thread_rng().gen_range(300..500),
            LAST_BEAT: Mutex::new(Local::now().timestamp()),
        }
    }
}

pub fn listen_leader_beat(node: web::Data<Node>) {
    println!("begin listen, role {}", node.ROLE.lock().unwrap());
    loop {
        if *node.ROLE.lock().unwrap() == String::from("FOLLOWER") {
        } else if *node.ROLE.lock().unwrap() == String::from("LEADER") {
            println!("be leader");
            break;
        } else if *node.ROLE.lock().unwrap() == String::from("CANDIDATE") {
            println!("be candidate");
            break;
        }
        let now = Local::now().timestamp();
        let last_beat = node.LAST_BEAT.lock().unwrap();
        let wait_for_leader = node.WAIT_FOR_LEADER;
        println!(
            "tick - last_beat({}) - wait_time({})",
            last_beat, wait_for_leader
        );
        if now > *last_beat + wait_for_leader / 1000 {
            // TIMEOUT
            println!("timeout");
        }
        thread::sleep(time::Duration::from_millis(150));
    }
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
            let mut role = node.ROLE.lock().unwrap();
            *role = String::from("LEADER");
        }
        thread::sleep(time::Duration::from_secs(2));
        {
            let node = node_web_data.clone();
            let mut role = node.ROLE.lock().unwrap();
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
            let mut role = node.ROLE.lock().unwrap();
            *role = String::from("LEADER");
        }
        thread::sleep(time::Duration::from_secs(5));
        // h.join().unwrap();
        // h2.join().unwrap();
    }
}


