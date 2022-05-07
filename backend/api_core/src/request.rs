
use rocket::http::Status;
use rocket::request::{FromRequest, Request};
use url::Url;
use rocket::request::Outcome;

#[derive(Debug)]
pub enum HostHeaderError {
    Missing,
    Invalid,
}
#[derive(Debug)]
pub struct HostHeader(pub Url);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostHeader{
    type Error = HostHeaderError;

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        
        match request.headers().get_one("Host") {
            Some(h) => {
                let mut full_url=String::from(h);
                if !h.contains("http") {
                   full_url=String::from("http://".to_owned()+h)
                }
                let parsed=Url::parse(&full_url);
                println!("{:?}",h);
                match parsed {
                    Ok(url)=>Outcome::Success(HostHeader(url)),
                    Err(_e)=>Outcome::Failure((Status::BadRequest,HostHeaderError::Invalid))
                }
            },
            None => Outcome::Failure((Status::BadRequest,HostHeaderError::Missing)),
        }
    }
}



