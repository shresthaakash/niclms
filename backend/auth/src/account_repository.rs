
use framework::repository::Repository;

use crate::account::{Account, AccountUpdate, SignUp, SignUpUpdate};


pub type AccountRepository=Repository<Account,AccountUpdate>;


pub type SignUpRepository=Repository<SignUp,SignUpUpdate>;
