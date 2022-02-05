#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{
    fs::FileServer,
    request::{self, Request, FromRequest},
    response::Redirect,
    State,
};

const HTTP_HOST: &str = "sa.m-h.ug";

struct HostCheck{}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostCheck {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("Host") {
            Some(HTTP_HOST) => request::Outcome::Success(HostCheck{}),
            _ => request::Outcome::Forward(()),
        }
    }
}

#[get("/", rank = 1)]
fn hit(hit_count: &State<HitCount>, _host: HostCheck) -> String {
    let prev_count = hit_count.count.fetch_add(1, Ordering::Relaxed);
    format!("Number of visits: {}", prev_count + 1)
}

#[get("/", rank = 20)]
fn redirect() -> Redirect {
    Redirect::to(format!("https://{}", HTTP_HOST))
}

struct HitCount {
    count: AtomicUsize,
}
impl HitCount {
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(HitCount::new())
        .mount("/hit", routes![hit, redirect])
        .mount(
            "/",
            FileServer::from("/www/public").rank(10),
        )
        //.mount("/", routes![redirect])
}
