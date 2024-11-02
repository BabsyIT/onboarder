use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};

use crate::superbabsys::{Availability, AvailabilityRange, LanguageCompetency, SuperBabsy};

pub fn fake_data() -> HashMap<String, SuperBabsy> {
    let mut babsy_map = HashMap::new();

    let fake_andrea = fake_andrea();
    let fake_catarina = fake_catarina();
    let fake_aswin = fake_aswin();
    let fake_mia = fake_mia();
    let fake_steffi = fake_steffi();
    
    babsy_map.insert(fake_andrea.id.to_string(), fake_andrea);
    babsy_map.insert(fake_catarina.id.to_string(), fake_catarina);
    babsy_map.insert(fake_aswin.id.to_string(), fake_aswin);
    babsy_map.insert(fake_mia.id.to_string(), fake_mia);
    babsy_map.insert(fake_steffi.id.to_string(), fake_steffi);

    babsy_map
}

pub fn fake_andrea() -> SuperBabsy {
    let mut andrea = SuperBabsy::new_base("Andrea S.".to_string(), r###"
        I am a super babsy
        Andrea, Präsidentin bei Babsy, freut sich andere Sitter:innen und Eltern im Verein zu begrüssen.
        Andrea, president herself at Babsy, is happy and looking forward of introducing new Sitters and Parents in our community.
    "###.to_string());

    andrea.availability = fake_availability();
    andrea.sitter = Some(vec![LanguageCompetency::german(), LanguageCompetency::english()]);
    andrea.parent = Some(vec![LanguageCompetency::german(), LanguageCompetency::english()]);
    
    andrea
}

pub fn fake_catarina() -> SuperBabsy {
    let mut catarina = SuperBabsy::new_base("Catarina O.".to_string(), r###"
        Catarina, selbst registrierte Sitterin bei Babsy, freut sich andere Sitter:innen und Eltern im Verein zu begrüssen.
        Catarina, registered Sitter herself at Babsy, is happy and looking forward of introducing new Sitters and Parents in our community.
    "###.to_string());

    catarina.availability = fake_availability_2();
    catarina.sitter = Some(vec![LanguageCompetency::english()]);
    catarina.parent = Some(vec![LanguageCompetency::english()]);
    
    catarina
}

pub fn fake_aswin() -> SuperBabsy {
    let mut aswin = SuperBabsy::new_base("Aswin S.".to_string(), r###"
        Aswin, selbst Sitter bei Babsy, freut sich andere Sitter:innen und Eltern im Verein zu begrüssen.
        
        Aswin, Sitter himself at Babsy, is happy and looking forward of introducing new Sitters and Parents in our community.
    "###.to_string());

    aswin.availability = fake_availability_2();
    aswin.sitter = Some(vec![LanguageCompetency::english()]);
    aswin.parent = None;
    
    aswin
}

pub fn fake_steffi() -> SuperBabsy {
    let mut steffi = SuperBabsy::new_base("Steffi M.".to_string(), r###"
        Steffi, Vize-Präsidentin bei Babsy, freut sich andere Sitter:innen und Eltern im Verein zu begrüssen.
        
        Steffi, vice-president herself at Babsy, is happy and looking forward of introducing new Sitters and Parents in our community.
    "###.to_string());

    steffi.availability = fake_availability();
    steffi.sitter = Some(vec![LanguageCompetency::english()]);
    steffi.parent = Some(vec![LanguageCompetency::german()]);
    
    steffi
}

pub fn fake_mia() -> SuperBabsy {
    let mut mia = SuperBabsy::new_base("Mia M.".to_string(), r###"
        Mia, selbst registrierte Sitterin bei Babsy, freut sich andere Sitter:innen und Eltern im Verein zu begrüssen.
        Mia, Sitter herself at Babsy, is happy and looking forward of introducing new Sitters and Parents in our community.
"###.to_string());

    mia.availability = fake_availability();
    mia.sitter = None;
    mia.parent = Some(vec![LanguageCompetency::german()]);
    
    mia
}


fn fake_availability_2() -> Availability {
    let mut availability = Availability::new();

    availability.add_date(
        NaiveDate::parse_from_str("2021-09-01", "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str("2027-01-01", "%Y-%m-%d").unwrap(),
    );

    availability
}

fn fake_availability() -> Availability {
    let mut availability = Availability::new();

    availability.add_date(
        NaiveDate::parse_from_str("2025-09-01", "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str("2027-01-01", "%Y-%m-%d").unwrap(),
    );

    availability
}
