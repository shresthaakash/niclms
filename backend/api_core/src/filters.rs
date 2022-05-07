use auth::{account::Role, jwt_auth::LoginInfo};
use serde_json::{Map, Value};

pub fn user_filter(login:LoginInfo,selector:Value)->Value{
    let mut query:Map<String,Value>=match selector {
        Value::Object(m)=> m,
        _ => Map::new()
    };
    match login.role {
        Role::Admin=>Value::Object(query),
        Role::User=>{
            query.insert("owner_id".into(), Value::String(login.account_id.clone()));
            Value::Object(query)
        }
    }
    
}