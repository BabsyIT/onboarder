use maud::{html, Markup};
use rocket::{response::content, State};

use crate::{
    bookings::Booking,
    persistence::{booking_requests::BookingRequests, super_babsys::SuperBabsys},
    superbabsys::SuperBabsy,
};

use super::page;

mod booking_table;
pub mod confirm_booking;
pub mod delete_booking;
pub mod reject_booking;
mod user_table;

#[get("/admin?<tab>&<babsy_id>")]
pub fn admin(
    tab: Option<String>,
    // todo replace by authenticed user state and get id from there
    babsy_id: Option<String>,
    booking_requests: &State<BookingRequests>,
    superbabsys: &State<SuperBabsys>,
) -> content::RawHtml<String> {
    let tab = tab.unwrap_or("bookings".to_string());
    let bookings = booking_requests.get_booking_requests();
    let bookings = match babsy_id {
        Some(id) => bookings
            .into_iter()
            .filter(|booking| booking.get_babsy_id() == &id)
            .collect(),
        None => bookings,
    };
    let superbabsys = superbabsys.get_super_babsys();

    let raw = match tab.as_str() {
        "bookings" => bookings_table(bookings),
        "users" => user_table(superbabsys),
        _ => "This tab does not exist".to_string(),
    };

    content::RawHtml(raw)
}

pub fn bookings_table(bookings: Vec<Booking>) -> String {
    page(
        "Onboarding Dashboard".to_string(),
        html! {
            {(tabs())}
            {(body(bookings))}
        },
    )
    .into_string()
}

pub fn user_table(superbabsys: Vec<SuperBabsy>) -> String {
    page(
        "Onboarding Dashboard".to_string(),
        html! {
            {(tabs())}
            body {
                main .container {
                    div #main-content{
                        {(user_table::generate(superbabsys))}
                    }
                }
            }

        },
    )
    .into_string()
}

pub fn body(bookings: Vec<Booking>) -> Markup {
    html! {
        body {
            main .container {
                div #main-content{
                    {(booking_table::generate(bookings))}
                }
            }
        }
    }
}

pub fn tabs() -> Markup {
    html! {
        header .container {
            nav{
                ul{
                    li{
                        "Onboarder Dashboard"
                    }
                }
                ul{
                    li{
                        a href="/admin?tab=bookings" { "Bookings" }
                    }
                    li{
                        a href="/admin?tab=users" { "SuperBabsys" }
                    }
                }
            }
        }

    }
}
