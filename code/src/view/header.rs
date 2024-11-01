use maud::{html, Markup};

pub fn header() -> Markup {
    html! {
        header {
            p { "header" }
        }
    }
}
