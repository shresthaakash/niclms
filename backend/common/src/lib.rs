

pub mod entities;
pub mod crypto_helper;

extern crate couch_rs;
extern crate rand;
extern crate crypto;
extern crate hmac;
extern crate hex_literal;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


pub trait OwnerInfo{
    fn get_owner_id(&self)->couch_rs::Cow<str>;
    fn set_owner_id(&mut self,owner_id:&str);

}


pub fn get_code(limit:u8)->String {
    let code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(limit.into())
        .map(char::from)
        .collect();
    code.to_lowercase()
}
