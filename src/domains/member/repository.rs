use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db_connection;
use crate::db_connection::DB_POOL;
use crate::domains::member::member::Member;
use crate::schema::members;
use crate::schema::members::{bio as column_bio, email, id as column_id};

pub fn get_by_id(id: Uuid) -> Member {
    let connection = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    members::table
        .find(id)
        .first::<Member>(&connection)
        .unwrap()
}

pub fn get_by_email(input_email: String) -> Option<Member> {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    members::table
        .filter(email.eq(input_email))
        .first::<Member>(&conn)
        .ok()
}

pub fn insert(new_member: &Member) {
    let connection = db_connection::pg_pool_handler(&DB_POOL).unwrap();

    diesel::insert_into(members::table)
        .values(new_member)
        .execute(&connection)
        .unwrap();
}
pub fn update(id: Uuid, bio: String) {
    let connection = db_connection::pg_pool_handler(&DB_POOL).unwrap();

    diesel::update(members::table)
        .filter(column_id.eq(id))
        .set(column_bio.eq(bio))
        .execute(&connection)
        .unwrap();
}
