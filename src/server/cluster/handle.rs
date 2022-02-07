use actix_web::HttpResponse;
use chrono::Local;
use serde_json::json;

use super::*;

pub async fn node_info() -> HttpResponse {
    let id = utils::id_generator().unwrap();
    let cluster = cluster::cluster_json().lock().unwrap();
    let data = cluster.as_object().unwrap().get(&id).unwrap();
    HttpResponse::Ok().json(data)
}
pub async fn start_listen_beat() -> HttpResponse {
    if !global_state::STATE
        .lock()
        .unwrap()
        .get("listen_leader_beat")
        .unwrap()
        .to_owned()
    {
        std::thread::spawn(|| cluster::listen_leader_beat());
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}

pub async fn leader_beat() -> HttpResponse {
    *cluster::node_struct().last_leader_beat.lock().unwrap() = Local::now().timestamp();
    HttpResponse::Ok().finish()
}

pub async fn node_id() -> HttpResponse {
    HttpResponse::Ok().json(utils::id_generator().unwrap())
}

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SyncForm {
    content: String,
}
pub async fn node_sync_v1(data: web::Form<SyncForm>) -> HttpResponse {
    let content = data.content.to_owned();
    let remote_cluster: serde_json::value::Value = serde_json::from_str(content.as_str()).unwrap();
    let mut local_cluster = cluster::cluster_json().lock().unwrap();
    let mut callback = false;
    for (id, node) in remote_cluster.as_object().unwrap().iter() {
        match local_cluster.as_object().unwrap().get(id){
            Some(_) => {},
            None => {
                local_cluster
                .as_object_mut()
                .unwrap()
                .insert(id.to_owned(), node.to_owned());
                continue;
            }
        }
        let remote_nv = node
            .as_object()
            .unwrap()
            .get("nv")
            .unwrap()
            .as_i64()
            .unwrap();
        let local_nv = local_cluster
            .as_object()
            .unwrap()
            .get(id)
            .unwrap()
            .as_object()
            .unwrap()
            .get("nv")
            .unwrap()
            .as_i64()
            .unwrap();
        if remote_nv > local_nv {
            local_cluster
                .as_object_mut()
                .unwrap()
                .insert(id.to_owned(), node.to_owned());
        } else if remote_nv < local_nv {
            callback = true;
        }
    }
    if callback {
        HttpResponse::Ok().json(json!({
            "state":"edit",
            "object":local_cluster.as_object().unwrap()
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "state":"ok",
            "object":""
        }))
    }
}
