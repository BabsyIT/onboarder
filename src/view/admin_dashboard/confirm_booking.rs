use rocket::{response::content::RawHtml, State};

use crate::persistence::booking_requests::BookingRequests;

use super::booking_table;

// leaving the METHOD as post to leave open soft delete rfc style
#[post("/bookings/confirm?<booking_id>")]
pub fn confirm_booking(booking_id: String, bookings: &State<BookingRequests>) -> RawHtml<String> {
    let confirmed = bookings.confirm_booking_request(&booking_id, "Confirmed".to_string());

    match confirmed {
        Some(core) => RawHtml(booking_table::tr(core).into_string()),
        None => RawHtml("".to_string()),
    }
}
