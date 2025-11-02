mod create;
mod delete;
mod fetch;
mod update;

pub use create::*;
pub use delete::*;
pub use fetch::*;
pub use update::*;

use crate::errors::Result;
use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

pub(crate) fn todo_from_row(row: sqlx::postgres::PgRow) -> Result<crate::models::Todo> {
    Ok(crate::models::Todo {
        id: row.try_get::<Uuid, _>("id")?,
        title: row.try_get::<String, _>("title")?,
        description: row.try_get::<Option<String>, _>("description")?,
        is_completed: row.try_get::<bool, _>("is_completed")?,
        created_by: row.try_get::<String, _>("created_by")?,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        updated_at: row.try_get::<DateTime<Utc>, _>("updated_at")?,
    })
}
