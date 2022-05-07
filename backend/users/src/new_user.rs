use common::entities::USERS;
use serde::Deserialize;

use crate::user::{Address, Gender, IdentityType, User};

#[derive(Deserialize)]
pub struct NewUser{
    pub id:Option<String>,
    pub email:Option<String>,
    pub full_name:String,
    pub address:Option<Address>,
    pub id_images:Option<Vec<String>>,
    pub account_id:Option<String>,
    pub owner_id:Option<String>,
    pub is_self:bool,
    pub gender:Gender,
    pub identity_type:IdentityType,
    pub mobile_no:Option<String>,
    pub identity_id:String,
    pub issue_date:i64,
    pub dob:i64,

}




impl From<NewUser> for User{
    fn from(user: NewUser) -> Self {
        User{
            _id:user.id.unwrap_or(uuid::Uuid::new_v4().to_string()),
            _rev:"".into(),
            email:user.email,
            owner_id:user.owner_id.unwrap(),
            account_id:user.account_id,
            entity_type:USERS.into(),
            full_name:user.full_name,
            address:user.address,
            id_images:user.id_images,
            gender:user.gender,
            identity_type:user.identity_type,
            identity_id:user.identity_id,
            nationality:"Nepali".into(),
            mobile_no:user.mobile_no,
            issue_date:user.issue_date,
            dob:user.dob


        }
    }
}
