use bcrypt::BcryptError;
use chrono::Utc;
use common::entities::{ACCOUNTS, SIGNUPS};
use couch_rs::error::CouchError;
use framework::{
    repository::{IRepository, Repository},
    service::IService,
    service::ServiceError,
    repository::RepoError
};
use serde_json::{Map, Value};
use std::error;
use common::get_code;

use super::{
    account::{Account, AccountUpdate},
    account_repository::AccountRepository,
};
use crate::{account::{Credentials, ForgotPassResult, LoginResult, ModifyPassword, ResetPassword, SignUp, SignUpUpdate, SignupResult}, account_repository::SignUpRepository, jwt_auth::generate_token};
use infra::sms::send_code;

static ONE_WEEK: i64 = 1000*60 * 60 * 24 * 7; // in milli seconds
pub struct AccountService {
    pub repo: Repository<Account, AccountUpdate>,
    pub signup_repo: Repository<SignUp, SignUpUpdate>,
}
unsafe impl Send for AccountService {}

impl IService<Account, AccountUpdate> for AccountService {
    fn repo(&self) -> &Repository<Account, AccountUpdate> {
        &self.repo
    }
}
#[derive(Debug)]
pub enum AuthError {
    ServiceError(ServiceError),
    AccountExists(Box<dyn error::Error + Send + Sync>),
    AccountNotFound(Box<dyn error::Error + Send + Sync>),
    InvalidLogin(Box<dyn error::Error + Send + Sync>),
    RsendCodeTooSoon(Box<dyn error::Error + Send + Sync>),
    SMSFailed(Box<dyn error::Error + Send + Sync>)
}

impl From<reqwest::Error> for AuthError{
    fn from(_: reqwest::Error) -> Self {
        AuthError::SMSFailed("Failed To Send SMS".into())
    }
}

impl From<RepoError> for AuthError {
    fn from(ce: RepoError) -> Self {
       let se=ServiceError::from(ce);
       AuthError::ServiceError(se)
    }
}

impl From<ServiceError> for AuthError {
    fn from(ce: ServiceError) -> Self {
       AuthError::ServiceError(ce)
    }
}

impl From<BcryptError> for AuthError {
    fn from(_: BcryptError) -> Self {
        AuthError::InvalidLogin("Invalid Credentials".into())
    }
}

impl AccountService {
    fn new(repo: AccountRepository) -> Self {
        AccountService {
            repo,
            signup_repo: SignUpRepository::new(SIGNUPS.to_string()),
        }
    }

    fn signup_repo(&self) -> &SignUpRepository {
        return &self.signup_repo;
    }
    pub async fn signup(&self, new_account: Credentials) -> Result<SignupResult, AuthError> {
        let mobile_no=new_account.mobile_no.clone();
        let mut signup = SignUp::from(new_account);
        let res = self.signup_repo().create(&mut signup).await?;
        send_code(mobile_no,signup.activation_code.clone()).await?;
        Ok(SignupResult{id:res._id,code:signup.activation_code})
    }

    pub async fn resend_code(&self, id: String) -> Result<SignupResult, AuthError> {
        let signup = self.signup_repo().get_by_id(id.clone()).await.ok();
        
        match signup {
            Some(s) => {
                let now = Utc::now().timestamp_millis();
                return if now - s.last_sent_on > 1000 * 60 * 5 {
                    let code=get_code(5);
                    let signup = self
                        .signup_repo()
                        .update(
                            id,
                            SignUpUpdate {
                                last_sent_on: Some(now),
                                activation_code: Some(code.clone()),
                            },
                        )
                        .await?;
                        send_code(s.credentials.mobile_no,code.clone()).await?;
                    Ok(SignupResult{
                        id:signup._id,
                        code:code,
                    })
                } else {
                    Err(AuthError::RsendCodeTooSoon("Try agin later".into()))
                };
            }
            None => Err(AuthError::AccountNotFound("Signup Not Found".into())),
        }
    }

    pub async fn verify_signup(&self, id: String, code: String) -> Result<LoginResult, AuthError> {
        let mut map = Map::new();
        map.insert("_id".into(), Value::String(id));
        map.insert("activation_code".into(), Value::String(code.clone()));

        let signup = self.signup_repo().find_one(Value::Object(map)).await?;
        match signup {
            Some(si) => {
                return if si.activation_code.eq(&code) {
                    let acc = Account::from(si);
                   let acc= self.create_account(acc).await?;
                    Ok(LoginResult {

                        access_token: generate_token(acc.clone(),ONE_WEEK),
                        ttl_ms: ONE_WEEK,
                        account: acc,
                    })
                } else {
                    Err(AuthError::InvalidLogin("Code Mismatch".into()))
                }
            }
            None => Err(AuthError::InvalidLogin("Code Mismatch".into())),
        }
    }

    pub async fn login(&self, cred: Credentials) -> Result<LoginResult, AuthError> {
        let exists = self.exists(cred.mobile_no.clone()).await;
        match exists {
            Some(acc) => {
                let hashed = bcrypt::verify(&cred.password, &acc.password);
                match hashed {
                    Ok(val) => {
                        return if val == true {
                            let token=generate_token(acc.clone(),ONE_WEEK);
                            Ok(LoginResult{
                                account:acc,
                                access_token:token,
                                ttl_ms:ONE_WEEK
                            })
                        } else {
                            Err(AuthError::InvalidLogin("Password mismatch".into()))
                        }
                    }
                    Err(_e) => {
                        print!("{:?}",_e);
                        Err(AuthError::InvalidLogin("something went wrong".into()))
                    },
                }
            }
            None => Err(AuthError::InvalidLogin("User Not Found".into())),
        }
    }

    pub async fn reset_password(&self, payload: ResetPassword) -> Result<Account, AuthError> {
        let acc = self.get_by_reset_token(payload.reset_token).await;
        match acc {
            Some(ac) => self
                .repo()
                .update(
                    ac._id,
                    AccountUpdate {
                        password: Some(payload.newpassword),
                        ..AccountUpdate::default()
                    },
                )
                .await
                .map_err(|e| e.into()),
            None => Err(AuthError::AccountNotFound("account does not exist".into())),
        }
    }

    pub async fn modify_password(&self, payload: ModifyPassword) -> Result<Account, AuthError> {
        let acc = self.get_by_id(payload.account_id.clone()).await?;

        let matches = bcrypt::verify(&payload.oldpassword, &acc.password)?;
        if matches == true {
            return self
                .repo()
                .update(
                    payload.account_id,
                    AccountUpdate {
                        password: Some(payload.newpassword),
                        ..AccountUpdate::default()
                    },
                )
                .await
                .map_err(|e| e.into());
        } else {
            Err(AuthError::InvalidLogin("Not Found".into()))
        }
    }

    pub async fn forgot_password(&self,mobile_no:String)->Result<ForgotPassResult,AuthError>{
        let code=get_code(16);
        let acc_update=AccountUpdate{
            reset_token:Some(code.clone()),
            password: None,
            role: None,
        };
        let mut query=Map::new();
        query.insert("mobile_no".into(), Value::String(mobile_no.clone()));
        self.repo().update_where(Value::Object(query), acc_update).await?;

        send_code(mobile_no.clone(),code.clone()).await?;

        return Ok(ForgotPassResult{
            mobile_no:mobile_no,
            code:code
        });
    }

    async fn get_by_reset_token(&self, reset_token: String) -> Option<Account> {
        let mut query = Map::new();
        query.insert("reset_token".to_string(), Value::String(reset_token));
        let exists = self
            .repo()
            .find_one(Value::Object(query))
            .await
            .ok()
            .flatten();
        exists
    }

    async fn create_account(&self, mut account: Account) -> Result<Account, AuthError> {
        let exists = self.exists(account.mobile_no.clone()).await;
        match exists {
            Some(_ac) => Err(AuthError::AccountExists("account exists".into())),
            None => self.repo().create(&mut account).await.map_err(|e| e.into()),
        }
    }

    async fn exists(&self, mobile_no: String) -> Option<Account> {
        let mut query = Map::new();
        query.insert("mobile_no".to_string(), Value::String(mobile_no));
        let exists = self
            .repo()
            .find_one(Value::Object(query))
            .await
            .ok()
            .flatten();
        exists
    }
}

pub fn account_service_factory() -> AccountService {
    let repo = AccountRepository::new(ACCOUNTS.to_string());
    return AccountService::new(repo);
}
