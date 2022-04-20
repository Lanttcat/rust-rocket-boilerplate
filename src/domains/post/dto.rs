use rocket::serde::{Deserialize, Serialize};
use serde::{de, Deserializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPostRequest {
    pub title: String,
    pub description: String,
    pub tags: Vec<i32>,
}

#[derive(Debug, PartialEq, FromFormField)]
pub enum VoteType {
    Up,
    Down,
}

#[derive(Deserialize)]
pub struct PostVoteRequest {
    pub vote_type: VoteType,
}

impl<'de> Deserialize<'de> for VoteType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        let state = match s.as_str() {
            "up" => VoteType::Up,
            "down" => VoteType::Down,
            other => {
                return Err(de::Error::custom(format!("Invalid state '{}'", other)));
            }
        };
        Ok(state)
    }
}
