use crate::models::*;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use uuid::Uuid;

pub async fn fetcht(
    Extension(pool): Extension<sqlx::PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match crate::postgres::fetch_todo(&pool, id).await {
        Ok(updated_todo) => {
            let response = TodoResponse {
                id: updated_todo.id,
                title: updated_todo.title,
                description: updated_todo.description,
                is_completed: updated_todo.is_completed,
                created_at: updated_todo.created_at,
                updated_at: updated_todo.updated_at,
                created_by: updated_todo.created_by,
            };

            (StatusCode::CREATED, Json(json!(response)))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to fetch Todo,{}",e)})),
        ),
    }
}
