use super::{
    entity::{Entity, EntityUpdate},
    repository::Repository,
};
use crate::repository::IRepository;
use crate::repository::RepoError;
use async_trait::async_trait;
use couch_rs::{
    types::{document::DocumentId, find::FindQuery},
};
use std::error::Error;

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
}

unsafe impl<T: Entity, U: EntityUpdate<T>> Sync for Service<T, U> {}

impl<T: Entity, U: EntityUpdate<T>> Service<T, U> {
    pub fn new(repo: Repository<T, U>) -> Self {
        Service { repo }
    }
}

#[async_trait]
pub trait IService<T: Entity + 'static, U: EntityUpdate<T> + 'static> {
    fn repo(&self) -> &Repository<T, U>;

    async fn get_all(&self, query: FindQuery) -> Result<Vec<T>, ServiceError> {
        let repo = self.repo();
        let res = repo.find_all(query).await?;
        return Ok(res);
    }

    async fn create(&self, item: &mut T) -> Result<T, ServiceError> {
        let repo = self.repo();
        let res = repo.create(item).await?;
        Ok(res)
    }
    async fn update(&self, doc_id: DocumentId, item: U) -> Result<T, ServiceError> {
        let repo = self.repo();
        let res = repo.update(doc_id, item).await?;
        Ok(res)
    }

    async fn delete(&self, doc_id: DocumentId) -> Result<bool, ServiceError> {
        let repo = self.repo();
        let res = repo.delete(doc_id).await?;
        Ok(res)
    }

    async fn get_by_id(&self, doc_id: DocumentId) -> Result<T, ServiceError> {
        let repo = self.repo();
        let res = repo.get_by_id(doc_id).await?;
        Ok(res)
    }
}
