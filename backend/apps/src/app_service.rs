use common::entities::APPS;
use couch_rs::{error::CouchError, types::find::FindQuery};
use framework::{repository::{IRepository, Repository}, service::{IService, ServiceError}};
use serde_json::{Map, Value, map::Values, json};

use super::{app::App,app::AppUpdate, app_repository::AppRepository};

pub struct AppService {
    pub repo: Repository<App, AppUpdate>,
}
unsafe impl Send for AppService {}

impl IService<App, AppUpdate> for AppService {
    fn repo(&self) -> &Repository<App, AppUpdate> {
        &self.repo
    }
}

impl AppService {
    pub fn new(repo: AppRepository) -> Self {
        AppService { repo }
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
        let apps = self.get_all(find).await.ok();
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
    return AppService::new(repo);
}
