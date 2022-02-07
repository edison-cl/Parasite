use actix_web::web;
use super::*;
#[path = "cluster/handle.rs" ]
mod cluster_handle;
#[path = "normal/handle.rs" ]
mod noraml_handle;

pub fn api_routes(sc: &mut web::ServiceConfig) {
    sc.service(
        web::scope("api")
        .route("/ping",web::get().to(noraml_handle::ping))
        .route("/leader/beat",web::get().to(cluster_handle::leader_beat))
        .route("/node/info",web::get().to(cluster_handle::node_info))
        .route("/node/id",web::get().to(cluster_handle::node_id))
        .route("/node/start_listen_beat",web::get().to(cluster_handle::start_listen_beat))
        .route("/node/sync/v1",web::post().to(cluster_handle::node_sync_v1))
    );
}
