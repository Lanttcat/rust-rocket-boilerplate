use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::post_tags;
use crate::schema::posts;

#[derive(PartialEq, Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub votes_received: Vec<Uuid>,
    pub anti_votes_received: Vec<Uuid>,
    pub tags: Vec<i32>,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(PartialEq, Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "post_tags"]
pub struct PostTag {
    pub id: i32,
    pub content: String,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
}
