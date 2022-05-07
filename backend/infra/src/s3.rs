use rusoto_credential::AwsCredentials;
use rusoto_core::Region;
use rusoto_s3::PutObjectRequest;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use serde::{Deserialize, Serialize};


#[derive(Serialize , Deserialize, Debug)]
pub struct SignedUrlResponse {
    pub url: String,
    pub key:String,
}


pub  fn get_presigned_url(bucket:String,key:String)->SignedUrlResponse{
    let access_key=std::env::var("ACCESS_KEY").unwrap();
    let access_secret=std::env::var("ACCESS_SECRET").unwrap();
    let credentials=AwsCredentials::new(access_key,access_secret, None, None);
    let region=Region::EuCentral1;
    let options = PreSignedRequestOption {
        expires_in: std::time::Duration::from_secs(300),
        ..Default::default()
    };
    let req = PutObjectRequest {
        bucket: bucket,
        
        key: key.clone(),
        ..Default::default()
    };
    let url = req.get_presigned_url(&region, &credentials, &options);
    SignedUrlResponse{
        url:url,
        key:key,
    }
}