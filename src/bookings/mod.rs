mod address;
mod booking;
mod booking_state;

pub use address::Address;
pub use booking::Booking;
pub use booking_state::BookingState;

#[derive(Clone)]
#[allow(unused)]
pub enum IdCard {
    Jpg(String),
    Pdf(String),
    Jpeg(String),
    Png(String),
}
