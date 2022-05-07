use crate::framework::entity::Entity;
use common::OwnerInfo;
use couch_rs::document::TypedCouchDocument;
use couch_rs::types::document::DocumentId;
use couch_rs::CouchDocument;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IdentityType {
    Citizenship,
    License,
    Passport,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub district: String,
    pub municipality_vdc: String,
    pub ward: String,
    pub tole: String,
}

#[derive(Serialize, Deserialize, CouchDocument, DocOwnerInfo, EntityDoc, Debug)]
pub struct User {
    pub email: Option<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,

    pub owner_id: String,
    pub account_id:Option<String>,
    pub entity_type: String,
    pub full_name: String,
    pub address: Option<Address>,
    pub nationality: String,
    pub id_images: Option<Vec<String>>,
    pub gender: Gender,
    pub identity_type: IdentityType,
    pub identity_id: String,
    pub issue_date: i64,
    pub dob: i64,
    pub mobile_no:Option<String>
}
