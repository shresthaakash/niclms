use async_trait::async_trait;
#[async_trait]
pub trait IStoreResolver:Send +Sync {
    async fn resolve_store_by_domain(&mut self,domain:String)->Option<String>;
    async fn resolve_db_by_store(&mut self,store_id:String)->Option<String>;
}