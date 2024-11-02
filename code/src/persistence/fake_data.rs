use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};

use crate::superbabsys::{Availability, AvailabilityRange, SuperBabsy};

pub fn fake_data() -> HashMap<String, SuperBabsy> {
    let mut babsy_map = HashMap::new();

    babsy_map.insert("Andrea S.".to_string(), fake_super_babsy());

    babsy_map
}

pub fn fake_super_babsy() -> SuperBabsy {
    let mut andrea = SuperBabsy::new_base("Andrea S.".to_string(), r###"
        I am a super babsy
        Andrea, Präsidentin bei Babsy, freut sich andere Sitter:innen und Eltern im Verein zu begrüssen.
        Andrea, president herself at Babsy, is happy and looking forward of introducing new Sitters and Parents in our community.
    "###.to_string());

    andrea.availability = fake_availability();

    andrea
}

fn fake_availability() -> Availability {
    let mut availability = Availability::new();

    availability.add_date(
        NaiveDate::parse_from_str("2025-09-01", "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str("2027-01-01", "%Y-%m-%d").unwrap(),
    );

    availability
}
