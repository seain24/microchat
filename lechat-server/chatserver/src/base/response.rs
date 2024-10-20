use std::fmt::Debug;

use actix_web::body::{BoxBody, EitherBody};
use actix_web::error::JsonPayloadError;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use thiserror::Error;

pub type Reply<T> = std::result::Result<Response<T>, Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    code: u16,
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,
}

impl<T: Serialize> Response<T> {
    pub fn new(data: Option<T>, msg: Option<&str>) -> Self {
        Response {
            code: StatusCode::OK.as_u16(),
            data,
            msg: msg.map(|v| v.to_string()),
        }
    }

    pub fn ok(data: T) -> Self {
        Response {
            code: 1000,
            data: Some(data),
            msg: None,
        }
    }

    pub fn error(code: u16, msg: Option<String>) -> Self {
        Response {
            code,
            msg,
            data: None,
        }
    }
}

impl<T: Serialize> Default for Response<T> {
    fn default() -> Self {
        Response {
            code: StatusCode::OK.as_u16(),
            data: None,
            msg: None,
        }
    }
}

impl<T: Serialize> From<T> for Response<T> {
    fn from(value: T) -> Self {
        Response {
            code: StatusCode::OK.as_u16(),
            msg: Some("操作成功".to_string()),
            data: Some(value),
        }
    }
}

impl<T: Serialize> Responder for Response<T> {
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

/// 用户未注册
#[derive(Debug, Error)]
pub enum Error {
    #[error("internal lechat-server error")]
    InternalServerError,
    #[error("{0}")]
    ParamInvalid(String),
    #[error("user is not register")]
    UserNotRegistered,
    #[error("username is duplicate")]
    UsernameDuplicate,
    #[error("username or password mismatch")]
    UserNameOrPasswordMismatch,
}

impl Error {
    fn error_code(&self) -> u16 {
        match self {
            Error::InternalServerError => 500,
            Error::ParamInvalid(_) => 1001,
            Error::UserNotRegistered => 1002,
            Error::UsernameDuplicate => 1003,
            Error::UserNameOrPasswordMismatch => 1004,
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::UserNotRegistered => StatusCode::UNAUTHORIZED,
            Error::ParamInvalid(_) | Error::UsernameDuplicate | Error::UserNameOrPasswordMismatch => {
                StatusCode::BAD_REQUEST
            }
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(Response::<()>::error(self.error_code(), Some(self.to_string())))
    }
}

// #[derive(Debug, Serialize)]
// pub struct Error {
//     pub code: u16,
//     pub msg: String,
//     pub reason: String,
// }
//
// impl Default for Error {
//     fn default() -> Self {
//         Error {
//             code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
//             msg: "服务错误, 请重试".to_string(),
//             reason: "UNKNOWN".to_string(),
//         }
//     }
// }

// impl Error {
//     pub fn new(code: u16, msg: &str, reason: &str) -> Self {
//         Error {
//             code,
//             msg: msg.to_string(),
//             reason: reason.to_string(),
//         }
//     }
//
//     /// bad request.
//     #[inline]
//     pub fn bad_request() -> Self {
//         Error::new(StatusCode::BAD_REQUEST.as_u16(), "BAD_REQUEST", "请求错误")
//     }
//
//     pub fn internal_server_error(err: &str) -> Self {
//         Error::new(
//             StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
//             "INTERNAL_SERVER_ERROR",
//             err,
//         )
//     }
//
//     #[inline]
//     pub fn param_invalid(reason: &str) -> Self {
//         Error::new(StatusCode::BAD_REQUEST.as_u16(), "PARAM_INVALID", reason)
//     }
//
//     pub fn user_not_exist(user: String) -> Self {
//         Error::new(
//             StatusCode::BAD_REQUEST.as_u16(),
//             "USER_NOT_EXIST",
//             &format!("用户{user}不存在"),
//         )
//     }
//
//     pub fn username_duplicate() -> Self {
//         Error::new(
//             StatusCode::BAD_REQUEST.as_u16(),
//             "USERNAME_DUPLICATE",
//             "用户名已经被注册",
//         )
//     }
//
//     pub fn phone_duplicate() -> Self {
//         Error::new(
//             StatusCode::BAD_REQUEST.as_u16(),
//             "PHONE_DUPLICATE",
//             "手机号已经被注册",
//         )
//     }
//
//     pub fn email_duplicate() -> Self {
//         Error::new(
//             StatusCode::BAD_REQUEST.as_u16(),
//             "EMAIL_DUPLICATE",
//             "邮箱已经被注册",
//         )
//     }
// }
//
// impl Display for Error {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "code:{}, msg:{}, reason:{}", self.code, self.msg, self.reason)
//     }
// }
//
// impl ResponseError for Error {
//     fn status_code(&self) -> StatusCode {
//         StatusCode::INTERNAL_SERVER_ERROR
//     }
//
//     fn error_response(&self) -> HttpResponse<BoxBody> {
//         let data = serde_json::to_string(self).unwrap();
//         HttpResponseBuilder::new(self.status_code())
//             .append_header(header::ContentType(mime::APPLICATION_JSON))
//             .body(data)
//     }
// }
