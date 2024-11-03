use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct Booking {
    id: String,
    super_babsy_id: String,
    date: NaiveDateTime,
    given_name: String,
    family_name: String,
    email: String,
    phone: String,
    adress: Adress,
    #[allow(unused)]
    file: Vec<IdCard>,

    //actual states
    state: BookingState,
}

#[derive(Clone)]
pub enum BookingState {
    Pending,
    Approved(String),
    Rejected,
}

impl Booking {
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn cloned_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_babsy_id(&self) -> &String {
        &self.super_babsy_id
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

    #[allow(unused)]
    pub fn get_file(&self) -> &Vec<IdCard> {
        &self.file
    }

    pub fn get_address(&self) -> Adress {
        self.adress.clone()
    }
    pub fn get_state(&self) -> &BookingState {
        &self.state
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        super_babsy_id: String,
        date: NaiveDateTime,
        given_name: String,
        family_name: String,
        email: String,
        phone: String,
        adress: Adress,
        file: Vec<IdCard>,
        state: BookingState,
    ) -> Self {
        Self {
            id,
            super_babsy_id,
            date,
            given_name,
            family_name,
            email,
            phone,
            adress,
            file,
            state,
        }
    }

    pub fn confirmed(&self, comment: String) -> Booking {
        let mut clone = self.clone();
        clone.state = BookingState::Approved(comment);
        clone
    }

    pub fn rejected(&self) -> Booking {
        let mut clone = self.clone();
        clone.state = BookingState::Rejected;
        clone
    }
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
#[allow(unused)]
pub enum IdCard {
    Jpg(String),
    Pdf(String),
    Jpeg(String),
    Png(String),
}
