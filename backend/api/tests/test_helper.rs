use rocket::{ local::asynchronous::Client};
use api::init_app;

pub async fn get_client()->Client{
    let client:rocket::local::asynchronous::Client = Client::tracked(init_app())
    .await.expect("valid rocket instance");
    return client;

}
