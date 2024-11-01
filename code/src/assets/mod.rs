use rocket::{
    fs::{relative, FileServer, Options},
    response::content,
    Build, Rocket,
};

mod htmx;
mod pico;

fn assets() -> FileServer {
    FileServer::new(relative!("/assets"), Options::default())
}

#[get("/htmx.min.js")]
fn htmx_code() -> content::RawJavaScript<&'static str> {
    content::RawJavaScript(htmx::CODE)
}

#[get("/pico.min.css")]
fn pico_code() -> content::RawCss<&'static str> {
    content::RawCss(pico::CODE)
}

pub fn mount_assets(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/_assets", assets())
        .mount("/_assets", routes![htmx_code])
        .mount("/_assets", routes![pico_code])
}
