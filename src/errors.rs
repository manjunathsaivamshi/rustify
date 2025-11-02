use axum::{http::StatusCode, response::IntoResponse};
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Duplicate field found :{}, {}", source, message))]
    UniqueViolation {
        source: sqlx::Error,
        message: String,
    },
    #[snafu(display("InternalError Occuered :{}, {}", source, message))]
    InternalError {
        source: sqlx::Error,
        message: String,
    },
    #[snafu(display("StandardError Occuered : {}", message))]
    StandardError { message: String },
    #[snafu(display("RowNotFound: {}", message))]
    RowNotFoundError { message: String },
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            Self::UniqueViolation { source, message } => (
                StatusCode::CONFLICT,
                format!("source : {}, message: {}", source, message),
            ),
            Self::RowNotFoundError { message } => {
                (StatusCode::NOT_FOUND, format!("message:{}", message))
            }
            Self::StandardError { message } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("message:{}", message),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unkown Error".to_string(),
            ),
        };
        (status, message).into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        use sqlx::Error as SqlxError;

        match value {
            SqlxError::RowNotFound => Self::RowNotFoundError {
                message: "RowNotFound".to_string(),
            },
            SqlxError::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    match code.as_ref() {
                        "23505" => Self::UniqueViolation {
                            message: db_err.message().to_string(),
                            source: SqlxError::Database(db_err),
                        },
                        _ => Self::InternalError {
                            message: db_err.message().to_string(),
                            source: SqlxError::Database(db_err),
                        },
                    }
                } else {
                    Self::InternalError {
                        message: db_err.message().to_string(),
                        source: SqlxError::Database(db_err),
                    }
                }
            }
            _ => Self::InternalError {
                source: value,
                message: "Unkown Error".to_string(),
            },
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
