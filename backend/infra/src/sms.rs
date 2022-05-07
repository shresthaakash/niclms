use reqwest::Response;


pub async fn send_code(number:String,code:String)->Result<Response,reqwest::Error>{
    let url=std::env::var("SMS_PROXY").unwrap();
    let request=format!("{}/?to={}&text={}&hash=somehash",url,number,code);
    println!("{}",request);
    let res=reqwest::get(request).await?;
    return Ok(res);

}

pub async fn call_sms_service(number:String,code:String)->Result<Response,reqwest::Error>{
    let url=std::env::var("SMS_URL").unwrap();
    let token=std::env::var("SMS_API_KEY").unwrap();
    let request=format!("{}/?token={}&from=InfoSMS&to={}&text={}",url,token,number,code);
    println!("{}",request);
    let res=reqwest::get(request).await?;
    return Ok(res);

}