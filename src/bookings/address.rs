#[derive(Clone)]
pub struct Address {
    street: String,
    house_number: String,
    canton: String,
    city: String,
    zip: String,
    country: String,
}

impl Address {
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
