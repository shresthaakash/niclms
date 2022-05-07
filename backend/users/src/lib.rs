extern crate framework;
#[macro_use]
extern crate macros;

extern crate common;

mod user;
mod user_update;
mod new_user;
mod user_repository;

pub use user::*;
pub use user_update::UserUpdate;
pub use new_user::NewUser;


pub mod user_service;