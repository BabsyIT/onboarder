use chrono::NaiveDateTime;

use rocket::fs::TempFile;

struct Booking {
    date: NaiveDateTime,
    given_name: String,
    family_name: String,
    email: String,
    phone: String,
    adress: Adress,
    file: Vec<IdCard<'static>>,
    message: String,
}

struct Adress {
    street: String,
    city: String,
    zip: String,
    country: String,
}

enum IdCard<'v> {
    Jpg(TempFile<'v>),
    Pdf(TempFile<'v>),
    Jpeg(TempFile<'v>),
    Png(TempFile<'v>),
}
