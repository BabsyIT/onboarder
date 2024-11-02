use chrono::Utc;
use maud::{html, Markup};

use crate::superbabsys::LanguageCompetency;

pub mod superbabsys;

pub enum UserType {
    Sitter,
    Parent,
}

impl UserType {
    pub fn code(&self) -> String {
        match self {
            UserType::Sitter => "sitter".to_string(),
            UserType::Parent => "parent".to_string(),
        }
    }
}

impl TryFrom<String> for UserType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "sitter" => Ok(UserType::Sitter),
            "parent" => Ok(UserType::Parent),
            _ => Err("Invalid user type".to_string()),
        }
    }
}

pub fn form(user_type: UserType) -> Markup {
    let langs = vec![LanguageCompetency::german(), LanguageCompetency::english()];
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

            input
                type="hidden"
                name="user-type"
                value={(user_type.code())}
            {}

            select
                name="language"
                value="german"
            {
                @for lang in langs {
                    option
                        value={(lang.code())} { (lang.name()) }
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
