use axum::Router;
use crate::state::AppState;

use crate::routes::auth;
use crate::routes::notes;
use crate::routes::notebooks;
use crate::routes::attachments;
use crate::routes::reminders;
use crate::routes::shared_notes;
use crate::routes::note_settings;
use crate::routes::user_settings;




pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth",          auth::router())
        .nest("/notes",         notes::router())
        .nest("/notebooks",     notebooks::router())
        .nest("/attachments",   attachments::router())
        .nest("/reminders",     reminders::router())
        .nest("/shared-notes",  shared_notes::router())
        .nest("/note-settings", note_settings::router())
        .nest("/user-settings", user_settings::router())
}
