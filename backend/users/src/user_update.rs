use framework::entity::EntityUpdate;
use serde::Deserialize;

use crate::user::{Address, Gender, IdentityType, User};

#[derive(Deserialize)]
pub struct UserUpdate{
    pub email:Option<String>,
    pub full_name:Option<String>,
    pub age:Option<u8>,
    pub temporary_address:Option<Address>,
    pub permanent_address:Option<Address>,
    pub gender:Option<Gender>,
    pub identity_type:Option<IdentityType>,
    pub identity_id:Option<String>,
    pub issue_date:Option<i64>,
    pub id_images:Option<Vec<String>>,
    pub mobile_no:Option<String>,
    pub dob:Option<i64>
}

impl EntityUpdate<User> for UserUpdate{
    fn apply_update<'c>(&self,stale:&'c mut User)-> & 'c mut User {
        if let Some(email)=&self.email{
            stale.email=Some(email.clone());
        }
        if let Some(full_name)=&self.full_name{
            stale.full_name=full_name.to_string();
        }
       
        if let Some(address)=&self.temporary_address{
            stale.address=Some((*address).clone());
        }
        
        if let Some(gender)=&self.gender{
            stale.gender=(*gender).clone();
        }
        if let Some(identity_type)=&self.identity_type{
            stale.identity_type=identity_type.clone();
        }
        if let Some(issue_date)=&self.issue_date{
            stale.issue_date=issue_date.clone();
        }
        if let Some(id_images)=&self.id_images{
            stale.id_images=Some(id_images.clone());
        }
        if let Some(mobile_no)=&self.mobile_no{
            stale.mobile_no=Some(mobile_no.clone());
        }
        if let Some(dob)=&self.dob{
            stale.dob=dob.clone();
        }
        stale
    }
}
