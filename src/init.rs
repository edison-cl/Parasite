use std::thread;
#[path = "cluster.rs"]
mod cluster;

pub fn prepare(){
    cluster::cluster_init();
    thread::spawn(||cluster::cluster_input_device());
    thread::spawn(||cluster::listen_leader_beat());
    // thread::spawn(||cluster::cluster_input_device());
}