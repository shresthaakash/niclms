use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct SMSPayload {
    pub phone: String,
    pub message: String,
    
}
