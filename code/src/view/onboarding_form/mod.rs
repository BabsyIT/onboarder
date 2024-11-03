use chrono::Utc;
use maud::{html, Markup};

use crate::superbabsys::LanguageCompetency;

pub mod booking;
pub mod hours;
pub mod new_booking;
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

    let days = vec!["Mo", "Tu", "We", "Th", "Fr", "Sa"];

    html! {

        img ."header-image" src="/_assets/cropped-babsy-logo.png"  {}

        form
                hx-post="/employees"
                hx-target="#employee-selection"
                hx-trigger="change from:input, change from:select, change from:checkbox"
                hx-swap="outerHTML"
        {

            label for="date" {"Ich bin verfügbar am oder nach / I'm available on or after"}
            input
                type="date"
                id="date"
                name="date"
                value={(formatted_time)}
                min={(formatted_time)} {};

            fieldset .container {
                @for day in days {
                    input
                        ."day-checkbox"
                        type="checkbox"
                        name={(day)} { (day) }
                }
            }

            hr{};

            label for="language" {"Sprache / Language"}
            select
                id="language"
                name="language"
                value="german"
            {
                @for lang in langs {
                    option
                        value={(lang.code())} { (lang.name()) }
                }

            }


            input
                type="hidden"
                name="user-type"
                value={(user_type.code())}
                {}
    }
         hr{};

         div id="employee-selection" {

         }
        }
}
