
use maud::{html, Markup};

use crate::view::onboarding_form;

pub fn body() -> Markup {
    html! {
        body {
            main .container {
                {(onboarding_form::form())}
            }
        }
    }
}
