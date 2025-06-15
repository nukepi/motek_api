use crate::api_handlers::auth::*;
use crate::api_handlers::notes::*;
use crate::api_handlers::notebooks::*;
use crate::api_handlers::note_settings::*;
use crate::api_handlers::attachments::*;
use crate::api_handlers::reminders::*;
use crate::api_handlers::shared_notes::*;
use crate::api_handlers::user_settings::*;
use tracing::{info, debug, error};
use std::io::Read;
use std::path::Path;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::atomic::{AtomicBool, Ordering};

static LOGGING_INITIALIZED: AtomicBool = AtomicBool::new(false);

// --- AUTH ---
pub async fn login(email: String, password: String) -> AuthResponse {
    debug!("Calling login function with email: {}", email);
    crate::api_handlers::auth::login(email, password).await
}

#[flutter_rust_bridge::frb]
pub async fn register(email: String, password: String) -> AuthResponse {
    debug!("Calling register function with email: {}", email);
    crate::api_handlers::auth::register(email, password).await
}

// --- NOTES ---
#[flutter_rust_bridge::frb]
pub async fn list_notes() -> Vec<Note> {
    debug!("Calling list_notes function");
    crate::api_handlers::notes::list_notes().await
}


pub async fn get_note(note_id: String) -> Option<Note> {
    debug!("Calling get_note function with id: {}", note_id);
    crate::api_handlers::notes::get_note(note_id).await
}


pub async fn create_note(title: String, content: String) -> Option<Note> {
    debug!("Calling create_note function with title: {}", title);
    crate::api_handlers::notes::create_note(title, content).await
}


pub async fn update_note(note_id: String, title: Option<String>, content: Option<String>) -> Option<Note> {
    debug!("Calling update_note function with id: {}", note_id);
    crate::api_handlers::notes::update_note(note_id, title, content).await
}

pub async fn delete_note(note_id: String) -> bool {
    debug!("Calling delete_note function with id: {}", note_id);
    crate::api_handlers::notes::delete_note(note_id).await
}

// --- NOTEBOOKS ---
pub async fn list_notebooks() -> Vec<Notebook> {
    debug!("Calling list_notebooks function");
    crate::api_handlers::notebooks::list_notebooks().await
}

pub async fn get_notebook(notebook_id: String) -> Option<Notebook> {
    debug!("Calling get_notebook function with id: {}", notebook_id);
    crate::api_handlers::notebooks::get_notebook(notebook_id).await
}

pub async fn create_notebook(name: String, parent_id: Option<String>) -> Option<Notebook> {
    debug!("Calling create_notebook function with name: {}", name);
    crate::api_handlers::notebooks::create_notebook(name, parent_id).await
}

pub async fn update_notebook(notebook_id: String, name: Option<String>, parent_id: Option<String>) -> Option<Notebook> {
    debug!("Calling update_notebook function with id: {}", notebook_id);
    crate::api_handlers::notebooks::update_notebook(notebook_id, name, parent_id).await
}

pub async fn delete_notebook(notebook_id: String) -> bool {
    debug!("Calling delete_notebook function with id: {}", notebook_id);
    crate::api_handlers::notebooks::delete_notebook(notebook_id).await
}

// --- NOTE_SETTINGS ---
pub async fn list_note_settings(user_id: String) -> Vec<NoteSettings> {
    debug!("Calling list_note_settings function for user: {}", user_id);
    crate::api_handlers::note_settings::list_note_settings(user_id).await
}

pub async fn get_note_settings(settings_id: String) -> Option<NoteSettings> {
    debug!("Calling get_note_settings function with id: {}", settings_id);
    crate::api_handlers::note_settings::get_note_settings(settings_id).await
}

pub async fn create_note_settings(note_id: String, color: String, font: String, view_mode: String) -> Option<NoteSettings> {
    debug!("Calling create_note_settings function for note: {}", note_id);
    crate::api_handlers::note_settings::create_note_settings(note_id, color, font, view_mode).await
}

pub async fn update_note_settings(settings_id: String, color: Option<String>, font: Option<String>, view_mode: Option<String>) -> Option<NoteSettings> {
    debug!("Calling update_note_settings function with id: {}", settings_id);
    crate::api_handlers::note_settings::update_note_settings(settings_id, color, font, view_mode).await
}

pub async fn delete_note_settings(settings_id: String) -> bool {
    debug!("Calling delete_note_settings function with id: {}", settings_id);
    crate::api_handlers::note_settings::delete_note_settings(settings_id).await
}

// --- ATTACHMENTS ---
pub async fn list_attachments() -> Vec<Attachment> {
    debug!("Calling list_attachments function");
    crate::api_handlers::attachments::list_attachments().await
}

pub async fn get_attachment(attachment_id: String) -> Option<Attachment> {
    debug!("Calling get_attachment function with id: {}", attachment_id);
    crate::api_handlers::attachments::get_attachment(attachment_id).await
}

pub async fn create_attachment(note_id: String, filename: String, url: String) -> Option<Attachment> {
    debug!("Calling create_attachment function for note: {}", note_id);
    crate::api_handlers::attachments::create_attachment(note_id, filename, url).await
}

pub async fn update_attachment(attachment_id: String, filename: Option<String>, url: Option<String>) -> Option<Attachment> {
    debug!("Calling update_attachment function with id: {}", attachment_id);
    crate::api_handlers::attachments::update_attachment(attachment_id, filename, url).await
}

pub async fn delete_attachment(attachment_id: String) -> bool {
    debug!("Calling delete_attachment function with id: {}", attachment_id);
    crate::api_handlers::attachments::delete_attachment(attachment_id).await
}

// --- REMINDERS ---
pub async fn list_reminders(user_id: String) -> Vec<Reminder> {
    debug!("Calling list_reminders function for user: {}", user_id);
    crate::api_handlers::reminders::list_reminders(user_id).await
}

pub async fn get_reminder(reminder_id: String) -> Option<Reminder> {
    debug!("Calling get_reminder function with id: {}", reminder_id);
    crate::api_handlers::reminders::get_reminder(reminder_id).await
}

pub async fn create_reminder(note_id: String, remind_at: i64) -> Option<Reminder> {
    debug!("Calling create_reminder function for note: {}", note_id);
    crate::api_handlers::reminders::create_reminder(note_id, remind_at).await
}

pub async fn update_reminder(reminder_id: String, is_done: Option<bool>, remind_at: Option<i64>) -> Option<Reminder> {
    debug!("Calling update_reminder function with id: {}", reminder_id);
    crate::api_handlers::reminders::update_reminder(reminder_id, is_done, remind_at).await
}

pub async fn delete_reminder(reminder_id: String) -> bool {
    debug!("Calling delete_reminder function with id: {}", reminder_id);
    crate::api_handlers::reminders::delete_reminder(reminder_id).await
}

// --- SHARED NOTES ---
pub async fn list_shared_notes(user_id: String) -> Vec<SharedNote> {
    debug!("Calling list_shared_notes function for user: {}", user_id);
    crate::api_handlers::shared_notes::list_shared_notes(user_id).await
}

pub async fn get_shared_note(note_id: String, user_id: String) -> Option<SharedNote> {
    debug!("Calling get_shared_note function for note: {} and user: {}", note_id, user_id);
    crate::api_handlers::shared_notes::get_shared_note(note_id, user_id).await
}

pub async fn create_shared_note(note_id: String, user_id: String, role: String) -> Option<SharedNote> {
    debug!("Calling create_shared_note function for note: {} and user: {}", note_id, user_id);
    crate::api_handlers::shared_notes::create_shared_note(note_id, user_id, role).await
}

pub async fn update_shared_note(note_id: String, user_id: String, role: String) -> Option<SharedNote> {
    debug!("Calling update_shared_note function for note: {} and user: {}", note_id, user_id);
    crate::api_handlers::shared_notes::update_shared_note(note_id, user_id, role).await
}

pub async fn delete_shared_note(note_id: String, user_id: String) -> bool {
    debug!("Calling delete_shared_note function for note: {} and user: {}", note_id, user_id);
    crate::api_handlers::shared_notes::delete_shared_note(note_id, user_id).await
}

// --- USER SETTINGS ---

pub async fn get_user_settings(user_id: String) -> Option<UserSettings> {
    debug!("Calling get_user_settings function for user: {}", user_id);
    crate::api_handlers::user_settings::get_user_settings(user_id).await
}

pub async fn create_user_settings(
    user_id: String,
    lang: String,
    theme: String,
    timezone: String,
    notifications_enabled: bool,
    default_sort: String,
    editor_mode: String,
) -> Option<UserSettings> {
    debug!("Calling create_user_settings function for user: {}", user_id);
    crate::api_handlers::user_settings::create_user_settings(
        user_id,
        lang,
        theme,
        timezone,
        notifications_enabled,
        default_sort,
        editor_mode,
    ).await
} 

pub async fn update_user_settings(
    settings_id: String,
    lang: Option<String>,
    theme: Option<String>,
    timezone: Option<String>,
    notifications_enabled: Option<bool>,
    default_sort: Option<String>,
    editor_mode: Option<String>,
) -> Option<UserSettings> {
    debug!("Calling update_user_settings function with id: {}", settings_id);
    crate::api_handlers::user_settings::update_user_settings(
        settings_id,
        lang,
        theme,
        timezone,
        notifications_enabled,
        default_sort,
        editor_mode,
    ).await
}

pub async fn delete_user_settings(settings_id: String) -> bool {
    debug!("Calling delete_user_settings function with id: {}", settings_id);
    crate::api_handlers::user_settings::delete_user_settings(settings_id).await
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Inicjalizacja logowania przy starcie aplikacji
    let log_dir = "logs";
    let log_path = "motek_ui";
    setup_multi_logging(log_dir, log_path);
    
    info!("Motek UI Rust library initialized");

    // Inicjalizacja tokenów
    crate::utils::token::initialize_tokens();
    crate::utils::helpers::initialize_tokens_from_main_storage();
    
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb]
pub fn refresh_tokens() -> bool {
    debug!("Manually refreshing token storages");
    
    // Inicjalizacja tokenów
    crate::utils::token::initialize_tokens();
    crate::utils::helpers::initialize_tokens_from_main_storage();
    
    // Sprawdź, czy tokeny są dostępne
    let has_tokens = crate::utils::token::has_tokens();
    info!("Token refresh completed, tokens available: {}", has_tokens);
    
    has_tokens
}

#[flutter_rust_bridge::frb]
pub fn configure_logging(log_dir: String, log_file_prefix: String) {
    info!("Configuring logging with dir: {}, prefix: {}", log_dir, log_file_prefix);
    setup_multi_logging(&log_dir, &log_file_prefix);
    info!("Logging reconfigured successfully");
}

#[flutter_rust_bridge::frb]
pub async fn check_api_connection() -> CheckApiResult {
    debug!("Checking API connection");
    
    let client = reqwest::Client::new();
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://139.59.138.164:3000".to_string());
    
    info!("Checking API connection to: {}", api_url);
    
    let url = format!("{}/api/health", api_url);
    
    match client.get(&url).timeout(std::time::Duration::from_secs(10)).send().await {
        Ok(resp) => {
            let status = resp.status();
            info!("API health check response status: {}", status);
            
            match resp.text().await {
                Ok(body) => {
                    info!("API health check response body: {}", body);
                    CheckApiResult {
                        success: status.is_success(),
                        status_code: status.as_u16() as i32,
                        message: format!("API responded with status: {} and body: {}", status, body),
                    }
                },
                Err(e) => {
                    error!("Failed to read API health check response: {}", e);
                    CheckApiResult {
                        success: false,
                        status_code: status.as_u16() as i32,
                        message: format!("Failed to read response body: {}", e),
                    }
                }
            }
        },
        Err(e) => {
            error!("API connection check failed: {}", e);
            CheckApiResult {
                success: false,
                status_code: 0,
                message: format!("Connection error: {}", e),
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckApiResult {
    pub success: bool,
    pub status_code: i32,
    pub message: String,
}

#[flutter_rust_bridge::frb]
pub fn get_logs(log_file_path: Option<String>) -> String {
    let log_path = match log_file_path {
        Some(path) => path,
        None => format!("{}/motek_ui.log", std::env::temp_dir().to_string_lossy())
    };
    
    debug!("Attempting to read log file: {}", log_path);
    
    match std::fs::File::open(&log_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    info!("Successfully read log file, size: {} bytes", contents.len());
                    contents
                },
                Err(e) => {
                    error!("Failed to read log file contents: {}", e);
                    format!("Error reading log file: {}", e)
                }
            }
        },
        Err(e) => {
            error!("Failed to open log file: {}", e);
            format!("Error opening log file at {}: {}", log_path, e)
        }
    }
}



use std::env;

#[flutter_rust_bridge::frb]
pub fn set_api_url(url: String) -> bool {
    info!("Setting API URL to: {}", url);
    
    env::set_var("API_URL", &url);
    
    let current = env::var("API_URL").unwrap_or_default();
    let success = current == url;
    
    if success {
        info!("API URL successfully set to: {}", url);
    } else {
        error!("Failed to set API URL. Current value: {}", current);
    }
    
    success
}

#[flutter_rust_bridge::frb]
pub fn get_api_url() -> String {
    let url = env::var("API_URL").unwrap_or_else(|_| "http://139.59.138.164:3000".to_string());
    info!("Current API URL: {}", url);
    url
}

fn setup_logging() {
    let log_dir = "logs";
    let log_path = "myapp";
    
    if !Path::new(log_dir).exists() {
        std::fs::create_dir_all(log_dir).expect("Failed to create log directory");
    }
    
    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(Rotation::DAILY) // Rotacja plików codziennie
        .filename_prefix(log_path) // Poprawiona metoda (zamiast .filename())
        .filename_suffix("log")    // Sufiks nazwy pliku (rozszerzenie)
        .build(log_dir)            // Katalog docelowy
        .expect("Failed to create rolling file appender");
    
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(non_blocking))
        .init();
    
    tracing::info!("Aplikacja uruchomiona");
}

static mut LOG_GUARD: Option<tracing_appender::non_blocking::WorkerGuard> = None;

fn setup_multi_logging(log_dir: &str, log_path: &str) {
    if LOGGING_INITIALIZED.load(Ordering::SeqCst) {
        eprintln!("Logging already initialized, skipping setup");
        return;
    }

    if !std::path::Path::new(log_dir).exists() {
        std::fs::create_dir_all(log_dir).expect("Nie udało się utworzyć katalogu logów");
    }
    
    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix(log_path)
        .filename_suffix("log")
        .build(log_dir)
        .expect("Nie udało się utworzyć rolling file appender");
    
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    
    unsafe {
        LOG_GUARD = Some(guard);
    }
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking)) 
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr)) 
        .init();

    LOGGING_INITIALIZED.store(true, Ordering::SeqCst);
        
    tracing::info!("System logowania zainicjalizowany w katalogu {} z prefixem {}", log_dir, log_path);
}

#[flutter_rust_bridge::frb]
pub fn test_rust_logging() {
    tracing::trace!("This is a TRACE message from Rust");
    tracing::debug!("This is a DEBUG message from Rust");
    tracing::info!("This is an INFO message from Rust");
    tracing::warn!("This is a WARN message from Rust");
    tracing::error!("This is an ERROR message from Rust");
}

#[flutter_rust_bridge::frb]
pub fn set_flutter_log_callback(callback: fn(level: String, message: String)) {
    let wrapper = move |level: &str, message: &str| {
        callback(level.to_string(), message.to_string());
    };
    
    crate::utils::logging::set_log_callback(wrapper);
    
    crate::rust_info!("Flutter log callback set successfully");
}

#[flutter_rust_bridge::frb]
pub fn setup_logging_bridge(log_level: String, log_file_path: Option<String>) {
    if LOGGING_INITIALIZED.load(Ordering::SeqCst) {
        info!("Logging already initialized. Changing log level to: {}", log_level);
        if let Some(path) = log_file_path {
            info!("Log file path: {}", path);
        }
        return;
    }

    let log_dir = match &log_file_path {
        Some(path) => {
            let p = std::path::Path::new(path);
            if let Some(parent) = p.parent() {
                parent.to_string_lossy().to_string()
            } else {
                "logs".to_string()
            }
        },
        None => "logs".to_string()
    };
    
    let log_path = match &log_file_path {
        Some(path) => {
            let p = std::path::Path::new(path);
            if let Some(file_stem) = p.file_stem() {
                file_stem.to_string_lossy().to_string()
            } else {
                "motek_ui".to_string()
            }
        },
        None => "motek_ui".to_string()
    };
    
    setup_multi_logging(&log_dir, &log_path);
    
    info!("Logging initialized with level: {}", log_level);
    if let Some(path) = log_file_path {
        info!("Log file path: {}", path);
    } else {
        info!("Using default log file path");
    }
}

#[flutter_rust_bridge::frb]
pub fn is_user_logged_in() -> bool {
    crate::utils::token::has_tokens()
}

#[flutter_rust_bridge::frb]
pub fn get_logged_in_email() -> Option<String> {
    crate::utils::token::get_user_email()
}

#[flutter_rust_bridge::frb]
pub fn get_logged_in_user_id() -> Option<String> {
    crate::utils::token::get_user_id()
}

#[flutter_rust_bridge::frb]
pub fn logout_user() -> bool {
    crate::utils::token::logout();
    true
}

