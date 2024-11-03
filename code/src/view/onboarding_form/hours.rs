use std::collections::HashMap;

use chrono::{Datelike, IsoWeek, NaiveDateTime, Weekday};
use maud::{html, Markup};
use rocket::{form::Form, response::content::RawHtml, State};

use crate::{persistence::super_babsys::SuperBabsys, superbabsys::SuperBabsy};

#[derive(FromForm)]
pub struct CurrentDate<'r> {
    //the date they are available from
    date: &'r str,
    // the super babsy id
    id: &'r str,
    // user type
    user_type: &'r str,
}

#[post("/employees/hours", data = "<current_date>")]
pub fn hours_view_html(
    current_date: Form<CurrentDate<'_>>,
    super_babsys: &State<SuperBabsys>,
) -> RawHtml<String> {
    let from_date = NaiveDateTime::parse_from_str(current_date.date, "%Y-%m-%d %H:%M:%S").unwrap();
    let week_index = from_date.iso_week();

    let Some(super_babsy) = super_babsys.get_super_babsy(current_date.id) else {
        return RawHtml(
            html! {
                p { "Super Babsy not found" }
            }
            .into_string(),
        );
    };

    let raw = html! {
        ({hours_view(
            super_babsy.clone(),
            week_index,
            super_babsy.get_available_hours(from_date),
            current_date.user_type.to_string())}
        );
    }
    .into_string();
    RawHtml(raw)
}

pub fn hours_view(
    super_babsy: SuperBabsy,
    week_index: IsoWeek,
    hours: Vec<NaiveDateTime>,
    user_type: String,
) -> Markup {
    let mut week_map: HashMap<Weekday, Vec<NaiveDateTime>> = HashMap::new();


    hours.iter().for_each(|h| {
        let week = h.iso_week();
        let day = h.weekday();

        if week == week_index {
            if let std::collections::hash_map::Entry::Vacant(e) = week_map.entry(day) {
                e.insert(vec![*h]);
            } else {
                week_map.get_mut(&day).unwrap().push(*h);
            }
        }
    });

    let mut entries = week_map
        .drain()
        .collect::<Vec<(Weekday, Vec<NaiveDateTime>)>>();

    entries.sort_by(|entry, other| entry.1.first().unwrap().cmp(other.1.first().unwrap()));

    html! {
        div #main-content .grid {
            @for (day, hours) in entries {
                article {
                    header {
                        h4 { (format!("{}, {}", weekday_to_string(day), hours.first().unwrap().date().format("%m/%d") ))   }
                    }
                    body{
                        ul {
                            @for hour in hours {


                                    form
                                        hx-post="/booking"
                                        hx-target="#main-content"
                                        hx-swap="outerHTML"


                                    {
                                        input type="hidden" name="id" value=(super_babsy.id) {};
                                        input type="hidden" name="date" value=(hour.format("%Y-%m-%d %H:%M:%S")) {};
                                        input type="hidden" name="user_type" value=(user_type) {};
                                        button
                                            type="submit"
                                            ."hour-button"
                                                { (hour.time().format("%H:%M")) }
                                    }
                            }
                        }
                    }

                }
            }
        }
    }
}

fn weekday_to_string(weekday: Weekday) -> String {
    match weekday {
        Weekday::Mon => "Mo",
        Weekday::Tue => "Tu",
        Weekday::Wed => "We",
        Weekday::Thu => "Th",
        Weekday::Fri => "Fr",
        Weekday::Sat => "Sa",
        Weekday::Sun => "Su",
    }
    .to_string()
}
