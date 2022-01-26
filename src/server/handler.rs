use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use chrono::Local;

use self::cluster::Node;

#[path = "../cluster.rs" ]
pub mod cluster;
#[path = "./state.rs" ]
pub mod state;


#[derive(Deserialize, Serialize, Debug, Clone)]
struct Alive {
    time: i64,
    message: String,
}
pub async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().json(Alive{message:String::from("alive"),time:Local::now().timestamp()})
}

pub async fn leader_beat() -> HttpResponse {
    *cluster::node_struct().last_leader_beat.lock().unwrap() = Local::now().timestamp();
    HttpResponse::Ok().finish()
}



pub async fn visit_count(app_state:web::Data<state::AppState>) -> HttpResponse {
    let mut count = app_state.count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().json(format!("{}",count))
}