mod create;
mod delete;
mod fetch;
mod update;

pub use create::*;
pub use delete::*;
pub use fetch::*;
pub use update::*;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/createtodo", post(createt))
        .route("/udpatetodo", put(updatet))
        .route("/deletetodo/{id}", delete(deletet))
        .route("/fetchtodo/{id}", get(fetcht))
}
