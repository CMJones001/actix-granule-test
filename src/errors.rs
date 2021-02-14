use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    /// Nicely formatted error message
    /// Use a default message that depends on the error type if no message is provided
    pub fn message(&self) -> String {
        match self {
            AppError {
                message: Some(message),
                cause: _,
                error_type: _,
            } => message.clone(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::DbError,
            } => "Unexpected database error".to_string(),
        }
    }

    // Wrapper functino for db_errors
    pub fn db_error(error: impl ToString) -> Self {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

// Error sent to the user
#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_message() {
        let db_error = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::DbError,
        };
        assert_eq!(
            db_error.message(),
            "Unexpected database error".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_custom_message() {
        let custom_msg = "Unable to add new author".to_string();

        let db_error = AppError {
            message: Some(custom_msg.clone()),
            cause: None,
            error_type: AppErrorType::DbError,
        };
        assert_eq!(
            db_error.message(),
            custom_msg,
            "Custom message should be used"
        );
    }
}
