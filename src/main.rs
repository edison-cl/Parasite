use std::io;


#[path = "server/engine.rs"]
mod engine;


#[actix_rt::main]
async fn main() -> io::Result<()>{
    engine::server_run().await
}
