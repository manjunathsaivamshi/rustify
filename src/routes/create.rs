use crate::{models::*, postgres};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub async fn create_todo(
    Extension(pool): Extension<sqlx::PgPool>,
    Json(req): Json<NewTodo>,
) -> impl IntoResponse {
    let todo = NewTodo {
        name: req.name,
        created_by: req.created_by,
    };

    match crate::postgres::create_todo(&pool, todo).await {
        Ok(created_article) => {
            info!("Article created successfully: {:?}", created_article.id);

            let response = ArticleResponse {
                id: created_article.id.unwrap().into(),
                title: created_article.title,
                content: created_article.content,
                status: created_article.status.to_string(),
                kind: created_article.kind.to_string(),
            };

            (StatusCode::CREATED, Json(json!(response)))
        }
        Err(e) => {
            error!("Failed to create article: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create article"})),
            )
        }
    }
}
