use maud::{html, Markup};
use rocket::{response::content, State};

use crate::{
    bookings::Booking,
    persistence::{booking_requests::BookingRequests, super_babsys::SuperBabsys},
    superbabsys::SuperBabsy,
};

use super::page;

mod booking_table;
mod user_table;

#[get("/admin?<tab>")]
pub fn admin(
    tab: Option<String>,
    booking_requests: &State<BookingRequests>,
    superbabsys: &State<SuperBabsys>,
) -> content::RawHtml<String> {
    let tab = tab.unwrap_or("bookings".to_string());
    let bookings = booking_requests.get_booking_requests();
    let superbabsys = superbabsys.get_super_babsys();

    let raw = match tab.as_str() {
        "bookings" => bookings_table(bookings),
        "users" => user_table(superbabsys),
        _ => bookings_table(bookings),
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
                        a href="/admin?tab=users" { "Users" }
                    }
                }
            }
        }

    }
}
