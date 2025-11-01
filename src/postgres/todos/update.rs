use crate::errors::Result;
use crate::models::{Todo, UpdateTodoRequest};
use crate::postgres::todos::todo_from_row;
use sqlx::PgPool;

pub async fn update_todo(pg_pool: &PgPool, update_todo_request: UpdateTodoRequest) -> Result<Todo> {
    let row = sqlx::query(
        r#"
    update "todos"
    set title = $1,
    description =$2,
    is_completed = $3
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
