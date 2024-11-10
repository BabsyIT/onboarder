use assets::mount_assets;
use rocket::{Build, Rocket};
use tec::mount_tec;
use view::admin_dashboard;
use view::onboarding_form::booking::bookings_view_html;
use view::onboarding_form::hours::hours_view_html;
use view::onboarding_form::new_booking;
use view::onboarding_form::superbabsys::get_superbabsys;

#[macro_use]
extern crate rocket;

mod assets;
mod auth;
mod bookings;
mod fake_app;
mod persistence;
mod superbabsys;
mod tec;
mod view;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let with_index = rocket.mount(
        "/",
        routes![
            fake_app::fake_app,
            view::index,
            get_superbabsys,
            hours_view_html,
            bookings_view_html,
            new_booking::new_booking,
            admin_dashboard::admin,
            admin_dashboard::delete_booking::delete_booking,
            admin_dashboard::confirm_booking::confirm_booking,
            admin_dashboard::reject_booking::reject_booking,
        ],
    );

    let with_assets = mount_assets(with_index);

    let with_tec = mount_tec(with_assets);

    let with_super_babsys = persistence::super_babsys::manage(with_tec);
    let with_super_babsy = persistence::booking_requests::manage(with_super_babsys);
    auth::mount_auth(with_super_babsy)
}
