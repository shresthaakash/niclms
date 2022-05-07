
extern crate serde_json;
mod test_helper;

use auth::account::{Credentials, SignupResult};
use rocket::http::ContentType;
use rocket::{self, async_test};
use rocket::{http::Status};
use test_helper::get_client;


#[async_test]
async fn hello_world()->Result<(),std::io::Error> {
    let client=get_client().await;
    let  response = client.get("/").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let  body=response.into_string().await;
    assert_eq!(body, Some(String::from("{version:1.0}")));
    Ok(())
}
#[async_test]
async fn signup(){
    let client=get_client().await;
    let cred=Credentials{
        mobile_no:"9861152662".to_string(),
        password:"somepassword".to_string()
    };
    let res=client.post("/auth/register")
    .header(ContentType::JSON)
    .body(serde_json::to_string(&cred).unwrap()).dispatch().await;
    let res_str=res.into_string().await;
    print!("{:?}",&res_str);
    assert_ne!(None,res_str);
}
#[async_test]
async fn verify_signup(){
    let client=get_client().await;
    let cred=Credentials{
        mobile_no:"9861152662".to_string(),
        password:"somepassword".to_string()
    };
    let res=client.post("/auth/register")
    .header(ContentType::JSON)
    .body(serde_json::to_string(&cred).unwrap()).dispatch().await;
    let res_str=res.into_string().await.unwrap();
    let signupresult=serde_json::from_str::<SignupResult>(&res_str).unwrap();
    let url=format!("/auth/verify_signup/{id}/{code}",id=signupresult.id,code=signupresult.code);
    let verify_res=client.get(url);
    print!("{:?}",&verify_res)
}

