use maud::{html, Markup, PreEscaped};
use onboarding_form::UserType;
use rocket::response::content;

mod body;
mod footer;
mod header;
pub mod onboarding_form;

#[get("/?<user_type>")]
pub fn index(user_type: Option<String>) -> content::RawHtml<String> {
    let user_type = user_type.unwrap_or("sitter".to_string());
    let user_type = UserType::try_from(user_type).unwrap_or(UserType::Sitter);

    let raw = page(html! {
        div hx-get="/refresh" hx-trigger="every 10000ms" {}
        ({body::body(user_type)})
        ({footer::footer()})
    })
    .into_string();
    content::RawHtml(raw)
}

const PICO: &str = r#"<link rel="stylesheet" href="_assets/pico.min.css">"#;
const CSS: &str = r#"<link rel="stylesheet" href="_assets/app.css">"#;
const HTMX: &str = r#"<script src="/_assets/htmx.min.js"></script>"#;
const REFRESH: &str = r#"<script src="/_assets/refresh.js"></script>"#;

pub fn page(markup: Markup) -> Markup {
    html! {
       html {

            head {
                ({scripts()})
                ({title("Welcome to me")})
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
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}
