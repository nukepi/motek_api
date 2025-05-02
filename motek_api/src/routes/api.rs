use axum::Router;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse};
use tracing::info; // for logging

use crate::{
    routes::attachments, routes::note_settings, routes::notebooks, routes::notes,
    routes::reminders, routes::shared_notes, routes::user_settings, state::AppState,
    utils::jwt::AuthClaims,
};

/// Protected endpoint available only for users coming from the "web" platform.
pub async fn protected_endpoint(
    State(_state): State<AppState>,
    AuthClaims(claims): AuthClaims,
) -> impl IntoResponse {
    // Log access attempt with user id and platform
    info!(
        "User {} is trying to access /protected from platform '{}'",
        claims.sub, claims.platform
    );

    // Only allow users from the "web" platform
    if claims.platform != "web" {
        info!(
            "Access denied for user {} from platform '{}'",
            claims.sub, claims.platform
        );
        return (StatusCode::FORBIDDEN, "Web only!").into_response();
    }

    // Log successful access
    info!(
        "User {} successfully accessed /protected from platform '{}'",
        claims.sub, claims.platform
    );

    (
        StatusCode::OK,
        format!(
            "Hello, user {} from platform {}",
            claims.sub, claims.platform
        ),
    )
        .into_response()
}

/// Configure all application routes.
/// This includes notes, notebooks, attachments, reminders, shared notes, and user settings.
pub fn router() -> Router<AppState> {
    Router::new()
        // Protected endpoint requiring authentication and correct platform
        .route("/protected", axum::routing::get(protected_endpoint))
        // Notes routes
        .nest("/notes", notes::router())
        // Attachments for a specific note
        .nest("/notes/{note_id}/attachments", attachments::router())
        // Reminders for a specific note
        .nest("/notes/{note_id}/reminders", reminders::router())
        // Settings for a specific note
        .nest("/notes/{note_id}/settings", note_settings::router())
        // Notebooks (global)
        .nest("/notebooks", notebooks::router())
        // Shared notes (global)
        .nest("/shared-notes", shared_notes::router())
        // User settings (global)
        .nest("/user-settings", user_settings::router())
}
