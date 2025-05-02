use crate::api_handlers::auth::*;
use crate::api_handlers::notes::*;
use crate::api_handlers::notebooks::*;
use crate::api_handlers::note_settings::*;
use crate::api_handlers::attachments::*;
use crate::api_handlers::reminders::*;
use crate::api_handlers::shared_notes::*;
use crate::api_handlers::user_settings::*;


// --- AUTH ---
pub async fn login(email: String, password: String) -> AuthResponse {
    crate::api_handlers::auth::login(email, password).await
}
#[flutter_rust_bridge::frb]
pub async fn register(email: String, password: String) -> AuthResponse {
    crate::api_handlers::auth::register(email, password).await
}

// --- NOTES ---
#[flutter_rust_bridge::frb]
pub async fn list_notes() -> Vec<Note> {
    crate::api_handlers::notes::list_notes().await
}


pub async fn get_note(note_id: String) -> Option<Note> {
    crate::api_handlers::notes::get_note(note_id).await
}


pub async fn create_note(title: String, content: String) -> Option<Note> {
    crate::api_handlers::notes::create_note(title, content).await
}


pub async fn update_note(note_id: String, title: Option<String>, content: Option<String>) -> Option<Note> {
    crate::api_handlers::notes::update_note(note_id, title, content).await
}

pub async fn delete_note(note_id: String) -> bool {
    crate::api_handlers::notes::delete_note(note_id).await
}

// --- NOTEBOOKS ---
pub async fn list_notebooks() -> Vec<Notebook> {
    crate::api_handlers::notebooks::list_notebooks().await
}


pub async fn get_notebook(notebook_id: String) -> Option<Notebook> {
    crate::api_handlers::notebooks::get_notebook(notebook_id).await
}


pub async fn create_notebook(name: String, parent_id: Option<String>) -> Option<Notebook> {
    crate::api_handlers::notebooks::create_notebook(name, parent_id).await
}


pub async fn update_notebook(notebook_id: String, name: Option<String>, parent_id: Option<String>) -> Option<Notebook> {
    crate::api_handlers::notebooks::update_notebook(notebook_id, name, parent_id).await
}


pub async fn delete_notebook(notebook_id: String) -> bool {
    crate::api_handlers::notebooks::delete_notebook(notebook_id).await
}

// --- NOTE_SETTINGS ---

pub async fn list_note_settings(user_id: String) -> Vec<NoteSettings> {
    crate::api_handlers::note_settings::list_note_settings(user_id).await
}


pub async fn get_note_settings(settings_id: String) -> Option<NoteSettings> {
    crate::api_handlers::note_settings::get_note_settings(settings_id).await
}


pub async fn create_note_settings(note_id: String, color: String, font: String, view_mode: String) -> Option<NoteSettings> {
    crate::api_handlers::note_settings::create_note_settings(note_id, color, font, view_mode).await
}


pub async fn update_note_settings(settings_id: String, color: Option<String>, font: Option<String>, view_mode: Option<String>) -> Option<NoteSettings> {
    crate::api_handlers::note_settings::update_note_settings(settings_id, color, font, view_mode).await
}


pub async fn delete_note_settings(settings_id: String) -> bool {
    crate::api_handlers::note_settings::delete_note_settings(settings_id).await
}

// --- ATTACHMENTS ---

pub async fn list_attachments() -> Vec<Attachment> {
    crate::api_handlers::attachments::list_attachments().await
}


pub async fn get_attachment(attachment_id: String) -> Option<Attachment> {
    crate::api_handlers::attachments::get_attachment(attachment_id).await
}


pub async fn create_attachment(note_id: String, filename: String, url: String) -> Option<Attachment> {
    crate::api_handlers::attachments::create_attachment(note_id, filename, url).await
}


pub async fn update_attachment(attachment_id: String, filename: Option<String>, url: Option<String>) -> Option<Attachment> {
    crate::api_handlers::attachments::update_attachment(attachment_id, filename, url).await
}


pub async fn delete_attachment(attachment_id: String) -> bool {
    crate::api_handlers::attachments::delete_attachment(attachment_id).await
}

// --- REMINDERS ---

pub async fn list_reminders(user_id: String) -> Vec<Reminder> {
    crate::api_handlers::reminders::list_reminders(user_id).await
}


pub async fn get_reminder(reminder_id: String) -> Option<Reminder> {
    crate::api_handlers::reminders::get_reminder(reminder_id).await
}


pub async fn create_reminder(note_id: String, remind_at: i64) -> Option<Reminder> {
    crate::api_handlers::reminders::create_reminder(note_id, remind_at).await
}


pub async fn update_reminder(reminder_id: String, is_done: Option<bool>, remind_at: Option<i64>) -> Option<Reminder> {
    crate::api_handlers::reminders::update_reminder(reminder_id, is_done, remind_at).await
}


pub async fn delete_reminder(reminder_id: String) -> bool {
    crate::api_handlers::reminders::delete_reminder(reminder_id).await
}

// --- SHARED NOTES ---

pub async fn list_shared_notes(user_id: String) -> Vec<SharedNote> {
    crate::api_handlers::shared_notes::list_shared_notes(user_id).await
}


pub async fn get_shared_note(note_id: String, user_id: String) -> Option<SharedNote> {
    crate::api_handlers::shared_notes::get_shared_note(note_id, user_id).await
}


pub async fn create_shared_note(note_id: String, user_id: String, role: String) -> Option<SharedNote> {
    crate::api_handlers::shared_notes::create_shared_note(note_id, user_id, role).await
}


pub async fn update_shared_note(note_id: String, user_id: String, role: String) -> Option<SharedNote> {
    crate::api_handlers::shared_notes::update_shared_note(note_id, user_id, role).await
}


pub async fn delete_shared_note(note_id: String, user_id: String) -> bool {
    crate::api_handlers::shared_notes::delete_shared_note(note_id, user_id).await
}

// --- USER SETTINGS ---

pub async fn get_user_settings(user_id: String) -> Option<UserSettings> {
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
    crate::api_handlers::user_settings::delete_user_settings(settings_id).await
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}