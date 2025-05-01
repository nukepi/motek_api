mod database;
mod models;
mod routes;
mod utils;
mod server;
mod state;

#[tokio::main]
async fn main() {
    server::run().await;
}
