use actix_web::web;
use super::handler::*;

#[path = "cluster/handle.rs" ]
mod cluster_handle;

pub fn api_routes(sc: &mut web::ServiceConfig) {
    sc.service(
        web::scope("api")
        .route("/alive",web::get().to(health_check_handler))
        .route("/leader/beat",web::get().to(leader_beat))
        .route("/node/info",web::get().to(cluster_handle::node_info))
    );
}



