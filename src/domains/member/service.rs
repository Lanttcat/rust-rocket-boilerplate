use chrono::Utc;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::domains::member::dto::{MemberInput, MemberOutput, MemberUpdateInput};
use crate::domains::member::member::Member;
use crate::domains::member::repository as member_repo;

pub fn create_new_member(member_input: MemberInput) -> MemberOutput {
    let new_member = Member {
        id: Uuid::new_v4(),
        email: member_input.email.trim().to_string(),
        nickname: member_input.nickname.trim().to_string(),
        bio: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    member_repo::insert(&new_member);
    new_member.to_output()
}

pub fn update_profile(id: Uuid, body: Json<MemberUpdateInput>) {
    member_repo::update(id, body.bio.to_string())
}
