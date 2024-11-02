use chrono::NaiveDateTime;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Adress {
    street: String,
    house_number: String,
    canton: String,
    city: String,
    zip: String,
    country: String,
}

impl Adress {
    pub fn new(
        street: String,
        house_number: String,
        canton: String,
        city: String,
        zip: String,
        country: String,
    ) -> Self {
        Self {
            street,
            house_number,
            canton,
            city,
            zip,
            country,
        }
    }

    pub fn get_street(&self) -> &String {
        &self.street
    }
    pub fn get_house_number(&self) -> &String {
        &self.house_number
    }
    pub fn get_canton(&self) -> &String {
        &self.canton
    }
    pub fn get_city(&self) -> &String {
        &self.city
    }
    pub fn get_zip(&self) -> &String {
        &self.zip
    }

    pub fn get_country(&self) -> &String {
        &self.country
    }
}

#[derive(Clone)]
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

    pub fn get_date(&self) -> &NaiveDateTime {
        &self.date
    }

    pub fn get_given_name(&self) -> &String {
        &self.given_name
    }

    pub fn get_family_name(&self) -> &String {
        &self.family_name
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_phone(&self) -> &String {
        &self.phone
    }
    pub fn get_adress(&self) -> &Adress {
        &self.adress
    }
    pub fn get_file(&self) -> &Vec<IdCard> {
        &self.file
    }
    pub fn get_message(&self) -> &String {
        &self.message
    }
    pub fn get_address(&self) -> Adress {
        self.adress.clone()
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
