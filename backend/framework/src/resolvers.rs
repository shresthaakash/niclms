use async_trait::async_trait;
#[async_trait]
pub trait IResolver<K,V>:Send +Sync {
    async fn resolve(&mut self,domain:K)->Option<V>;
}