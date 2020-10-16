use actix_web::{
    error::{BlockingError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
#[derive(Debug, Display, PartialEq)]
pub enum ServiceError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    InternalServerError(String),
    NotFound(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// 自定义错误
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            ServiceError::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            ServiceError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json::<ErrorResponse>(errors.to_vec().into())
            }
            ServiceError::Unauthorized(error) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(error.into())
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

/// 将String转化为ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// 将Vec<String>转化为ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert Thread BlockingErrors to ServiceErrors
impl From<BlockingError<ServiceError>> for ServiceError {
    fn from(error: BlockingError<ServiceError>) -> ServiceError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ServiceError::BlockingError("Thread blocking error".into()),
        }
    }
}
