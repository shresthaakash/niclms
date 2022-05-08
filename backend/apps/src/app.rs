use crate::framework::entity::Entity;
use common::OwnerInfo;
use couch_rs::document::TypedCouchDocument;
use couch_rs::types::document::DocumentId;
use couch_rs::CouchDocument;
use framework::entity::EntityUpdate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CouchDocument, DocOwnerInfo, EntityDoc, Debug)]
pub struct App {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    pub institute_id:String,
    pub domains:Vec<String>,
    pub created_on:u64,
    pub owner_id:String,
    pub entity_type:String,
}


#[derive(Serialize, Deserialize,Debug)]
pub struct AppUpdate{
    pub domains:Option<Vec<String>>,
    pub institute_id:Option<String>
}

impl EntityUpdate<App> for AppUpdate {
    fn apply_update<'c>(&self, stale:& 'c mut App) -> &'c mut App{
        if let Some(domain)=&self.domains{
            stale.domains=domain.clone();
        }
        if let Some(institute_id)=&self.institute_id{
            stale.institute_id=institute_id.clone()
        }

        stale
        
    }
}