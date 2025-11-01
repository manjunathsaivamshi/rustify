use crate::models::*;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use uuid::Uuid;

pub async fn updatet(
    Extension(pool): Extension<sqlx::PgPool>,
    Json(req): Json<UpdateTodoRequest>,
) -> impl IntoResponse {
    match crate::postgres::update_todo(&pool, req).await {
        Ok(updated_todo) => {
            let response = TodoResponse {
                id: updated_todo.id,
                title: updated_todo.title,
                description: updated_todo.description,
                is_completed: updated_todo.is_completed,
                created_at: updated_todo.created_at,
                updated_at: updated_todo.updated_at,
            };

            (StatusCode::CREATED, Json(json!(response)))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update Todo"})),
        ),
    }
}
