use chrono::Utc;
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use jsonwebtoken::{Header, Validation};
use jsonwebtoken::{EncodingKey, DecodingKey};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};
use serde::{Deserialize, Serialize};

use crate::account::{Account, Role};
use crate::account_service::AuthError;



#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo{
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub account_id: String,
    pub role:Role
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUser{
    pub account_id: String,
    pub role:Role
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginInfo {
    type Error = AuthError;
    
   async fn from_request(
        request: &'r Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = decode_token(token.to_string()) {
                    return Outcome::Success(token_data.claims);
                    
                } 
            }
        }

        Outcome::Failure((Status::Unauthorized,AuthError::InvalidLogin("Invalid".into())))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = AuthError;
    
   async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let login=request.guard::<LoginInfo>().await;
        if let Outcome::Success(lg)=login{
             if let Role::Admin=lg.role {
                 let admin=AdminUser{
                     account_id:lg.account_id,
                     role:lg.role
                 };
                return Outcome::Success(admin);
             }
        }
        Outcome::Failure((Status::Unauthorized,AuthError::InvalidLogin("Forbidden".into())))
    }
}

pub fn generate_token(login: Account,expire_after:i64) -> String {
    let now = Utc::now().timestamp_millis(); // nanosecond -> millis
    let payload = LoginInfo {
        iat: now,
        exp: now + expire_after,
        account_id: login._id,
        role:login.role,
    };
    jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret("secret".as_bytes())).unwrap()
}

fn decode_token(token: String) -> Result<TokenData<LoginInfo>> {
    jsonwebtoken::decode::<LoginInfo>(&token, &DecodingKey::from_secret("secret".as_bytes()), &Validation::default())
}
