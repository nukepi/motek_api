mod database;
mod model;
mod config;
mod utils; // <-- dodaj to!

use dotenv::dotenv;
use crate::config::config_loader;
use std::io::{self, Write};
use crate::database::db;
use crate::config_loader::Config;
use crate::utils::crypto::hash_password;

#[tokio::main]
async fn main() {
    dotenv().ok();
    check_env();
    let config = config_loader::Config::load();
    let pool = db::get_pool(&config.database_url).await;

    println!("1. Rejestracja\n2. Logowanie\nWybierz opcję:");
    let mut opcja = String::new();
    io::stdin().read_line(&mut opcja).unwrap();

    match opcja.trim() {
        "1" => {
            let (username, password, email) = get_data();
            let password_hash = match hash_password(&password) {
                Ok(hash) => hash,
                Err(e) => {
                    println!("Błąd hashowania hasła: {:?}", e);
                    return;
                }
            };
            match db::register_user(&pool, &username, &password_hash, &email).await {
                Ok(user) => println!("Zarejestrowano: {:?}", user),
                Err(e) => println!("Błąd rejestracji: {:?}", e),
            }
        },
        "2" => {
            let (username, password, _) = get_data();
            match db::login_user(&pool, &username, &password).await {
                Ok(user) => println!("Zalogowano: {:?}", user),
                Err(e) => println!("Błąd logowania: {:?}", e),
            }
        },
        _ => println!("Nieznana opcja"),
    }
}

fn get_data() -> (String, String, String) {
    let mut username = String::new();
    let mut password = String::new();
    let mut email = String::new();

    print!("Podaj login: "); io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    print!("Podaj hasło: "); io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();

    print!("Podaj email: "); io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();

    (username.trim().to_string(), password.trim().to_string(), email.trim().to_string())
}

fn check_env(){
    dotenv::dotenv().ok();
    let config = Config::load();
    println!("{:?}", config);
}
