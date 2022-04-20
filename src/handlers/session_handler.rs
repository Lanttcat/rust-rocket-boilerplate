use rand::rngs::OsRng;
use rand::Rng;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use tokio::task::spawn_blocking;

use crate::domains::member::repository as member_repo;
use crate::domains::session::dto::{AuthRequest, SessionRequest};
use crate::domains::session::service as session_service;

#[post("/sessions", data = "<body>")]
pub async fn create_request(body: Json<SessionRequest>) -> Status {
    let email = body.clone().email;
    let member = member_repo::get_by_email(email).unwrap();

    let code: i32 = OsRng.gen_range(0..1000000);

    spawn_blocking(move || {
        session_service::code_notify(member.email.clone(), format!("{:0>6}", code))
    });
    session_service::create_or_update_session(member.id, &format!("{:0>6}", code));

    Status::Created
}

#[post("/auth", data = "<body>")]
pub async fn auth_request(cookies: &CookieJar<'_>, body: Json<AuthRequest>) -> Status {
    let body_value = body.clone();
    let data = session_service::create_session_code(&body_value.email, &body_value.code);

    match data {
        Some(m) => {
            cookies.add_private(Cookie::new("session_code", m.session_code.to_string()));
            cookies.add_private(Cookie::new("member_id", m.member_id.to_string()));
            Status::Ok
        }
        _ => Status::NotFound,
    }
}
