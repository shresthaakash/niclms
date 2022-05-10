use auth::account_service::{account_service_factory, AccountService};
use users::user_service::{user_service_factory, UserService};

use crate::resolvers::AppResolver;
pub struct AppContext {
    pub user_service: UserService,
    pub account_service: AccountService,
    pub app_resolver:AppResolver,
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            user_service: user_service_factory(),
            account_service: account_service_factory(),
            app_resolver:AppResolver::new(),
        }
    }
}
