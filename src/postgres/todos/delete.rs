use crate::errors::Result;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_todo(pg_pool: &PgPool, todo_id: Uuid) -> Result<()> {
    let _row = sqlx::query(
        r#"
    delete from "todos"
    where id = $1
    returning *
    "#,
    )
    .bind(&todo_id)
    .execute(pg_pool)
    .await?;

    Ok(())
}
