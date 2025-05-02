pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".into());
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err("Password must contain a lowercase letter".into());
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("Password must contain an uppercase letter".into());
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain a digit".into());
    }
    if !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        return Err("Password must contain a special character".into());
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), String> {
    if !email.contains('@') {
        return Err("Email must contain an @ symbol".into());
    }
    if !email.contains('.') {
        return Err("Email must contain a . symbol".into());
    }
    if email.is_empty() {
        return Err("Invalid email address".into());
    }
    Ok(())
}