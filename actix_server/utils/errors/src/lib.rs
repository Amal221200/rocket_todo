use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum TodoError {
    NoTodoFound = 0,
    InvalidData = 2,
    InternalServerError = 3,
    NoSuchTodoFound = 4,
}

impl ResponseError for TodoError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TodoError::NoTodoFound => StatusCode::NOT_FOUND,
            TodoError::InvalidData => StatusCode::BAD_REQUEST,
            TodoError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            TodoError::NoSuchTodoFound => StatusCode::NOT_FOUND,
        }
    }
}
