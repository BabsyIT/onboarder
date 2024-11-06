use maud::{html, Markup, PreEscaped};

use crate::bookings::{Booking, BookingState};

pub fn generate(bookings: Vec<Booking>) -> Markup {
    html! {
        table .striped {
            thead {
                tr {
                    th { "ID" }
                    th { "Date" }
                    th { "Booking State" }
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
                    th { "Actions"}
                }
            }
            tbody {
                @for booking in bookings {
                   ({tr(booking)})
                }
            }
        }
    }
}

pub fn tr(booking: Booking) -> Markup {
    html! {
            tr #(booking_target_id(booking.clone())) {
                td { (booking.get_id()) }
                td { (booking.get_date()) }
                td {
                    (booking_state(booking.get_state().clone()))
                }
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

                td {
                    div .group {
                        button ."action-button"
                         hx-target=(to_id(&booking_target_id(booking.clone())))
                         hx-post={(format!("bookings/delete?booking_id={}", booking.get_id()))}
                         hx-swap="outerHTML"
                         { (garbage())}


                        button ."action-button"
                            hx-target=(to_id(&booking_target_id(booking.clone())))
                         hx-post={(format!("bookings/confirm?booking_id={}", booking.get_id()))}
                         hx-swap="outerHTML"
                         { (confirm()) }

                         button ."action-button"
                             hx-target=(to_id(&booking_target_id(booking.clone())))
                          hx-post={(format!("bookings/reject?booking_id={}", booking.get_id()))}
                          hx-swap="outerHTML"
                          { (no()) }
                    }

                }
            }
        }
    }
}

fn to_id(string: &str) -> String {
    let mut prefix = "#".to_string();
    prefix.push_str(string);
    prefix
}

fn booking_target_id(booking: Booking) -> String {
    let mut prefix = "b".to_string();
    prefix.push_str(booking.get_given_name());
    prefix.push_str(booking.get_family_name());
    prefix.push_str(booking.get_address().get_city());
    prefix.replace(' ', "")
}

fn booking_state(booking_state: BookingState) -> Markup {
    match booking_state {
        crate::bookings::BookingState::Pending => html! {
              kbd { "Pending" }
        },
        crate::bookings::BookingState::Approved(comment) => html! {
               ins { (comment) }
        },
        crate::bookings::BookingState::Rejected => html! {
             del { "Rejected" }
        },
    }
}

pub fn garbage() -> Markup {
    html! {
        (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5">
          <path fill-rule="evenodd" d="M8.75 1A2.75 2.75 0 0 0 6 3.75v.443c-.795.077-1.584.176-2.365.298a.75.75 0 1 0 .23 1.482l.149-.022.841 10.518A2.75 2.75 0 0 0 7.596 19h4.807a2.75 2.75 0 0 0 2.742-2.53l.841-10.52.149.023a.75.75 0 0 0 .23-1.482A41.03 41.03 0 0 0 14 4.193V3.75A2.75 2.75 0 0 0 11.25 1h-2.5ZM10 4c.84 0 1.673.025 2.5.075V3.75c0-.69-.56-1.25-1.25-1.25h-2.5c-.69 0-1.25.56-1.25 1.25v.325C8.327 4.025 9.16 4 10 4ZM8.58 7.72a.75.75 0 0 0-1.5.06l.3 7.5a.75.75 0 1 0 1.5-.06l-.3-7.5Zm4.34.06a.75.75 0 1 0-1.5-.06l-.3 7.5a.75.75 0 1 0 1.5.06l.3-7.5Z" clip-rule="evenodd" />
        </svg>
"#))
    }
}

pub fn no() -> Markup {
    html! {
        (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5">
          <path fill-rule="evenodd" d="m5.965 4.904 9.131 9.131a6.5 6.5 0 0 0-9.131-9.131Zm8.07 10.192L4.904 5.965a6.5 6.5 0 0 0 9.131 9.131ZM4.343 4.343a8 8 0 1 1 11.314 11.314A8 8 0 0 1 4.343 4.343Z" clip-rule="evenodd" />
        </svg>

"#))
    }
}

pub fn confirm() -> Markup {
    html! {
        (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5">
          <path fill-rule="evenodd" d="M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16Zm3.857-9.809a.75.75 0 0 0-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 1 0-1.06 1.061l2.5 2.5a.75.75 0 0 0 1.137-.089l4-5.5Z" clip-rule="evenodd" />
        </svg>
"#))
    }
}
