use actix_rt::spawn;
use super::*;

pub async fn prepare(){
    super::cluster::cluster_init();
    super::command::command_parse();
    
    // thread::spawn(||cluster::cluster_input_device());
    // thread::spawn(||cluster::listen_leader_beat());
    spawn(cluster::cluster_input_device());
    spawn(cluster::listen_leader_beat());
}