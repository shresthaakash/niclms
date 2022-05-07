


use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use infra::s3::get_presigned_url;
use infra::s3::SignedUrlResponse;
use common::get_code;
#[get("/upload_url", format = "application/json",)]
pub async fn get_upload_url()->Json<SignedUrlResponse>{
    let bucket=std::env::var("S3_BUCKET").unwrap();
  return Json(get_presigned_url(bucket, get_code(16) ));
}


pub fn file_routes(build: Rocket<Build>) -> Rocket<Build> {
    let rc = build.mount("/files", routes![get_upload_url]);
    rc
}