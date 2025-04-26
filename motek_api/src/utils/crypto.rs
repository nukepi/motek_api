use argon2::{self, Config};

pub fn hash_password(password: &str) -> String {
    // W realnym kodzie generuj losową sól!
    argon2::hash_encoded(password.as_bytes(), b"randomsalt", &Config::default()).unwrap()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}