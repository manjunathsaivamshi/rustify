use crate::errors::Result;
use crate::models::{CreateTodoRequest, Todo};
use crate::postgres::todos::todo_from_row;
use sqlx::PgPool;

pub async fn create_todo(pg_pool: &PgPool, create_todo_request: CreateTodoRequest) -> Result<Todo> {
    let row = sqlx::query(
        r#"
    insert into todos (title, description, created_by) values ($1, $2, $3)
    returning *
    "#,
    )
    .bind(create_todo_request.title)
    .bind(create_todo_request.description)
    .bind(create_todo_request.created_by)
    .fetch_one(pg_pool)
    .await?;
    todo_from_row(row)
}
