use maud::{html, Markup};
use rocket::{response::content, State};

use crate::{bookings::Booking, persistence::booking_requests::BookingRequests};

use super::page;

mod table;

#[get("/admin")]
pub fn admin(booking_requests: &State<BookingRequests>) -> content::RawHtml<String> {
    let bookings = booking_requests.get_booking_requests();
    let raw = page(
        "Onboarding Dashboard".to_string(),
        html! {
            {(body(bookings))}
        },
    )
    .into_string();

    content::RawHtml(raw)
}

pub fn body(bookings: Vec<Booking>) -> Markup {
    html! {
        body {
            main .container {
                div #main-content{
                    {(table::generate(bookings))}
                }
            }
        }
    }
}
