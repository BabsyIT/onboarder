use maud::{html, Markup};

use crate::view::onboarding_form;

use super::onboarding_form::UserType;

pub fn body(user_type: UserType) -> Markup {
    html! {
        body {
            main .container {
                div #main-content{
                    {(onboarding_form::form(user_type))}
                }
            }
        }
    }
}
