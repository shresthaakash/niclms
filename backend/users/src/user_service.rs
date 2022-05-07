use common::entities::USERS;
use couch_rs::error::CouchError;
use framework::{repository::{IRepository, Repository}, service::IService};
use serde_json::{Map, Value, map::Values};

use super::{user::User, user_repository::UserRepository};
use crate::user_update::UserUpdate;

pub struct UserService {
    pub repo: Repository<User, UserUpdate>,
}
unsafe impl Send for UserService {}

impl IService<User, UserUpdate> for UserService {
    fn repo(&self) -> &Repository<User, UserUpdate> {
        &self.repo
    }
}

impl UserService {
    fn new(repo: UserRepository) -> Self {
        UserService { repo }
    }

    pub async fn get_by_account(&self,account_id:String)->Option<User>{
        let mut map=Map::new();
        map.insert("account_id".into(), Value::String(account_id));
        return self.repo().find_one(Value::Object(map)).await.ok().flatten();
    }
}

pub fn user_service_factory() -> UserService {
    let repo = UserRepository::new(USERS.to_string());
    return UserService::new(repo);
}
