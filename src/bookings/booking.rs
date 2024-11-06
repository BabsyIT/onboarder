use chrono::NaiveDateTime;

use super::{Address, BookingState, IdCard};

#[derive(Clone)]
pub struct Booking {
    id: String,
    super_babsy_id: String,
    date: NaiveDateTime,
    given_name: String,
    family_name: String,
    email: String,
    phone: String,
    adress: Address,
    #[allow(unused)]
    file: Vec<IdCard>,

    state: BookingState,
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

    pub fn get_address(&self) -> Address {
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
        adress: Address,
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
