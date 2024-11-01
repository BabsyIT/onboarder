use maud::{html, Markup, PreEscaped};
use rocket::response::content;

mod header;
mod body;
mod footer;
mod onboarding_form;

#[get("/")]
pub fn index() -> content::RawHtml<String> {
    let raw = page(html! {
        div hx-get="/refresh" hx-trigger="every 300ms" {}
        ({body::body()})
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
