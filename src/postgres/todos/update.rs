use crate::errors::Result;
use crate::models::{Todo, UpdateTodoRequest};
use crate::postgres::todos::todo_from_row;
use sqlx::PgPool;

pub async fn update_todo(pg_pool: &PgPool, update_todo_request: UpdateTodoRequest) -> Result<Todo> {
    let row = sqlx::query(
        r#"
    update todos
    set title = COALESCE($1, title),
        description = COALESCE($2, description),
        is_completed = COALESCE($3, is_completed)
    where id = $4
    returning *
    "#,
    )
    .bind(update_todo_request.title)
    .bind(update_todo_request.description)
    .bind(update_todo_request.is_completed)
    .bind(update_todo_request.todo_id)
    .fetch_one(pg_pool)
    .await?;
    todo_from_row(row)
}
