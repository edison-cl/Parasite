use actix_web::{web, App, HttpServer,HttpResponse};
use std::io;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use chrono::Local;

#[path = "handler.rs"]
mod handler;
use handler::{raft,state};
#[path = "router.rs"]
mod router;
// 要命!!!!
// app_data结构体引入主体只能有一个,否则各自引入造成类型不对?
// #[path = "state.rs"]
// pub mod state;
#[path = "../utils.rs"]
mod utils;



pub async fn server_run() -> io::Result<()>{
    let config = utils::parse_args();
    
    let node_orgin = raft::Node::new();
    let node_web_data = web::Data::new(node_orgin);
    let node_web_data_clone = node_web_data.clone();
    
    std::thread::spawn(|| raft::listen_leader_beat(node_web_data));
   
    let shared_data = web::Data::new(state::AppState {
        count: Mutex::new(0)
    });
    let app =  move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(node_web_data_clone.clone())
            .configure(router::general_routes)
            .configure(router::test_router)
    };
    HttpServer::new(app)
        .bind(String::from("127.0.0.1:") + &config.port)?
        .run()
        .await
}