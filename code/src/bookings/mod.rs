
use chrono::NaiveDateTime;


pub struct Booking {
    id: String,
    date: NaiveDateTime,
    given_name: String,
    family_name: String,
    email: String,
    phone: String,
    adress: Adress,
    file: Vec<IdCard>,
    message: String,
}

pub struct Adress {
    street: String,
    house_number: String,
    canton: String,
    city: String,
    zip: String,
    country: String,
}

impl Adress {
    pub fn new(street: String, house_number: String, canton: String, city: String, zip: String, country: String) -> Self {
        Self {
            street,
            house_number,
            canton,
            city,
            zip,
            country,
        }
    }
    
}

pub enum IdCard {
    Jpg(String),
    Pdf(String),
    Jpeg(String),
    Png(String),
}

impl Booking {
    pub fn get_id(&self) -> &String {
        &self.id
    }
    
    pub fn new(
        id: String,
        date: NaiveDateTime,
        given_name: String,
        family_name: String,
        email: String,
        phone: String,
        adress: Adress,
        file: Vec<IdCard>,
        message: String,
    ) -> Self {
        Self {
            id,
            date,
            given_name,
            family_name,
            email,
            phone,
            adress,
            file,
            message,
        }
    }
}