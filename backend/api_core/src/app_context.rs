use auth::account_service::{account_service_factory, AccountService};
use users::user_service::{user_service_factory, UserService};
pub struct AppContext {
    pub user_service: UserService,
    pub account_service: AccountService,
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            user_service: user_service_factory(),
            account_service: account_service_factory(),
        }
    }
}
