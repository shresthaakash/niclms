#[macro_use]
extern crate rocket;
extern crate infra;
mod sms_payload;


use rocket::{Build, Rocket,serde::json::Json};
use sms_payload::SMSPayload;
use infra::sms::call_sms_service;
use dotenv::dotenv;

#[get("/")]
fn hello() -> &'static str {
    "{name: sms_api , version:1.0}"
}

#[get("/send?<to>&<text>&<hash>")]
async fn send_sms(to:&str,text:&str,hash:&str) -> &'static str {
    call_sms_service(to.into(),text.into()).await.unwrap();
    "{sent:OK}"
}

pub fn init_app() ->Rocket<Build>{
    dotenv().ok();
    let mut rc=rocket::build();

    rc=rc.mount("/",routes![hello,send_sms]);
    
    rc
}




#[launch]
fn launch() -> _ {
    init_app()
}