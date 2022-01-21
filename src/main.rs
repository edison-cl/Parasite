use std::io;

#[path = "server/engine.rs" ]
mod engine;

#[path = "raft.rs" ]
mod raft;

// #[actix_rt::main]
// async fn main() -> io::Result<()>{
//     engine::server_run().await
// }

fn main(){
    raft::run_test()
}