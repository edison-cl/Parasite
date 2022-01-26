use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::Local;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use once_cell::sync::{Lazy, OnceCell};

#[path = "handler.rs"]
mod handler;
use handler::{cluster, state};
#[path = "router.rs"]
mod router;


// 要命!!!!
// app_data结构体引入主体只能有一个,否则各自引入造成类型不对?
// #[path = "state.rs"]
// pub mod state;
#[path = "../utils.rs"]
mod utils;
#[path = "../init.rs" ]
mod init;



pub async fn server_run() -> io::Result<()> {
    init::prepare();
    let config = utils::parse_args();
    // let node_web_data = web::Data::new(cluster::Node::new());
    // let node = node_web_data.clone();
    // 使用one_cell代替webData
    // let node = node_web_data.clone();
    // let shared_data = web::Data::new(state::AppState {
    //     count: Mutex::new(0),
    // });
    let app = move || {
        App::new()
            // .app_data(shared_data.clone())
            // .app_data(node.clone())
            .configure(router::api_routes)
    };
    HttpServer::new(app)
        .bind(String::from("127.0.0.1:") + &config.port)?
        .run()
        .await
}
