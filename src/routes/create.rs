use crate::models::*;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

pub async fn createt(
    Extension(pool): Extension<sqlx::PgPool>,
    Json(req): Json<CreateTodoRequest>,
) -> impl IntoResponse {
    match crate::postgres::create_todo(&pool, req).await {
        Ok(created_todo) => {
            let response = TodoResponse {
                id: created_todo.id,
                title: created_todo.title,
                description: created_todo.description,
                is_completed: created_todo.is_completed,
                created_at: created_todo.created_at,
                updated_at: created_todo.updated_at,
                created_by: created_todo.created_by,
            };

            (StatusCode::CREATED, Json(json!(response)))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create Todo"})),
        ),
    }
}
