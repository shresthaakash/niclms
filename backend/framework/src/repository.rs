use std::marker::PhantomData;

use super::entity::{Entity, EntityUpdate};
use crate::error_msgs::{
    FAILED_TO_CONNECT, FORBIDDEN_ACCESS, INVALID_OPERATION, RESOURCE_CONFLICT, RESOURCE_NOT_FOUND,
    UNAUTHORIZED_ACCESS, UNKNOWN_ERROR,
};
use async_trait::async_trait;
use couch_rs::{
    database::Database,
    error::CouchError,
    types::{document::DocumentId, find::FindQuery},
};
use rocket::http::hyper::StatusCode;
use serde_json::{Map, Value};
use std::error::Error;

#[derive(Debug)]
pub enum RepoError {
    EntityExists(Box<dyn Error + Send + Sync>),
    EntityNotFound(Box<dyn Error + Send + Sync>),
    SaveFailed(Box<dyn Error + Send + Sync>),
    UpdateFailed(Box<dyn Error + Send + Sync>),
    DeleteFailed(Box<dyn Error + Send + Sync>),
    FailedToConnect(Box<dyn Error + Send + Sync>),
    InvalidOperation(Box<dyn Error + Send + Sync>),
    ForbiddenAction(Box<dyn Error + Send + Sync>),
    UnknownError(Box<dyn Error + Send + Sync>),
}

impl From<CouchError> for RepoError {
    fn from(ce: CouchError) -> Self {
        let status = ce.status;
        match status {
            StatusCode::FORBIDDEN => RepoError::ForbiddenAction(FORBIDDEN_ACCESS.into()),
            StatusCode::NOT_FOUND => RepoError::EntityNotFound(RESOURCE_NOT_FOUND.into()),
            StatusCode::BAD_REQUEST => RepoError::InvalidOperation(INVALID_OPERATION.into()),
            StatusCode::CONFLICT => RepoError::EntityExists(RESOURCE_CONFLICT.into()),
            StatusCode::UNAUTHORIZED => RepoError::FailedToConnect(UNAUTHORIZED_ACCESS.into()),
            _ => RepoError::UnknownError(UNKNOWN_ERROR.into()),
        }
    }
}

pub struct Repository<T: Entity, U: EntityUpdate<T>> {
    pub collection: String,
    pub phantom: PhantomData<T>,
    pub phantomu: PhantomData<U>,
}

impl<T: Entity, U: EntityUpdate<T>> Repository<T, U> {
    pub fn new(collection: String) -> Self {
        Repository {
            collection,
            phantom: PhantomData::default(),
            phantomu: PhantomData::default(),
        }
    }
}

unsafe impl<T: Entity, U: EntityUpdate<T>> Sync for Repository<T, U> {}

#[async_trait]
pub trait IRepository: Sync + Send {
    type EntityType: Entity;
    type Update: std::marker::Send + std::marker::Sync + EntityUpdate<Self::EntityType>;

    async fn db(&self) -> Result<Database, CouchError>;

    fn collection(&self) -> String;

    async fn find_all(&self, query: FindQuery) -> Result<Vec<Self::EntityType>, RepoError> {
        let db = self.db().await?;

        let mut q = query;
        q.selector = self.set_entity_selector(q.selector);
        let results = db.find::<Self::EntityType>(&q).await?;
        Ok(results.rows)
    }

    async fn find_one(&self, query: Value) -> Result<Option<Self::EntityType>, RepoError> {
        println!("{:?}", &query);
        let mut q = FindQuery::new(query).limit(1).skip(0);
        q.selector = self.set_entity_selector(q.selector);
        let db = self.db().await?;
        let results = db
            .find::<Self::EntityType>(&q)
            .await
            .map(|r| r.rows.into_iter().nth(0))?;
        Ok(results)
    }

    async fn create(&self, item: &mut Self::EntityType) -> Result<Self::EntityType, RepoError> {
        let db = self.db().await?;
        item.set_entity_type(&self.collection());
        let saved = db.create(item).await?;
        let i = db.get::<Self::EntityType>(&saved.id).await?;
        Ok(i)
    }

    async fn update(
        &self,
        doc_id: DocumentId,
        update: Self::Update,
    ) -> Result<Self::EntityType, RepoError> {
        let db = self.db().await?;

        let mut updated = db
            .get::<Self::EntityType>(&doc_id)
            .await
            .map(|mut db_doc| {
                update.apply_update(&mut db_doc);
                db_doc
            })?;

        let saved = db
            .upsert(&mut updated)
            .await
            .map(|_s| db.get::<Self::EntityType>(&doc_id))?
            .await?;

        Ok(saved)
    }

    async fn delete_where(&self, query: Value) -> Result<i32, RepoError> {
        let q = FindQuery::new(query).limit(1000000).skip(0);
        let db = self.db().await?;
        let docs = self.find_all(q).await?;
        let mut deleted = 0;
        for doc in docs {
            db.remove(&doc).await;
            deleted = deleted + 1;
        }
        Ok(deleted)
    }

    async fn update_where(
        &self,
        query: Value,
        update: Self::Update,
    ) -> Result<Vec<Self::EntityType>, RepoError> {
        let q = FindQuery::new(query).limit(1000000).skip(0);
        let db = self.db().await?;
        let docs = self.find_all(q).await?;
        let mut results = Vec::<Self::EntityType>::new();
        for mut doc in docs {
            let updated = update.apply_update(&mut doc);
            let res = db.save(updated).await?;
            let saved = db.get(&res.id).await?;
            results.push(saved)
        }
        Ok(results)
    }

    async fn delete(&self, doc_id: DocumentId) -> Result<bool, RepoError> {
        let db = self.db().await?;
        let saved = db.get::<Self::EntityType>(&doc_id).await?;
        let res = db.remove(&saved).await;
        Ok(res)
    }

    async fn get_by_id(&self, doc_id: DocumentId) -> Result<Self::EntityType, RepoError> {
        let db = self.db().await?;
        let i = db.get::<Self::EntityType>(&doc_id).await?;
        Ok(i)
    }

    fn set_entity_selector(&self, selector: Value) -> Value {
        let entity_type = self.collection();

        let mut map = match selector {
            Value::Object(map) => map,
            _ => {
                let m = Map::new();
                m
            }
        };
        map.insert("entity_type".to_string(), Value::String(entity_type));
        return Value::Object(map);
    }
}

#[async_trait]
impl<T: Entity, U: EntityUpdate<T>> IRepository for Repository<T, U> {
    type EntityType = T;
    type Update = U;

    async fn db(&self) -> Result<Database, CouchError> {
        let host = std::env::var("DB_URL").unwrap();
        let user = std::env::var("DB_USERNAME").unwrap();
        let password = std::env::var("DB_PASSWORD").unwrap();
        let dbname = std::env::var("DB_NAME").unwrap();
        let client = couch_rs::Client::new(&host, &user, &password)?;
        let db = client.db(&dbname).await?;
        Ok(db)
    }

    fn collection(&self) -> String {
        return self.collection.clone();
    }
}
