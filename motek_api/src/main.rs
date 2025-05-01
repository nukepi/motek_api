mod database;
mod models;
mod routes;
mod server;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    if let Err(e) = server::run().await {
    eprintln!("Błąd uruchamiania serwera: {:?}", e);
    }
}
