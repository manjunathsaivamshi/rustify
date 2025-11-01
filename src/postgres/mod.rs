use crate::errors::{InternalSnafu, Result};
use snafu::ResultExt;
use sqlx::{postgres::PgPoolOptions, PgPool};
mod todos;

pub async fn establish_connection(db_url: String) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .context(InternalSnafu {
            message: "Failed to run migrations".to_string(),
        })?;

    Ok(pool)
}
