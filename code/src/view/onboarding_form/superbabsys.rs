use chrono::{Datelike, NaiveTime};
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
    #[field(name = "Monday")]
    monday: Option<&'r str>,
    #[field(name = "Tuesday")]
    tuesday: Option<&'r str>,
    #[field(name = "Wednesday")]
    wednesday: Option<&'r str>,
    #[field(name = "Thursday")]
    thursday: Option<&'r str>,
    #[field(name = "Friday")]
    friday: Option<&'r str>,
    #[field(name = "Saturday")]
    saturday: Option<&'r str>,
    //hours for example 8:00
    from: &'r str,
    //hours for example 21:00
    to: &'r str,
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
    let from = date_window.from;
    let to = date_window.to;
    let user_type = date_window.user_type;
    let language = date_window.language;
    let mon = date_window.monday.map(|f| f.to_string());
    let tue = date_window.tuesday.map(|f| f.to_string());
    let wed = date_window.wednesday.map(|f| f.to_string());
    let thu = date_window.thursday.map(|f| f.to_string());
    let fri = date_window.friday.map(|f| f.to_string());
    let sat = date_window.saturday.map(|f| f.to_string());

    let only_available = only_available(
        super_babsys,
        date.to_string(),
        from.to_string(),
        to.to_string(),
        mon,
        tue,
        wed,
        thu,
        fri,
        sat,
    );

    let only_capable = only_capable(only_available, user_type.to_string(), language.to_string());

    match only_capable {
        Ok(c) => RawHtml(super_babsys_html(c).into_string()),
        Err(e) => RawHtml(
            html! {
                p { (e) }
            }
            .into_string(),
        ),
    }
}

pub fn super_babsys_html(super_babsys: Vec<SuperBabsy>) -> Markup {
    html! {
         div id="employee-selection" {
        @for super_babsy in super_babsys {
                h2 { (super_babsy.name) }
                p { (super_babsy.description) }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn only_available(
    super_babsys: Vec<SuperBabsy>,
    date: String,
    from: String,
    to: String,
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
