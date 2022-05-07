use api_core::params::Params;
use auth::{account::LoginResult, jwt_auth::LoginInfo};
use framework::service::IService;
use couch_rs::types::find::FindQuery;
use api_core::{app_context::AppContext, errors::ApiError};
use auth::{account::{Account, Credentials, ForgotPassResult, ModifyPassword, ResetPassword, SignupResult}};
use rocket::{Build, Rocket, State, serde::json::Json,};
use rocket::serde::json::Error as JsonError;


#[post("/register", format = "application/json", data = "<data_result>")]
pub async fn signup(data_result: Result<Json<Credentials>,JsonError<'_>>, app: &State<AppContext>) -> Result<Json<SignupResult>, ApiError> {
    let data=data_result?;
    let  new_user: Credentials = data.into_inner();
    let account_service=&app.account_service;
    let res = account_service
        .signup(new_user)
        .await
        .map(|u| Json(u))
        .map_err(|e| {
            println!("{:?}", e);
            ApiError::from(e)
        });
    res

}

#[post("/login", format = "application/json", data = "<data>")]
pub async fn login(data: Json<Credentials>, app: &State<AppContext>) -> Result<Json<LoginResult>, ApiError> {
    let new_user: Credentials = data.into_inner();
    let account_service=&app.account_service;
    let res = account_service
        .login(new_user)
        .await
        .map(|u| Json(u))
        .map_err(|e| {
            println!("{:?}", e);
            ApiError::from(e)
        });
    res

}


#[get("/verify_signup/<id>/<code>", format = "application/json")]
pub async fn verify_signup(id:String,code:String, app: &State<AppContext>) -> Result<Json<LoginResult>, ApiError> {
    let account_service=&app.account_service;
    let res = account_service
        .verify_signup(id,code)
        .await
        .map(|u| Json(u))
        .map_err(|e| {
            println!("{:?}", e);
            ApiError::from(e)
        });
    res

}

#[get("/?<params..>",format = "application/json")]
async fn list_accounts(_login:LoginInfo,params: Params, state: &State<AppContext>) -> Result<Json<Vec<Account>>, ApiError> {
    let svc = &state.account_service;
    print!("{:?}", &params);
    let query = FindQuery::from(params);
    print!("{:?}", &query);

    return svc
        .get_all( query)
        .await
        .map(|docs| Json(docs))
        .map_err(|e| e.into());
}


#[post("/reset_password", format = "application/json", data = "<data>")]
pub async fn reset_password(data: Json<ResetPassword>, app: &State<AppContext>) -> Result<Json<Account>, ApiError> {
    let payload: ResetPassword = data.into_inner();
    let account_service=&app.account_service;
    let res = account_service
        .reset_password(payload)
        .await
        .map(|u| Json(u))
        .map_err(|e| {
            println!("{:?}", e);
            ApiError::from(e)
        });
    res

}

#[get("/signup/resend_code/<id>", format = "application/json")]
pub async fn resend_signup_code(id: String, app: &State<AppContext>) -> Result<Json<SignupResult>, ApiError> {
    let account_service=&app.account_service;
    let res = account_service
        .resend_code(id)
        .await
        .map(|u| Json(u))
        .map_err(|e| {
            println!("{:?}", e);
            ApiError::from(e)
        });
    res

}

#[post("/modify_password", format = "application/json", data = "<data>")]
pub async fn modify_password(login:LoginInfo,data: Json<ModifyPassword>, app: &State<AppContext>) -> Result<Json<Account>, ApiError> {
    let mut payload: ModifyPassword = data.into_inner();
    payload.account_id=login.account_id;
    let account_service=&app.account_service;
    let res = account_service
        .modify_password(payload)
        .await
        .map(|u| Json(u))
        .map_err(|e| {
            println!("{:?}", e);
            ApiError::from(e)
        });
    res

}

#[get("/forgot_password/<mobile_no>",format = "application/json")]
pub async fn forgot_password(mobile_no:String,state: &State<AppContext>) -> Result<Json<ForgotPassResult>, ApiError> {
    let svc = &state.account_service;

    return svc
        .forgot_password(mobile_no)
        .await
        .map(|r|Json(r))
        .map_err(|e| ApiError::from(e));
}




pub fn auth_routes(build: Rocket<Build>) -> Rocket<Build> {
    let rc = build.mount("/auth", 
    routes![
        signup,
        login,
        verify_signup,
        list_accounts,
        reset_password,
        forgot_password,
        modify_password,
        resend_signup_code
        ]

);
    rc
}