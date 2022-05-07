use std::{ collections::HashMap,fmt::{Display, Formatter}};

use couch_rs::types::find::FindQuery;
use rocket::form::{ FromForm, FromFormField, ValueField, prelude::ErrorKind};
use serde::Deserialize;
use serde_json::Value;
#[derive(Debug,Deserialize,Clone,Serialize)]
pub  struct QueryValue(Value);

impl From<QueryValue> for Value{
    fn from(v: QueryValue) -> Self {
        let QueryValue(json_val)=v;
        json_val
    }
}



#[rocket::async_trait]
impl<'r> FromFormField<'r> for QueryValue {
    fn from_value(field: ValueField<'r>) -> rocket::form::Result<'r, Self> {
        serde_json::from_str(field.value)
        .map(|v|QueryValue(v))
        .map_err(|e|{
            println!("{:?}",&e);
            let mut errors=rocket::form::Errors::new();
            errors.push(ErrorKind::Unexpected.into());
            errors
            
        })
    }
}





#[derive(Deserialize,Debug,FromForm)]
pub struct Params{
    pub query:HashMap<String,QueryValue>,
    #[field(default = 1)]
    pub page:u32,
    #[field(default = 10)]
    pub limit:u32,

}

impl Display for Params {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
         let map=self.query.clone();
         for (key, value) in map.into_iter() {
             write!(f,"key{:?} : value:{:?}\n", key,Value::from(value)).unwrap();

         }
        write!(f, "limit: {}, page: {}", self.limit, self.page)
    }
}

impl From<Params> for FindQuery{
    fn from(p: Params) -> Self {
        let mut q=FindQuery::find_all();
        q=q.limit(p.limit.into());
        q=q.skip(((p.page-1)*p.limit).into());
        let mut jsonmap= serde_json::map::Map::new();
        for(key,qvalue) in p.query.into_iter(){
            jsonmap.insert(key, Value::from(qvalue));
        }
        q.selector=serde_json::Value::Object(jsonmap);
        q
    }
}