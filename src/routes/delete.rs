use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use uuid::Uuid;

pub async fn deletet(
    Extension(pool): Extension<sqlx::PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match crate::postgres::delete_todo(&pool, id).await {
        Ok(()) => {
            let response = {};
            (StatusCode::CREATED, Json(json!(response)))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete Todo"})),
        ),
    }
}
