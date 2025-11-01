mod create;
mod delete;
mod update;

pub use create::*;
pub use delete::*;
pub use update::*;

use axum::{
    routing::{delete, post, put},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/createtodo", post(createt))
        .route("/udpatetodo", put(updatet))
        .route("/deletetodo/{id}", delete(deletet))
}
