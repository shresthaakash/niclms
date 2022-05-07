use sha2::Sha256;
use hmac::{Hmac, Mac};
use hex_literal::hex;

type HmacSha256 = Hmac<Sha256>;
pub fn gen_hash(value:String){
    let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
    .expect("HMAC can take key of any size");
    mac.update(value.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
}