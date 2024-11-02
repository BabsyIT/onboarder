use maud::{html, Markup};

use crate::bookings::Booking;

pub fn generate(bookings: Vec<Booking>) -> Markup {
    html! {
        table {
            thead {
                tr {
                    th { "ID" }
                    th { "Date" }
                    th { "Given name" }
                    th { "Family name" }
                    th { "Email" }
                    th { "Phone" }
                    th { "Street" }
                    th { "House Number" }
                    th { "Canton" }
                    th { "City" }
                    th { "Zip" }
                    th { "Country" }
                }
            }
            tbody {
                @for booking in bookings {
                    tr {
                        td { (booking.get_id()) }
                        td { (booking.get_date()) }
                        td { (booking.get_given_name()) }
                        td { (booking.get_family_name()) }
                        td { (booking.get_email()) }
                        td { (booking.get_phone()) }
                        td { (booking.get_address().get_street()) }
                        td { (booking.get_address().get_house_number()) }
                        td { (booking.get_address().get_canton()) }
                        td { (booking.get_address().get_city()) }
                        td { (booking.get_address().get_zip()) }
                        td { (booking.get_address().get_country())
                    }
                }
            }
        }
    }
    }
}
