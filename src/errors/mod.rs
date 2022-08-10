use actix_web::{
    error::HttpError,
    http::{
        header::{ContentType, ToStrError},
        StatusCode,
    },
    HttpResponse, ResponseError,
};
use derive_more::Display;
use std::num::ParseIntError;
use validator::ValidationErrors;

#[derive(Debug, Display)]
pub enum ShopError {
    AlreadyExistsError,
    BcryptError(String),
    ConnectionError,
    DieselError(String),
    InvalidInput,
    JWTError(String),
    NoPermission(String),
    NotEnoughInStockError,
    NotFoundError(String),
    SerdeJsonError(String),
    ToStringError(String),
    ValidationErrors(String),
    ParseError(String),
    HttpError(String),
}

impl ResponseError for ShopError {
    fn status_code(&self) -> StatusCode {
        match self {
            ShopError::AlreadyExistsError => StatusCode::ALREADY_REPORTED,
            ShopError::NotFoundError(_) => StatusCode::NOT_FOUND,
            ShopError::ConnectionError => StatusCode::REQUEST_TIMEOUT,
            ShopError::InvalidInput => StatusCode::BAD_REQUEST,
            ShopError::NoPermission(_) => StatusCode::UNAUTHORIZED,
            ShopError::SerdeJsonError(_) => StatusCode::BAD_REQUEST,
            ShopError::DieselError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ShopError::BcryptError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ShopError::ToStringError(_) => StatusCode::BAD_REQUEST,
            ShopError::JWTError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ShopError::ValidationErrors(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ShopError::NotEnoughInStockError => StatusCode::INTERNAL_SERVER_ERROR,
            ShopError::ParseError(_) => StatusCode::BAD_REQUEST,
            ShopError::HttpError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl From<serde_json::Error> for ShopError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e.to_string())
    }
}

impl From<diesel::result::Error> for ShopError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => ShopError::NotFoundError(e.to_string()),
            e => ShopError::DieselError(e.to_string()),
        }
    }
}
impl From<r2d2::Error> for ShopError {
    fn from(e: r2d2::Error) -> Self {
        ShopError::DieselError(e.to_string())
    }
}

impl From<bcrypt::BcryptError> for ShopError {
    fn from(e: bcrypt::BcryptError) -> Self {
        match e {
            e => ShopError::BcryptError(e.to_string()),
        }
    }
}

impl From<ToStrError> for ShopError {
    fn from(e: ToStrError) -> Self {
        Self::ToStringError(e.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for ShopError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        ShopError::JWTError(e.to_string())
    }
}

impl From<ValidationErrors> for ShopError {
    fn from(e: ValidationErrors) -> Self {
        ShopError::ValidationErrors(e.to_string())
    }
}

impl From<ParseIntError> for ShopError {
    fn from(e: ParseIntError) -> Self {
        ShopError::ParseError(e.to_string())
    }
}

impl From<HttpError> for ShopError {
    fn from(e: HttpError) -> Self {
        ShopError::HttpError(e.to_string())
    }
}
