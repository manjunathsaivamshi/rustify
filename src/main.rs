mod errors;
mod models;
mod postgres;
mod routes;

use crate::errors::{Error, Result};
use crate::routes::routes;
use axum::Extension;
use axum::Router;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let pg_pool = postgres::establish_connection(
        "postgres://postgres:postgres@localhost:5432/gaur?sslmode=disable".to_string(),
    )
    .await?;
    let app = Router::new();
    let app = app.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_headers(Any)
            .allow_methods(Any),
    );
    let app = app.merge(routes());

    let app = app.layer(Extension(pg_pool));
    let addr = format!("{}:{}", "localhost", "3000");

    let listener =
        tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|_| Error::StandardError {
                message: "internal_error".to_string(),
            })?;

    axum::serve(listener, app)
        .await
        .map_err(|e| Error::StandardError {
            message: format!("Server error on {}: {}", addr, e),
        })?;

    Ok(())
}
