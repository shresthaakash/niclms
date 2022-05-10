use std::sync::{Arc};

use async_trait::async_trait;
use common::entities::APPS;
use couch_rs::{error::CouchError, types::find::FindQuery};
use framework::{repository::{IRepository, Repository}, service::{IService, ServiceError}, resolvers::IResolver};
use serde_json::{Map, Value, map::Values, json};
use tokio::sync::Mutex;

use super::{app::App,app::AppUpdate, app_repository::AppRepository};

pub struct MasterResolver{}

#[async_trait]
impl IResolver<String,String> for MasterResolver {
    async fn resolve(&mut self,domain:String)->Option<String> {
        return Some("master_db".to_owned());
    }
}


pub struct AppService {
    pub repo: Repository<App, AppUpdate>,
    pub db_resolver: Arc<Mutex<dyn IResolver<String,String>>>,
}
unsafe impl Send for AppService {}

impl IService<App, AppUpdate> for AppService {
    fn repo(&self) -> &Repository<App, AppUpdate> {
        &self.repo
    }

    fn get_resolver(&self) -> &Arc<Mutex<dyn IResolver<String,String>>> {
        &self.db_resolver
    }
}

impl AppService {
    pub fn new(repo: AppRepository,resolver:Arc<Mutex<dyn IResolver<String,String>>>) -> Self {
        AppService { repo ,db_resolver:resolver}
    }

    pub async  fn  get_by_domain(&self,domain:&str)->Option<App>{
        let query = json!({
            "domains":{
                "$elemMatch":{
                    "$eq":domain
                }
            }
        });
        let find= FindQuery::from(query);
        let apps = self.get_all("master_id".to_owned(),find).await.ok();
        return apps.and_then(| mut a|{
            if a.len()>0{
                if let Some(l)=a.pop(){
                     return Some(l);
                }
            }
            None
        })

    }

}

pub fn app_service_factory() -> AppService {
    let repo = AppRepository::new(APPS.to_string());
    return AppService::new(repo,Arc::new(Mutex::new(MasterResolver{})));
}
