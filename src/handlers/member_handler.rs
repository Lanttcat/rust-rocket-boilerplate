use rocket::http::Status;
use rocket::response::status::Created;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

use crate::domains::member::dto::{MemberInput, MemberOutput, MemberUpdateInput};
use crate::domains::member::member::Member;
use crate::domains::member::repository as member_repo;
use crate::domains::member::service as member_service;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/members")]
pub async fn get_request(member: Member) -> Json<MemberOutput> {
    Json::from(member.to_output())
}

#[get("/members/<id>")]
pub async fn get_member_request(id: Uuid) -> Json<MemberOutput> {
    let member = member_repo::get_by_id(id);
    Json::from(member.to_output())
}

#[post("/members", data = "<member>")]
pub async fn create_request(member: Json<MemberInput>) -> Created<Json<MemberOutput>> {
    let member_value = member.clone();
    let new_member = member_service::create_new_member(member_value);

    Created::new("/").body(Json::from(new_member))
}

#[post("/members/<id>", data = "<body>")]
pub async fn update_request(id: Uuid, member: Member, body: Json<MemberUpdateInput>) -> Status {
    let member_value = member.clone();
    member_service::update_profile(member_value.id, body);

    Status::Ok
}

#[cfg(test)]
mod test_member {
    use rocket::http::{Cookie, Status};
    use rocket::local::asynchronous::Client;
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::rocket_builder;
    use crate::test_helper::helper::{clear_member, mock_member};

    #[derive(Deserialize)]
    struct MemberOutput {
        pub id: Uuid,
        pub email: String,
        pub nickname: String,
    }

    #[rocket::async_test]
    async fn should_get_member_info() {
        let (new_member, session_code) = mock_member(None);
        let client = Client::tracked(rocket_builder()).await.unwrap();
        let r1 = client
            .get("/api/members")
            .private_cookie(Cookie::new("session_code", session_code.to_string()))
            .private_cookie(Cookie::new("member_id", new_member.id.to_string()))
            .dispatch()
            .await;

        assert_eq!(r1.status(), Status::Ok);

        clear_member()
    }

    #[rocket::async_test]
    async fn should_get_bad_request_when_not_have_cookie() {
        let client = Client::tracked(rocket_builder()).await.unwrap();
        let r1 = client.get("/api/members").dispatch().await;

        assert_eq!(r1.status(), Status::BadRequest);
    }
}
