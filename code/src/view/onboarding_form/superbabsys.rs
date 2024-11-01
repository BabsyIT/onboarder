use maud::{html, Markup};
use rocket::{form::Form, http::Status, response::content::RawHtml, State};

use crate::{
    persistence::{self, SuperBabsys},
    superbabsys::SuperBabsy,
};

#[derive(FromForm)]
pub struct DateWindow<'r> {
    date: &'r str,
    monday: Option<&'r str>,
    tuesday: Option<&'r str>,
    wednesday: Option<&'r str>,
    thursday: Option<&'r str>,
    friday: Option<&'r str>,
    saturday: Option<&'r str>,
    from: &'r str,
    to: &'r str,
}

#[post("/employees", data = "<date_window>")]
pub fn get_superbabsys(
    date_window: Form<DateWindow<'_>>,
    super_babsys: &State<SuperBabsys>,
) -> RawHtml<String> {
    let super_babsys = super_babsys.get_super_babsys();

    RawHtml(super_babsys_html(super_babsys).into_string())
}

pub fn super_babsys_html(super_babsys: Vec<SuperBabsy>) -> Markup {
    html! {}
}
