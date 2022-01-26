use actix_web::{web, HttpResponse};

#[path = "../../cluster.rs" ]
mod cluster;

pub fn node_info() -> HttpResponse {
    let node = cluster::cluster_json().lock().unwrap();
    HttpResponse::Ok().json(node.as_object())
}


