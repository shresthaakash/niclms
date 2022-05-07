use couch_rs::document::TypedCouchDocument;

pub trait Entity:TypedCouchDocument+std::marker::Send+std::marker::Sync{
    fn get_entity_type(&self)->String;
    fn set_entity_type(&mut self,entity_type:&str);
    
}

pub trait EntityUpdate<T:Entity>:std::marker::Send+std::marker::Sync {
    fn apply_update<'c>(&self,stale:& 'c mut T)-> &'c mut T;
}