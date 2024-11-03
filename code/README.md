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

### Unwraps and validations

As of now wrongly formatted input will mostly end up in either 422 or a 500, but should not be persisted.
There are some unwraps where a error handling would be better, let us consider it a TODO.

## Design Decisions and Goals

### The scaling

While this application does help to also scale on organization level, it is not the only place to see it.
The main focus is that this project can be maintianed by one or two individuals.

This is achieved through:

- Writing the code in a way that it is easy to add a simply database model.
- Organizing the code to find the relevant parts quickly.
- Writing view close to html and mostly relying on clear functions and no magic.
- Mirroring the flow of the onboarding in the code.
- Only having Runtime failures where sensible.

### Reusability

The applications as a whole, can be forked and then adjusted to a different domain easily.
For example a booking system for a hair salon or a city tour office.

Future changes should steer even more towards this goal.

However the code is and should not be generialzed as it has advantages in being opinionated.
For example what a SuperBabsy is might be easier to understand if it is called like that in the code.

### Embedded js and css

inside of code/src/assets there are files which are constants for:

- htmx
- pico css

Those two libraries are used to make the frontend more dynamic and nice to look at.

When writing an applicaton in rust with rocket you get consistency, reproducability and smooth builds. 
By embedding core frontend libraries you can preserve the benefits eventhough you depend on java script libraries.
On top of that the code has a single source of ecosystem dependencies, crates.io.

