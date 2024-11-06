use chrono::NaiveDateTime;
use maud::html;
use rocket::{
    form::Form, fs::TempFile, response::content::RawHtml, tokio::io::AsyncReadExt, State,
};

use crate::{
    bookings::{Address, Booking, BookingState, IdCard},
    persistence::booking_requests,
};

#[derive(FromForm)]
pub struct NewBookingData<'r> {
    super_babsy_id: &'r str,
    // actual date time
    date: &'r str,
    // Vorname / First name
    first_name: &'r str,
    // Nachname / Last name
    last_name: &'r str,
    // Telefon / Phone
    phone: &'r str,
    // E-Mail / Email
    email: &'r str,
    // Name der Strasse / Street Name
    street_name: &'r str,
    // Hausnummer / Number
    house_number: &'r str,
    // PLZ / Postal Code
    postal_code: &'r str,
    // Stadt / City
    city: &'r str,
    // Kanton / Canton
    canton: &'r str,
    // Land / Country
    country: &'r str,
    // Personalausweis Vorderseite (ID, Reisepass) / Identity Card front (ID, Passport)
    id_front: Option<TempFile<'r>>,
    // Personalausweis Rückseite (ID, Reisepass) / Identity Card back (ID, Passport)
    id_back: Option<TempFile<'r>>,
}

#[post("/booking/new", data = "<new_booking_data>")]
pub async fn new_booking(
    new_booking_data: Form<NewBookingData<'_>>,
    bookings: &State<booking_requests::BookingRequests>,
) -> RawHtml<String> {
    let new_booking_data = new_booking_data.into_inner();
    let id = uuid::Uuid::new_v4().to_string();
    let date = NaiveDateTime::parse_from_str(new_booking_data.date, "%Y-%m-%d %H:%M:%S").unwrap();

    let given_name = new_booking_data.first_name.to_string();
    let family_name = new_booking_data.last_name.to_string();
    let email = new_booking_data.email.to_string();
    let phone = new_booking_data.phone.to_string();
    let street = new_booking_data.street_name.to_string();
    let house_number = new_booking_data.house_number.to_string();
    let canton = new_booking_data.canton.to_string();
    let city = new_booking_data.city.to_string();
    let zip = new_booking_data.postal_code.to_string();
    let country = new_booking_data.country.to_string();

    let mut pictures: Vec<IdCard> = vec![];

    if let Some(front) = new_booking_data.id_front {
        let mut front_buffer = String::new();
        let _ = front
            .open()
            .await
            .unwrap()
            .read_to_string(&mut front_buffer)
            .await;
        pictures.push(IdCard::Jpg(front_buffer));
    }

    if let Some(back) = new_booking_data.id_back {
        let mut back_buffer = String::new();
        let _ = back
            .open()
            .await
            .unwrap()
            .read_to_string(&mut back_buffer)
            .await;
        pictures.push(IdCard::Jpg(back_buffer));
    }

    let booking = Booking::new(
        id.clone(),
        new_booking_data.super_babsy_id.to_string(),
        date,
        given_name,
        family_name,
        email,
        phone,
        Address::new(street, house_number, canton, city, zip, country),
        pictures,
        BookingState::Pending,
    );
    bookings.add_booking_request(booking);

    let raw = html! {
           article {
             header {
                   h1 { "Vielen Dank für Ihre Buchungsanfrage!" }
             }
             body{
                 p { "Wir werden uns so bald wie moeglich melden" }
                 p { "Du wirst eine Bestätigung von der Super Babsy bekommen" }
                 p { "Bitte kontrollieren Sie Ihren spam ordner" }
             }
             footer {
                 p { "Danke für Ihr Vertrauen" }
             }
           }


           article {
             header {
                   h1 { "Thank you for your booking request!"  }
             }
             body{

                 p { "We will get back to you as soon as possible." }
                 p { "You will receive a confirmation email from the Super Babsy." }
                 p { "Please check your spam folder if you do not receive an email." }
             }
             footer {
                 p { "Thank you for your trust!" }
             }
           }


    };
    RawHtml(raw.into_string())
}
