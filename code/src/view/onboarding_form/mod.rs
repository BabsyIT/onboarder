use chrono::Utc;
use maud::{html, Markup};

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



        input
            hx-post="/employee"
            hx-trigger="change delay:500ms"
            type="date"
            name="date"
            min={(formatted_time)} {
        };

            @for day in days {
                input
                    hx-post="/employee"
                    hx-trigger="change delay:500ms"
                    type="checkbox"
                    value={(day)}
                    name={(day)} { (day) }
            }



        select
            hx-post="/employee"
            hx-trigger="change delay:500ms"
        {
            @for hour in hours {
                option
                    value={(hour)} { (hour) }
            }
        }

     hr{};

     div id="employee-selection" {

     }
    }
}
