#[macro_use]
extern crate rocket;
use api::init_app;

#[launch]
fn launch() -> _ {
    init_app()
}