use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::members;

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "members"]
pub struct Member {
    pub id: Uuid,
    pub email: String,
    pub nickname: String,
    pub bio: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
