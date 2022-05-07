use super::user::User;
use super::user_update::UserUpdate;
use framework::repository::Repository;

pub type UserRepository=Repository<User,UserUpdate>;
