use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rocket::{Build, Rocket};

use crate::bookings::Booking;

pub struct BookingRequests {
    booking_map: Arc<Mutex<HashMap<String, Booking>>>,
}

impl BookingRequests {
    pub fn new(booking_map: Arc<Mutex<HashMap<String, Booking>>>) -> Self {
        Self { booking_map }
    }
    pub fn add_booking_request(&self, booking_request: Booking) {
      let _ = &self.booking_map.lock().unwrap().insert(booking_request.get_id().clone(), booking_request);
    }
}

pub fn manage(rocket: Rocket<Build>) -> Rocket<Build> {
    let map = Arc::new(Mutex::new(HashMap::new()));
    let in_memory_booking_requests = BookingRequests::new(map);
    rocket.manage(in_memory_booking_requests)
}