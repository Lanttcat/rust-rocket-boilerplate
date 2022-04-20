use diesel::{QueryDsl, RunQueryDsl};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use uuid::Uuid;

use crate::db_connection::{pg_pool_handler, PgPool};
use crate::domains::member::member::Member;
use crate::schema::members::dsl::members;

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Member {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Member, Self::Error> {
        let db_pool = request.guard::<&State<PgPool>>().await.unwrap();

        let conn = pg_pool_handler(db_pool).unwrap();

        match request.cookies().get_private("member_id") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(member_id_cookie) => {
                let member_id = member_id_cookie.value().parse::<Uuid>().unwrap();
                let maybe_member = members.find(member_id).first::<Member>(&conn).ok();

                match maybe_member {
                    Some(member) => Outcome::Success(member),
                    None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
                }
            }
        }
    }
}
