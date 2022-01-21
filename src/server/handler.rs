use actix_web::{web, HttpResponse};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[path = "../raft.rs" ]
pub mod raft;
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

pub async fn leader_beat(web_data:web::Data<raft::Node>) -> HttpResponse {
    let mut last_beat = web_data.LAST_BEAT.lock().unwrap();
    *last_beat = Local::now().timestamp();
    HttpResponse::Ok().finish()
}



pub async fn visit_count(app_state:web::Data<state::AppState>) -> HttpResponse {
    let mut count = app_state.count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().json(format!("{}",count))
}