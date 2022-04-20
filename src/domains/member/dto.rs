use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domains::member::member::Member;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberInput {
    pub email: String,
    pub nickname: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberUpdateInput {
    pub bio: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BaseMemberOutput {
    pub id: Uuid,
    pub nickname: String,
}

#[derive(Serialize)]
pub struct MemberOutput {
    pub id: Uuid,
    pub email: String,
    pub nickname: String,
    pub bio: Option<String>,
}

impl Member {
    pub fn to_base_output(&self) -> BaseMemberOutput {
        BaseMemberOutput {
            id: self.id,
            nickname: self.nickname.to_string(),
        }
    }

    pub fn to_output(&self) -> MemberOutput {
        MemberOutput {
            id: self.id,
            email: self.email.to_string(),
            nickname: self.nickname.to_string(),
            bio: self.bio.to_owned(),
        }
    }
}
