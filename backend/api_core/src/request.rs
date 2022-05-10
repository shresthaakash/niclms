
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


pub async fn get_app_id(host:HostHeader,state :&State<AppContext>)->Option<String>{
    let mut resolver=state.app_resolver.lock().await;
    let url=host.0.host_str().unwrap();
    let store_id=resolver.resolve(String::from(url)).await;
    store_id
}


#[rocket::async_trait]
impl <'r> FromRequest<'r> for AppId {
    type Error=ResolveStoreError;

    async fn from_request(request: & 'r Request<'_>)->rocket::request::Outcome<Self, Self::Error> {
        let host_out=HostHeader::from_request(request).await.unwrap();
        let state=request.guard::<&State<AppContext>>().await.unwrap();
        let storeid=get_app_id(host_out, state).await;
        match storeid{
            Some(id)=>Outcome::Success(StoreId(id)),
            None=>Outcome::Failure((Status::BadRequest,ResolveStoreError::ResolutionFailed))
        }
    }
}



