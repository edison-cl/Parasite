use super::*;
use actix_web::*;
use reqwest;
use std::io::{self};
use std::process::exit;
use std::thread;

#[path = "router.rs"]
mod router;

// 要命!!!!
// app_data结构体引入主体只能有一个,否则各自引入造成类型不对?
// #[path = "state.rs"]
// pub mod state;

pub async fn server_run() -> io::Result<()> {
    init::prepare().await;
    if !global_state::STATE
        .lock()
        .unwrap()
        .get("start_server")
        .unwrap()
    {
        exit(0)
    };

    thread::spawn(|| {
        let mut timeout_count = 0;
        loop {
            match watch_server() {
                Ok(_) => {
                    utils::ColorPrint::greenln(format!(
                        "🚀start server => listening port :{}",
                        config::global_data::port()
                    ));
                    break;
                }
                Err(_) => {
                    timeout_count += 1;
                    if timeout_count > 3 {
                        utils::ColorPrint::redln(format!("❌failed start server, cause timeout"));
                        exit(0);
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });
    let app = move || App::new().configure(router::api_routes);
    HttpServer::new(app)
        .bind(format!("127.0.0.1:{}", config::global_data::port()))?
        .run()
        .await
}

fn watch_server() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://127.0.0.1:{}/api/ping", config::global_data::port());
    println!("🔎detect => {}", url);
    reqwest::blocking::get(url)?.json::<String>()?;
    Ok(())
}
