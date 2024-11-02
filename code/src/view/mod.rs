use maud::{html, Markup, PreEscaped};
use onboarding_form::UserType;
use rocket::response::content;

mod body;
pub mod dashboard;
mod footer;
pub mod onboarding_form;

#[get("/?<user_type>")]
pub fn index(user_type: Option<String>) -> content::RawHtml<String> {
    let user_type = user_type.unwrap_or("sitter".to_string());
    let user_type = UserType::try_from(user_type).unwrap_or(UserType::Sitter);

    let raw = page(
        "Babsy Onboarding".to_string(),
        html! {
            div hx-get="/refresh" hx-trigger="every 10000ms" {}
            ({body::body(user_type)})
            ({footer::footer()})
        },
    )
    .into_string();
    content::RawHtml(raw)
}

const PICO: &str = r#"<link rel="stylesheet" href="_assets/pico.min.css">"#;
const CSS: &str = r#"<link rel="stylesheet" href="_assets/app.css">"#;
const HTMX: &str = r#"<script src="/_assets/htmx.min.js"></script>"#;
const REFRESH: &str = r#"<script src="/_assets/refresh.js"></script>"#;
const FONTS: &str = r###"
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&display=swap" rel="stylesheet">"###;

pub fn page(titlex: String, markup: Markup) -> Markup {
    html! {
       html data-theme="light" ."ubuntu-regular"  {

            head {
                ({scripts()})
                ({title(titlex)})
            }

            body {
                (markup)
            }
        }
    }
}

fn scripts() -> Markup {
    html! {
       (PreEscaped(PICO))
       (PreEscaped(CSS))
       (PreEscaped(HTMX))
       (PreEscaped(REFRESH))
       (PreEscaped(FONTS))
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}
