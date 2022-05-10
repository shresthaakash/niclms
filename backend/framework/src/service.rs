use super::{
    entity::{Entity, EntityUpdate},
    repository::Repository,
};
use crate::{repository::IRepository, resolvers::IResolver};
use crate::repository::RepoError;
use async_trait::async_trait;
use couch_rs::{
    types::{document::DocumentId, find::FindQuery},
};
use rocket::tokio::sync::Mutex;
use std::{error::Error, sync::Arc};

#[derive(Debug)]
pub enum ServiceError {
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

impl From<RepoError> for ServiceError {
    fn from(re: RepoError) -> Self {
        match re {
            RepoError::EntityExists(e) => ServiceError::EntityExists(e),
            RepoError::EntityNotFound(e) => ServiceError::EntityNotFound(e),
            RepoError::SaveFailed(e) => ServiceError::SaveFailed(e),
            RepoError::UpdateFailed(e) => ServiceError::UpdateFailed(e),
            RepoError::DeleteFailed(e) => ServiceError::DeleteFailed(e),
            RepoError::FailedToConnect(e) => ServiceError::FailedToConnect(e),
            RepoError::InvalidOperation(e) => ServiceError::InvalidOperation(e),
            RepoError::ForbiddenAction(e) => ServiceError::ForbiddenAction(e),
            RepoError::UnknownError(e) => ServiceError::UnknownError(e),
        }
    }
}

pub struct Service<T: Entity, U: EntityUpdate<T>> {
    pub repo: Repository<T, U>,
    pub resolver: Arc<Mutex<dyn IResolver<String,String>>>,
}

unsafe impl<T: Entity, U: EntityUpdate<T>> Sync for Service<T, U> {}

impl<T: Entity, U: EntityUpdate<T>> Service<T, U> {
    pub fn new(repo: Repository<T, U>,resolver: Arc<Mutex<dyn IResolver<String,String>>>) -> Self {
        Service { repo,resolver }
    }
}

#[async_trait]
pub trait IService<T: Entity + 'static, U: EntityUpdate<T> + 'static> {
    fn repo(&self) -> &Repository<T, U>;
    fn get_resolver(&self) -> &Arc<Mutex<dyn IResolver<String,String>>>;

    async fn get_all(&self, app_id: String,query: FindQuery) -> Result<Vec<T>, ServiceError> {
        let repo = self.repo();
        let db_name = self.db_name(app_id).await;
        let res = repo.find_all(db_name,query).await?;
        return Ok(res);
    }

    async fn create(&self, app_id: String,item: &mut T) -> Result<T, ServiceError> {
        let repo = self.repo();
        let db_name = self.db_name(app_id).await;
        let res = repo.create(db_name,item).await?;
        Ok(res)
    }
    async fn update(&self, app_id: String,doc_id: DocumentId, item: U) -> Result<T, ServiceError> {
        let repo = self.repo();
        let db_name = self.db_name(app_id).await;
        let res = repo.update(db_name,doc_id, item).await?;
        Ok(res)
    }

    async fn delete(&self, app_id: String,doc_id: DocumentId) -> Result<bool, ServiceError> {
        let repo = self.repo();
        let db_name = self.db_name(app_id).await;
        let res = repo.delete(db_name,doc_id).await?;
        Ok(res)
    }

    async fn get_by_id(&self, app_id: String,doc_id: DocumentId) -> Result<T, ServiceError> {
        let repo = self.repo();
        let db_name = self.db_name(app_id).await;
        let res = repo.get_by_id(db_name,doc_id).await?;
        Ok(res)
    }

    async fn db_name(&self, app_id: String) -> String {
        let mut resolver = self.get_resolver().lock().await;
        let db = resolver.resolve(app_id).await.unwrap();
        db
    }
}
