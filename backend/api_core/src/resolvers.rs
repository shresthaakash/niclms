use std::collections::HashMap;
use apps::{app_repository, app_service::{AppService, app_service_factory}};
#[derive(Clone)]
pub struct AppId(String);

impl AppId {
   fn get_master_id()->String{
       return String::from("57370747-2fe1-4d6f-a6fc-93a3f951b89a");
   }
}

pub struct AppResolver {
    pub domain_app_map:HashMap<String,AppId>,
    pub app_service: AppService,

    
}

impl  AppResolver {
    pub  fn new()-> Self{
        AppResolver {
         domain_app_map:HashMap::new(),
         app_service: app_service_factory(),
        }
     }

     pub async fn resolve(&mut self,domain:&str)->Option<AppId>{

       let cached = self.domain_app_map.get(domain);
       if let None = cached {
           let app = self.app_service.get_by_domain(domain).await;
           if let Some(a) = app {
               for domain in a.domains {
                   self.domain_app_map.insert(domain, AppId(a._id.clone()));
               }
           }
           
       }
       self.domain_app_map.get(domain).map(|a|a.clone())
     
     }
}