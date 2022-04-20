use rocket::http::Status;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::domains::member::member::Member;
use crate::domains::post::domain::Post;
use crate::domains::post::dto::{NewPostRequest, PostVoteRequest};
use crate::domains::post::repository as post_repo;
use crate::domains::post::service as post_service;

#[get("/posts/<id>")]
pub fn get_request(_member: Member, id: Uuid) -> Json<Post> {
    let post = post_repo::get_by_id(id);
    Json::from(post)
}

#[post("/posts", data = "<body>")]
pub fn create_post_request(current_member: Member, body: Json<NewPostRequest>) -> Status {
    post_service::create_new_post(body.clone(), current_member.id);
    Status::Created
}

#[post("/posts/<id>", data = "<body>")]
pub fn vote_post_request(current_member: Member, id: Uuid, body: Json<PostVoteRequest>) -> Status {
    let new_body = PostVoteRequest {
        vote_type: body.0.vote_type,
    };
    post_service::post_vote(id, new_body, current_member.id);
    Status::Created
}
