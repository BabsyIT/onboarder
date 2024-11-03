# Code Readme

## Local development

### Prerequisites

- Rust
- Cargo

### Optional

- just  


### Running the server

```bash

cargo run

```


```bash

just run

```

> local address: http://127.0.0.1:8000


### Tests and lints

```bash

cargo test
cargo fmt --all -- --check
cargo clippy

```

```bash

just verify

```

## Important  

The heart of the application is the calendar booking system.
SuperBabsys have availabilities which determine if they are shown to the potential sitter or parent.
If forward looking time window of the booking intesects an availability of a SuperBabsy the SuperBabsy can be shown to the user.

The second important part are the compentencies options (Language per users type: English for Sitters), which need to intersect with the SuperBabsys compentencies.

## Pitfalls and Limitations

### Authentication

As of now there is no authentication for the admin/superbabsy dashboard.

The dashboard would only show the bookings for the SuperBabsy who is logged in. 
This can be simulated by:

> going to the URL with the  superbabsy_id query parameter: `/admin?tab=bookings&superbabsy_id=<id>`

### Database

As of now the database is are two simple hash maps.
But as you may figure out it will not be hard to switch to a real database later.
The availability of the SuperBabsy can be stored as a list of time stamps pairs in one table.
The rest is just data about the user or about a booking.

As the ids are UUIDs it is okay to leave the generation to the Application.

### Frontend

The frontend, eventhough optimized for mobile, may have one or two small hickups when it comes to button not being scaled well.

### Managing state

The state is managed without any abstraction, meaning no traits in the state extractor of the http handler functions:

```rust

// the booking state is the whole struct: bookings: &State<BookingRequests> not a trait
#[post("/bookings/delete?<booking_id>")]
pub fn delete_booking(booking_id: String, bookings: &State<BookingRequests>) -> RawHtml<String> {
    bookings.delete_booking_request(&booking_id);

    RawHtml("".to_string())
}

```

### The fake app

The fake app is only to demonstrate the flow from the mobile app to the onboarder.
It might be removed in the future.