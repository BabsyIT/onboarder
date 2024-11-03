#[derive(Clone)]
pub enum BookingState {
    Pending,
    Approved(String),
    Rejected,
}
