use crate::{state::AppState, utils::extractors::AuthUser};
use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::attachment::Attachment;

/// Returns a router for attachment-related endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_attachments).post(create_attachment))
        .route("/{id}", get(fetch_attachment).delete(remove_attachment))
}

// --- HANDLERS ---

/// List all attachments that belong to the authenticated user.
/// Only attachments for notes owned by the user will be returned.
pub async fn list_attachments(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<(StatusCode, Json<Vec<Attachment>>), (StatusCode, String)> {
    info!("User {} requested to list all their attachments", user_id);

    let items = sqlx::query_as::<_, Attachment>(
        // Select only attachments belonging to notes owned by the user.
        "SELECT a.* FROM attachments a
         JOIN notes n ON a.note_id = n.id
         WHERE n.user_id = $1",
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch attachments for user {}: {}", user_id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?;
    info!(
        "User {} successfully fetched {} attachments",
        user_id,
        items.len()
    );
    Ok((StatusCode::OK, Json(items)))
}

/// Payload for creating a new attachment.
#[derive(Deserialize)]
pub struct CreateAttachmentPayload {
    pub note_id: Uuid,
    pub filename: String,
    pub url: String,
}

/// Create a new attachment for a note.
/// Note: You should additionally check if the note belongs to the user.
pub async fn create_attachment(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateAttachmentPayload>,
) -> Result<(StatusCode, Json<Attachment>), (StatusCode, String)> {
    info!(
        "User {} is trying to create attachment '{}' for note {}",
        user_id, payload.filename, payload.note_id
    );
    // TODO: Check ownership of note by user_id!

    let a = sqlx::query_as::<_, Attachment>(
        "INSERT INTO attachments (note_id, filename, url) \
             VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(payload.note_id)
    .bind(payload.filename.clone())
    .bind(payload.url.clone())
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        error!(
            "Failed to create attachment for note {} by user {}: {}",
            payload.note_id, user_id, e
        );
        (StatusCode::BAD_REQUEST, "Invalid request".to_string())
    })?;
    info!(
        "Attachment '{}' created with id {} for note {} by user {}",
        a.filename, a.id, a.note_id, user_id
    );
    Ok((StatusCode::CREATED, Json(a)))
}

/// Fetch a single attachment by its ID.
/// Note: Should verify that the attachment belongs to the user (ownership check recommended).
pub async fn fetch_attachment(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Attachment>), (StatusCode, String)> {
    info!(
        "User {} is trying to fetch attachment with id {}",
        user_id, id
    );

    let opt = sqlx::query_as::<_, Attachment>("SELECT * FROM attachments WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            error!(
                "Failed to fetch attachment {} for user {}: {}",
                id, user_id, e
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
        })?;

    if let Some(att) = opt {
        info!(
            "Attachment {} successfully fetched by user {}",
            att.id, user_id
        );
        Ok((StatusCode::OK, Json(att)))
    } else {
        info!("Attachment {} not found for user {}", id, user_id);
        Err((StatusCode::NOT_FOUND, "Attachment not found".to_string()))
    }
}

/// Remove an attachment by its ID.
/// Note: Should verify that the attachment belongs to the user (ownership check recommended).
pub async fn remove_attachment(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!(
        "User {} is trying to remove attachment with id {}",
        user_id, id
    );

    let result = sqlx::query("DELETE FROM attachments WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => {
            info!("Attachment {} successfully removed by user {}", id, user_id);
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            error!(
                "Failed to remove attachment {} by user {}: {}",
                id, user_id, e
            );
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))
        }
    }
}
