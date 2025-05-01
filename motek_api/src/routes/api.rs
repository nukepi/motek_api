use crate::state::AppState;
use axum::Router;

use crate::routes::attachments;
use crate::routes::note_settings;
use crate::routes::notebooks;
use crate::routes::notes;
use crate::routes::reminders;
use crate::routes::shared_notes;
use crate::routes::user_settings;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/notes", notes::router())
        .nest("/notes/{note_id}/attachments", attachments::router())
        .nest("/notes/{note_id}/reminders", reminders::router())
        .nest("/notes/{note_id}/settings", note_settings::router())
        // jeśli chcesz też globalnie:
        .nest("/notebooks", notebooks::router())
        .nest("/shared-notes", shared_notes::router())
        .nest("/user-settings", user_settings::router())
}
