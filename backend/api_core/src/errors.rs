use std::{fmt,error, io::{Cursor}};

use auth::account_service::AuthError;
use couch_rs::error::CouchError;
use rocket::{Request, Response, debug,http::{self, hyper::StatusCode}, response, serde::json::serde_json::{self}};
use rocket::response::Responder;
use serde::Serializer;
use framework::service::ServiceError;

#[derive(Serialize)]
struct SerializeError<'a> {
    #[serde(serialize_with = "serialize_msg")]
    msg: &'a dyn fmt::Display,
}

fn serialize_msg<S>(msg: &&dyn fmt::Display, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.collect_str(msg)
}

#[derive(Debug)]
pub enum ApiError {
    NotFound(Box<dyn error::Error + Send + Sync>),
    Forbidden(Box<dyn error::Error + Send + Sync>),
    BadRequest(Box<dyn error::Error + Send + Sync>),
    AuthFailed(Box<dyn error::Error + Send + Sync>),
    RequestThrottled(Box<dyn error::Error + Send + Sync>),
    Other(Box<dyn error::Error + Send + Sync>),
    ConflictError(Box<dyn error::Error + Send + Sync>),
    UnprocessableEntity(Box<dyn error::Error +Send + Sync>),
    InternalServerError(Box<dyn error::Error + Send + Sync>),
}

impl From<ServiceError> for ApiError{
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::EntityNotFound(msg) => ApiError::NotFound(msg),
            ServiceError::ForbiddenAction(msg) => ApiError::Forbidden(msg),
            ServiceError::DeleteFailed(msg) => ApiError::Forbidden(msg),
            ServiceError::EntityExists(msg) => ApiError::ConflictError(msg),
            ServiceError::InvalidOperation(msg) => ApiError::BadRequest(msg),
            ServiceError::SaveFailed(msg) => ApiError::BadRequest(msg),
            ServiceError::UpdateFailed(msg) => ApiError::BadRequest(msg),
            ServiceError::FailedToConnect(msg) => ApiError::BadRequest(msg),
            ServiceError::UnknownError(msg) => ApiError::InternalServerError(msg),

        }
    }
}

impl From<CouchError> for ApiError{
    fn from(ce: CouchError) -> Self {
        let status=ce.status;
        let message=ce.message;
        match status{
            StatusCode::FORBIDDEN=>{
                ApiError::Forbidden(message.into())
            },
            StatusCode::NOT_FOUND=>{
                ApiError::NotFound(message.into())
            },
            StatusCode::BAD_REQUEST=>{
                ApiError::BadRequest(message.into())
            },
            _ =>{
                ApiError::Other(message.into())
            }
        }
    }
}


impl From<rocket::serde::json::Error<'_>> for ApiError{
    fn from(e: rocket::serde::json::Error) -> Self {
        match e {
            rocket::serde::json::Error::Io(e) => ApiError::Other(e.into()),
            rocket::serde::json::Error::Parse(_msg,e) => ApiError::UnprocessableEntity(e.into()),
        }
    }
}

impl From<AuthError> for ApiError{
    fn from(ce: AuthError) -> Self {

        match ce {
            AuthError::AccountExists(msg)=>{
                ApiError::Forbidden(msg.into())
            },
            AuthError::AccountNotFound(msg)=>{
                ApiError::NotFound(msg.into())
            },
            AuthError::InvalidLogin(msg)=>{
                ApiError::AuthFailed(msg.into())
            },
            AuthError::RsendCodeTooSoon(msg)=>{
                ApiError::RequestThrottled(msg.into())
            },
            AuthError::SMSFailed(msg)=>{
                ApiError::Other(msg.into())
            },
            AuthError::ServiceError(err)=>{
                ApiError::from(err)
            },
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'o> {
        let (status, err) = match self {
            ApiError::NotFound(err) => {
                debug!("request failed with {:?}", err);

                (http::Status::NotFound, err)
            }
            ApiError::BadRequest(err) => {
                debug!("request failed with {:?}", err);

                (http::Status::BadRequest, err)
            }
            ApiError::Other(err) => {
                rocket::error!("request failed with {:?}", err);

                (http::Status::InternalServerError, err)
            },
            ApiError::Forbidden(err)=>{
                rocket::error!("request failed with {:?}", err);
                (http::Status::Forbidden, err)
            },
            ApiError::AuthFailed(err)=>{
                rocket::error!("request failed with {:?}", err);
                (http::Status::Unauthorized, err)
            },
            ApiError::RequestThrottled(e)=>{
                rocket::error!("request failed with {:?}", e);
                (http::Status::BadRequest, e)
            }
            ApiError::UnprocessableEntity(e)=>{
                rocket::error!("request failed with {:?}", e);
                (http::Status::UnprocessableEntity, e)
            }
            ApiError::InternalServerError(e)=>{
                rocket::error!("request failed with {:?}", e);
                (http::Status::InternalServerError, e)
            }
            ApiError::ConflictError(e)=>{
                rocket::error!("request failed with {:?}", e);
                (http::Status::Conflict, e)
            }
        };

        let err = serde_json::to_vec(&SerializeError { msg: &err }).unwrap_or_else(|_| Vec::new());

        Response::build()
            .sized_body(None::<usize>, Cursor::new(err))
            .header(http::ContentType::JSON)
            .status(status)
            .ok()
    }
}