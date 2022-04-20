use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::auths;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Insertable, AsChangeset, Serialize, Deserialize,
)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "auths"]
pub struct Auth {
    pub member_id: Uuid,
    pub code: Option<String>,
    pub session_code: Option<String>,
    pub created_at: NaiveDateTime,
    pub active_at: NaiveDateTime,
}
