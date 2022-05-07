use bcrypt::{DEFAULT_COST, hash};
use chrono::Utc;
use common::entities::{ACCOUNTS, SIGNUPS};
use couch_rs::document::TypedCouchDocument;
use couch_rs::types::document::DocumentId;
use couch_rs::CouchDocument;
use serde::{Deserialize, Serialize};

use common::get_code;


use crate::framework::entity::{Entity, EntityUpdate};
#[derive(Serialize, Deserialize,Debug,Clone)]
pub enum Role {
    Admin,
    User
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub enum AccountType {
    Member,
    SilverMember,
    GoldMember,
    PlatinumMember,
}

impl AccountType {
    fn member() -> Self { AccountType::Member }
}




#[derive(Serialize, Deserialize, CouchDocument,EntityDoc, Debug,Clone)]
pub struct Account {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    /// Document Revision, provided by CouchDB, helps negotiating conflicts
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    pub mobile_no:String,
    pub role:Role,
    pub password:String,
    pub reset_token:Option<String>,
    pub entity_type:String,
    #[serde(default = "AccountType::member")]
    pub account_type:AccountType
}
#[derive(Serialize, Deserialize,Debug)]
pub enum SomeType {
    TypeA,
    TypeB,
    TypeC
}



#[derive(Serialize, Deserialize,Debug)]
pub struct Credentials {
    pub mobile_no:String,
    pub password:String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct SignupResult {
    pub id:String,
    pub code:String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ForgotPassResult {
    pub mobile_no:String,
    pub code:String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct LoginResult {
    pub account:Account,
    pub access_token:String,
    pub ttl_ms:i64,
}




#[derive(Serialize, Deserialize, CouchDocument,EntityDoc, Debug)]
pub struct SignUp {
    pub credentials:Credentials,
    pub activation_code:String,
    pub last_sent_on:i64,
    pub entity_type:String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    /// Document Revision, provided by CouchDB, helps negotiating conflicts
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,

}

impl From<Credentials> for SignUp {
    fn from(cred: Credentials) -> Self {
        let code=get_code(5);
        print!("{:?}",&code);
        SignUp{
            credentials:cred,
            activation_code:code,
            last_sent_on:Utc::now().timestamp_millis(),
            entity_type:SIGNUPS.to_string(),
            _id:uuid::Uuid::new_v4().to_string(),
            _rev:"".to_string()
        }
    }
}

pub struct SignUpUpdate {
    pub activation_code:Option<String>,
    pub last_sent_on:Option<i64>
}

impl EntityUpdate<SignUp> for SignUpUpdate {
    fn apply_update<'c>(&self,stale:& 'c mut SignUp)-> &'c mut SignUp {
        if let Some(code)=&self.activation_code{
            stale.activation_code=code.to_string();
        }
        stale
    }
}

#[derive(Serialize, Deserialize)]
pub struct ModifyPassword {
    pub account_id:DocumentId,
    pub newpassword:String,
    pub oldpassword:String,
}


#[derive(Serialize, Deserialize)]
pub struct ResetPassword {
    pub reset_token:String,
    pub newpassword:String,
}


#[derive(Serialize, Deserialize)]
pub struct AccountUpdate {
    pub password:Option<String>,
    pub role:Option<Role>,
    pub reset_token:Option<String>,
}

impl EntityUpdate<Account> for AccountUpdate{
    fn apply_update<'c>(&self,stale:&'c mut Account)-> & 'c mut Account {
        if let Some(password)=&self.password{
            let hashed_pwd = hash(&password, DEFAULT_COST).unwrap();
            stale.password=hashed_pwd;
        }
        if let Some(role)=&self.role{
            stale.role=role.clone();
        }
        if let Some(token)=&self.reset_token{
            stale.reset_token=Some(token.to_string());
        }
        stale
    }
}

impl Default for AccountUpdate{
    fn default() -> Self {
        Self { password: None, role: None, reset_token: None }
    }
}



impl From<SignUp> for Account {
    fn from(na: SignUp) -> Self {
        let hashed_pwd = hash(&na.credentials.password, DEFAULT_COST).unwrap();
        Account {
            mobile_no:na.credentials.mobile_no,
            password:hashed_pwd,
            _id:uuid::Uuid::new_v4().to_string(),
            _rev:"".into(),
            reset_token:None,
            role:Role::User,
            entity_type:ACCOUNTS.to_string(),
            account_type:AccountType::Member,

        }
    }
}
