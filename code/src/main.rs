use std::env;

use assets::mount_assets;
use rocket::{Build, Rocket};
use tec::mount_tec;
use view::onboarding_form::hours::hours_view_html;
use view::onboarding_form::superbabsys::get_superbabsys;

#[macro_use]
extern crate rocket;

mod assets;
mod persistence;
mod superbabsys;
mod tec;
mod view;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let port = env::var("ROCKET_PORT").unwrap_or("8000".to_string());

    let path = format!("http://127.0.0.1:{}", port);
    println!("Server running on {}", path);

    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let with_index = rocket.mount("/", routes![view::index, get_superbabsys, hours_view_html,]);

    let with_assets = mount_assets(with_index);

    let with_tec = mount_tec(with_assets);

    persistence::manage(with_tec)
}
