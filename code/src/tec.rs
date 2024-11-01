use std::sync::atomic::AtomicBool;

use rocket::{http::Header, Build, Rocket, State};

pub struct Refresh {
    semiphore: AtomicBool,
}

#[derive(Responder)]
pub struct HXResponder<T> {
    inner: T,
    header: Header<'static>,
}

#[get("/refresh")]
pub fn refresh(semiphore: &State<Refresh>) -> HXResponder<&'static str> {
    let fresh = semiphore
        .semiphore
        .load(std::sync::atomic::Ordering::Relaxed);
    if fresh {
        println!("refreshing");
        semiphore
            .semiphore
            .swap(false, std::sync::atomic::Ordering::Relaxed);
        HXResponder {
            inner: "",
            header: HXHeader("refresh".to_string()).into(),
        }
    } else {
        println!("not-refreshing");
        HXResponder {
            inner: "",
            header: HXHeader("not-refresh".to_string()).into(),
        }
    }
}

struct HXHeader(String);

impl From<HXHeader> for Header<'static> {
    #[inline(always)]
    fn from(hx: HXHeader) -> Self {
        Header::new("HX-Trigger", hx.0)
    }
}

pub fn mount_tec(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![refresh]).manage(Refresh {
        semiphore: AtomicBool::new(true),
    })
}
