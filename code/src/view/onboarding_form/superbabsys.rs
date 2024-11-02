use chrono::{Datelike, NaiveTime};
use chrono::{Duration, NaiveDate, Weekday};
use maud::{html, Markup};
use rocket::{form::Form, http::Status, response::content::RawHtml, State};

use crate::{
    persistence::{self, SuperBabsys},
    superbabsys::SuperBabsy,
};

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
}

#[post("/employees", data = "<date_window>")]
pub fn get_superbabsys(
    date_window: Form<DateWindow<'_>>,
    super_babsys: &State<SuperBabsys>,
) -> RawHtml<String> {
    let super_babsys = super_babsys.get_super_babsys();
    let only_available = only_available(super_babsys, date_window.into_inner());

    RawHtml(super_babsys_html(only_available).into_string())
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

fn only_available(super_babsys: Vec<SuperBabsy>, date_window: DateWindow) -> Vec<SuperBabsy> {
    let from_date = NaiveDate::parse_from_str(date_window.date, "%Y-%m-%d").unwrap();

    let mondays = if date_window.monday.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Mon)
    } else {
        vec![]
    };
    
    let tuesdays = if date_window.tuesday.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Tue)
    } else {
        vec![]
    };
    
    let wednesdays = if date_window.wednesday.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Wed)
    } else {
        vec![]
    };
    
    let thursdays = if date_window.thursday.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Thu)
    } else {
        vec![]
    };
    
    let fridays = if date_window.friday.is_some() {
        generate_date_window(from_date, from_date + Duration::weeks(3), Weekday::Fri)
    } else {
        vec![]
    };
    
    let saturdays = if date_window.saturday.is_some() {
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

    super_babsys.into_iter().filter(|babsy| 
        user_availability.iter().any(|date| {
           let res = babsy.is_available(*date);
           println!("{} is available on {} {}", babsy.name, date, res);
           res
        }) 
    ).collect()
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
        current_date = current_date + Duration::days(1);
    }

    dates
}
