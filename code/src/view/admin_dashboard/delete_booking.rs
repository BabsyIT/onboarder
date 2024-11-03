use rocket::{response::content::RawHtml, State};

use crate::persistence::booking_requests::BookingRequests;

// leaving the METHOD as post to leave open soft delete rfc style
#[post("/bookings/delete?<booking_id>")]
pub fn delete_booking(booking_id: String, bookings: &State<BookingRequests>) -> RawHtml<String> {
    bookings.delete_booking_request(&booking_id);

    // return nothing to replace td with nothing
    RawHtml("".to_string())
}
