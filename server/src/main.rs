use crate::server::run_server;

mod server;
mod endpoints;
mod data_source;

#[tokio::main]
async fn main() {
    run_server().await;
}
