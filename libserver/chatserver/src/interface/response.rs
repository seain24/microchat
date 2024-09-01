use std::fmt::{Display, Formatter};

use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder, ResponseError};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::error::JsonPayloadError;
use actix_web::http::{header, StatusCode};
use serde::Serialize;

pub type Result<T> = std::result::Result<Response<T>, Error>;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    msg: String,
}

impl<T: Serialize> Default for Response<T> {
    fn default() -> Self {
        Response {
            code: StatusCode::OK.as_u16(),
            msg: "操作成功".to_string(),
            data: None,
        }
    }
}

impl<T> From<T> for Response<T>
where
    T: Serialize,
{
    fn from(value: T) -> Self {
        Response {
            code: StatusCode::OK.as_u16(),
            msg: "操作成功".to_string(),
            data: Some(value),
        }
    }
}

impl<T> Responder for Response<T>
where
    T: Serialize,
{
    type Body = EitherBody<String>;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        match serde_json::to_string(&self) {
            Ok(body) => match HttpResponse::Ok().content_type(mime::APPLICATION_JSON).message_body(body) {
                Ok(res) => res.map_into_left_body(),
                Err(err) => HttpResponse::from_error(err).map_into_right_body(),
            },
            Err(err) => HttpResponse::from_error(JsonPayloadError::Serialize(err)).map_into_right_body(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: u16,
    pub msg: String,
    pub reason: String,
}

impl Default for Error {
    fn default() -> Self {
        Error {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            msg: "服务错误, 请重试".to_string(),
            reason: "UNKNOWN".to_string(),
        }
    }
}

impl Error {
    fn new(code: u16, msg: &str, reason: &str) -> Self {
        Error {
            code,
            msg: msg.to_string(),
            reason: reason.to_string(),
        }
    }
    /// bad request.
    #[inline]
    pub fn bad_request() -> Self {
        Error::new(StatusCode::BAD_REQUEST.as_u16(), "BAD_REQUEST", "请求错误")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "code:{}, msg:{}, reason:{}", self.code, self.msg, self.reason)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let data = serde_json::to_string(self).unwrap();
        HttpResponseBuilder::new(self.status_code())
            .append_header(header::ContentType(mime::APPLICATION_JSON))
            .body(data)
    }
}
