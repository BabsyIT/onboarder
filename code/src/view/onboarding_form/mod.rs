use chrono::Utc;
use maud::{html, Markup};

pub mod superbabsys;

pub fn form() -> Markup {
    let dt = Utc::now();

    // Format the datetime to the required string format
    let formatted_time = dt.format("%Y-%m-%d").to_string();

    let days = vec![
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    let range = 8..21;
    let hours: Vec<String> = range.map(|i| format!("{}:00", i)).collect();
    html! {

            form
                hx-post="/employees"
                hx-target="#employee-selection"
                hx-trigger="change from:input, change from:select, change from:checkbox"
                hx-swap="outerHTML"
        {

            input
                type="date"
                name="date"
                min={(formatted_time)} {};

            fieldset {
                @for day in days {
                    input
                        type="checkbox"
                        name={(day)} { (day) }
                }
            }
            select
                name="from"
            {
                @for hour in hours.clone() {
                    option
                        value={(hour)} { (hour) }
                }
            }

            select
                name="to"
            {
                @for hour in hours {
                    option
                        value={(hour)} { (hour) }
                }
            }
    }
         hr{};

         div id="employee-selection" {

         }
        }
}
