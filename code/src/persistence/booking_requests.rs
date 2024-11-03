use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rocket::{Build, Rocket};

use crate::bookings::Booking;

use super::fake_data;

pub struct BookingRequests {
    booking_map: Arc<Mutex<HashMap<String, Booking>>>,
}

impl BookingRequests {
    pub fn new(booking_map: Arc<Mutex<HashMap<String, Booking>>>) -> Self {
        Self { booking_map }
    }
    pub fn add_booking_request(&self, booking_request: Booking) {
        let _ = &self
            .booking_map
            .lock()
            .unwrap()
            .insert(booking_request.get_id().clone(), booking_request);
    }

    pub fn delete_booking_request(&self, id: &str) {
        let _ = &self.booking_map.lock().unwrap().remove(id);
    }

    pub fn confirm_booking_request(&self, id: &str, comment: String) -> Option<Booking> {
        let booking = self.booking_map.lock().unwrap().get(id).cloned()?;
        let confirmed = booking.confirmed(comment);
        self.booking_map
            .lock()
            .unwrap()
            .insert(confirmed.cloned_id(), confirmed.clone());
        Some(confirmed)
    }

    pub fn reject_booking_request(&self, id: &str) -> Option<Booking> {
        let booking = self.booking_map.lock().unwrap().get(id).cloned()?;
        let rejected = booking.rejected();
        self.booking_map
            .lock()
            .unwrap()
            .insert(rejected.cloned_id(), rejected.clone());
        Some(rejected)
    }

    pub fn get_booking_requests(&self) -> Vec<Booking> {
        self.booking_map.lock().unwrap().values().cloned().collect()
    }
}

pub fn manage(rocket: Rocket<Build>) -> Rocket<Build> {
    let mut map = HashMap::new();
    let fake_booking = fake_data::fake_booking();
    map.insert(fake_booking.get_id().clone(), fake_booking);

    let map = Arc::new(Mutex::new(map));

    let in_memory_booking_requests = BookingRequests::new(map);
    rocket.manage(in_memory_booking_requests)
}
