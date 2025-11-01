mod create;

use axum::{
    routing::{post, put},
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/newtodo", post(create_todo))
}
