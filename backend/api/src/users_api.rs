use api_core::app_context::AppContext;
use api_core::errors::ApiError;
use api_core::filters::user_filter;
use api_core::params::Params;
use auth::jwt_auth::LoginInfo;
use couch_rs::types::find::FindQuery;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};

use users::{ User,NewUser,UserUpdate};
use framework::service::IService;
use rocket::serde::json::Error as JsonError;



#[post("/", format = "application/json", data = "<data_result>")]
pub async fn create_user(login:LoginInfo,data_result: Result<Json<NewUser>,JsonError<'_>>, state: &State<AppContext>) -> Result<Json<User>, ApiError> {
    let data=data_result?;
    let  mut new_user: NewUser = data.into_inner();
    new_user.owner_id=Some(login.account_id.clone());
    new_user.account_id = match new_user.is_self{
        true =>Some(login.account_id),
        false=> None
    };
    let mut user: User = User::from(new_user);
    println!("{:?}",user);
    return state.user_service
        .create( &mut user)
        .await
        .map(|u| {
            println!("res");
            println!("{:?}",u);
            Json(u)
        })
        .map_err(|e| {
            println!("{:?}", e);
            ApiError::from(e)
        });
}



#[patch("/<id>", format = "application/json", data = "<data>")]
pub async fn update_user(
    _login:LoginInfo,
    data: Json<UserUpdate>,
    id: String,
    state: &State<AppContext>,
) -> Result<Json<User>, ApiError> {
    let user: UserUpdate = data.into_inner();
    return state.user_service
        .update( id, user)
        .await
        .map(|u| Json(u))
        .map_err(|e| {
            println!("{:?}", e);
            ApiError::from(e)
        });
}

#[delete("/<id>", format = "application/json")]
pub async fn delete_user(
    _login:LoginInfo,
    id: String,
    state: &State<AppContext>,
) -> Result<Json<bool>, ApiError> {

    return state.user_service
        .delete( id)
        .await
        .map(|u| Json(u))
        .map_err(|e| {
            println!("{:?}", e);
            ApiError::from(e)
        });
}

#[get("/?<params..>",format = "application/json")]
async fn list_users(login:LoginInfo, params: Params, state: &State<AppContext>) -> Result<Json<Vec<User>>, ApiError> {
    let mut query = FindQuery::from(params);
    query.selector=user_filter(login, query.selector);
    return state.user_service
        .get_all( query)
        .await
        .map(|docs| Json(docs)).map_err(|e|e.into());
}

#[get("/me",format = "application/json")]
async fn get_self(login:LoginInfo,state: &State<AppContext>) -> Result<Json<User>, ApiError> {
    return state.user_service
        .get_by_account( login.account_id)
        .await
        .map(|docs| Json(docs))
        .ok_or(ApiError::NotFound("Not Found".into()));
}

#[get("/<id>",format = "application/json")]
async fn get_user_by_id(_login:LoginInfo,id: String, state: &State<AppContext>) -> Result<Json<User>, ApiError> {
    return state.user_service
        .get_by_id(id)
        .await
        .map(|docs| Json(docs))
        .map_err(|e| e.into());
}



pub fn user_routes(build: Rocket<Build>) -> Rocket<Build> {
    let rc = build.mount("/users", routes![list_users, create_user, update_user,delete_user,get_user_by_id,get_self]);
    rc
}



