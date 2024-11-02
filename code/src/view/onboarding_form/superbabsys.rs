use std::collections::HashMap;
use std::fmt::Pointer;

use chrono::{Datelike, IsoWeek, NaiveDateTime, NaiveTime};
use chrono::{Duration, NaiveDate, Weekday};
use maud::{html, Markup};
use rocket::{form::Form, http::Status, response::content::RawHtml, State};

use crate::superbabsys::LanguageCompetency;
use crate::{persistence::SuperBabsys, superbabsys::SuperBabsy};

use super::UserType;

#[derive(FromForm)]
pub struct DateWindow<'r> {
    //the date they are available from
    date: &'r str,
    // none or "on"
    #[field(name = "Mo")]
    monday: Option<&'r str>,
    #[field(name = "Tu")]
    tuesday: Option<&'r str>,
    #[field(name = "We")]
    wednesday: Option<&'r str>,
    #[field(name = "Th")]
    thursday: Option<&'r str>,
    #[field(name = "Fr")]
    friday: Option<&'r str>,
    #[field(name = "Sa")]
    saturday: Option<&'r str>,
    #[field(name = "user-type")]
    user_type: &'r str,
    language: &'r str,
}

#[post("/employees", data = "<date_window>")]
pub fn get_superbabsys(
    date_window: Form<DateWindow<'_>>,
    super_babsys: &State<SuperBabsys>,
) -> RawHtml<String> {
    let super_babsys = super_babsys.get_super_babsys();

    let date = date_window.date;
    let user_type = date_window.user_type;
    let language = date_window.language;
    let mon = date_window.monday.map(|f| f.to_string());
    let tue = date_window.tuesday.map(|f| f.to_string());
    let wed = date_window.wednesday.map(|f| f.to_string());
    let thu = date_window.thursday.map(|f| f.to_string());
    let fri = date_window.friday.map(|f| f.to_string());
    let sat = date_window.saturday.map(|f| f.to_string());

    let only_available =
        only_available(super_babsys, date.to_string(), mon, tue, wed, thu, fri, sat);

    let only_capable = only_capable(only_available, user_type.to_string(), language.to_string());

    let extended = format!("{} 00:00:00", date);
    let Ok(from_date) = NaiveDateTime::parse_from_str(&extended, "%Y-%m-%d %H:%M:%S") else {
        panic!("Could not parse date {}", date)
    };
    
    match only_capable {
        Ok(c) => RawHtml(super_babsys_html(c, from_date).into_string()),
        Err(e) => RawHtml(
            html! {
                p { (e) }
            }
            .into_string(),
        ),
    }
}

pub fn super_babsys_html(
    super_babsys: Vec<SuperBabsy>,
    from_date: NaiveDateTime,
) -> Markup {
    
    let from_date = from_date.format("%Y-%m-%d %H:%M:%S").to_string();
    
    html! {

            div id="employee-selection" {

           @for super_babsy in super_babsys {

               article {
                   header {
                       h2 { (super_babsy.name) }
                   }
                   body {
                       @if let Some(url) = super_babsy.image_url.clone() {
                              img ."face-image" src=(url) alt=(super_babsy.name.clone()) {}
                       }
                       p { (super_babsy.description.clone()) }
                   }
                   footer {
                     form hx-post="/employees/hours" hx-trigger="load delay:500ms" {
                         input type="hidden" name="id" value=(super_babsy.id) {}
                         
                         input type="hidden" name="date" value=(from_date) {}
                     }
                   }
               }
           }
       }
    }
}

#[derive(FromForm)]
pub struct CurrentDate<'r> {
    //the date they are available from
    date: &'r str,
    // the super babsy id
    id: &'r str,
}

#[post("/employees/hours", data = "<current_date>")]
pub fn hours_view_html(current_date: Form<CurrentDate<'_>>,  super_babsys: &State<SuperBabsys>,)-> RawHtml<String>{ 
    let from_date = NaiveDateTime::parse_from_str(&current_date.date, "%Y-%m-%d %H:%M:%S").unwrap();
    let week_index = from_date.iso_week();

    let Some(super_babsy) = super_babsys.get_super_babsy(current_date.id) else {
        return RawHtml(
            html! {
                p { "Super Babsy not found" }
            }
            .into_string(),
        );
    };  
    
    let raw = html!{
        ({hours_view(week_index, super_babsy.get_available_hours(from_date))});
    }.into_string();
    RawHtml(raw)
}

#[allow(clippy::too_many_arguments)]
fn only_available(
    super_babsys: Vec<SuperBabsy>,
    date: String,
    mon: Option<String>,
    tue: Option<String>,
    wed: Option<String>,
    thu: Option<String>,
    fri: Option<String>,
    sat: Option<String>,
) -> Vec<SuperBabsy> {
    let from_date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d").unwrap();

    let mondays = if mon.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Mon)
    } else {
        vec![]
    };

    let tuesdays = if tue.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Tue)
    } else {
        vec![]
    };

    let wednesdays = if wed.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Wed)
    } else {
        vec![]
    };

    let thursdays = if thu.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Thu)
    } else {
        vec![]
    };

    let fridays = if fri.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Fri)
    } else {
        vec![]
    };

    let saturdays = if sat.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Sat)
    } else {
        vec![]
    };

    let mut user_availability = mondays
        .into_iter()
        .chain(tuesdays)
        .chain(wednesdays)
        .chain(thursdays)
        .chain(fridays)
        .chain(saturdays)
        .collect::<Vec<NaiveDate>>();

    user_availability.dedup();
    println!("--------- User Availability ---------");
    println!("users: {user_availability:?}");

    super_babsys
        .into_iter()
        .filter(|babsy| {
            user_availability.iter().any(|date| {
                let res = babsy.is_available(*date);
                println!("{} is available on {} {}", babsy.name, date, res);
                res
            })
        })
        .collect()
}

fn generate_date_window(
    start_date: NaiveDate,
    end_date: NaiveDate,
    target_weekday: Weekday,
) -> Vec<NaiveDate> {
    let mut dates = Vec::new();
    let mut current_date = start_date;

    // Ensure the end_date does not exceed the year 4000
    let max_end_date = NaiveDate::from_ymd_opt(4000, 12, 31).unwrap();
    let final_end_date = if end_date > max_end_date {
        max_end_date
    } else {
        end_date
    };

    // Find the first target_weekday on or after the start_date
    while current_date <= final_end_date {
        if current_date.weekday() == target_weekday {
            dates.push(current_date);
        }
        current_date += Duration::days(1)
    }

    dates
}

pub fn only_capable(
    super_babsys: Vec<SuperBabsy>,
    user_type: String,
    lang: String,
) -> Result<Vec<SuperBabsy>, String> {
    let user_type = UserType::try_from(user_type)?;
    let lang = LanguageCompetency::try_from(lang)?;

    Ok(super_babsys
        .into_iter()
        .filter(|babsy| {
            match user_type {
                UserType::Sitter => {
                    if let Some(sitter) = babsy.get_sitter() {
                        return sitter.contains(&lang);
                    };
                }
                UserType::Parent => {
                    if let Some(parent) = babsy.get_parent() {
                        return parent.contains(&lang);
                    };
                }
            };

            false
        })
        .collect())
}

pub fn hours_view(week_index: IsoWeek, hours: Vec<NaiveDateTime>) -> Markup {
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

    let entries = week_map
        .drain()
        .collect::<Vec<(Weekday, Vec<NaiveDateTime>)>>();

    html! {
        @for (day, hours) in entries {
            div {
                h4 { (weekday_to_string(day)) (hours.first().unwrap().date().format("%m-%d")) }
                ul {
                    @for hour in hours {
                        button { (hour.time().format("%H:%M")) }
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