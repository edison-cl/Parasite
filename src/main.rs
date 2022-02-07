use std::io;

#[path = "cluster.rs" ]
mod cluster;
#[path = "init.rs" ]
mod init;
#[path = "server/engine.rs"]
mod engine;
#[path = "utils.rs" ]
mod utils;
#[path = "config.rs" ]
mod config;
#[path = "command.rs" ]
mod command;
#[path = "state.rs" ]
mod global_state;

#[actix_rt::main]
async fn main() -> io::Result<()>{
    engine::server_run().await
}

// use async_std::task::{spawn};
// #[async_std::main]
// async fn main(){
//     spawn(p1());
//     p2().await;
// }

// async fn p1(){
//     loop{
//         println!("1111111111111");
//         std::thread::sleep(std::time::Duration::from_secs(1));
//     }
// }

// async fn p2(){
//     loop{
//         println!("222222222222222222");
//         std::thread::sleep(std::time::Duration::from_secs(1));
//     }
// }