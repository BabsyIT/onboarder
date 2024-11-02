use maud::html;
use rocket::response::content::RawHtml;

use crate::view::page;

#[get("/fakeapp")]
pub fn fake_app() -> RawHtml<String> {
    let raw = page(
        "We are in the app I promise".to_string(),
        html! {
            ul {
                li{
                    a href="/?user_type=sitter" { "Sitter" }
                }
                li{
                     a href="/?user_type=parent" { "Parent" }
                }
            }
        },
    )
    .into_string();
    RawHtml(raw)
}
