use crate::{errors::Result, models::Todo, postgres::todo_from_row};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn fetch_todo(pg_pool: &PgPool, todo_id: Uuid) -> Result<Todo> {
    let row = sqlx::query("select * from todos where id = $1")
        .bind(&todo_id)
        .fetch_one(pg_pool)
        .await?;

    todo_from_row(row)
}
