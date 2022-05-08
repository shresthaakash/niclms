use super::app::{App,AppUpdate};
use framework::repository::Repository;

pub type AppRepository=Repository<App,AppUpdate>;
