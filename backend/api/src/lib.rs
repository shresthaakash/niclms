#[macro_use]
extern crate rocket;
extern crate api_core;
extern crate users;
extern crate dotenv;
extern crate rocket_cors;
extern crate infra;

mod users_api;
mod accounts_api;
mod file_api;


use rocket::{Build, Rocket, http::Method};
use users_api::user_routes;
use accounts_api::auth_routes;
use api_core::app_context::AppContext;
use rocket_cors::{AllowedHeaders,AllowedOrigins};


use dotenv::dotenv;

use crate::file_api::file_routes;

#[get("/")]
fn hello() -> &'static str {
    "{version:1.0}"
}




pub fn init_app() ->Rocket<Build>{
    dotenv().ok();
    let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get,Method::Post,Method::Patch,Method::Put,Method::Options,Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("Failed building cors");
    let mut rc=rocket::build().manage(AppContext::default())
    .attach(cors);

    rc=rc.mount("/",routes![hello]);
    rc=user_routes(rc);
    rc=auth_routes(rc);
    rc= file_routes(rc);
    rc
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::{http::Status, local::asynchronous::Client};
    
   

    #[async_test]
    async fn hello_world()->Result<(),std::io::Error> {
        let client = Client::tracked(super::init_app()).await.expect("valid rocket instance");
        let  response = client.get("/").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let  body=response.into_string().await;
        
        
        assert_eq!(body, Some(String::from("Hello, world!")));
        Ok(())
    }
}
