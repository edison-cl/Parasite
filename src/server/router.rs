use actix_web::web;
use super::handler::*;

pub fn general_routes(sc: &mut web::ServiceConfig) {
    sc.service(
        web::scope("api")
        .route("/alive",web::get().to(health_check_handler))
        .route("/leader/beat",web::get().to(leader_beat))
    );
}

pub fn test_router(cfg:&mut web::ServiceConfig) {
    cfg.route("/",web::get().to(visit_count));
}