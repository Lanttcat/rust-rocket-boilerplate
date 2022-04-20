use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db_connection;
use crate::db_connection::DB_POOL;
use crate::domains::session::domain::Auth;
use crate::schema::auths;
use crate::schema::auths::{
    code as column_code, member_id as column_member_id, session_code as column_session_code,
};

pub fn insert_or_update(new_record: Auth) {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    diesel::insert_into(auths::table)
        .values(&new_record)
        .on_conflict(column_member_id)
        .do_update()
        .set(&new_record)
        .execute(&conn)
        .unwrap();
}

pub fn update_session_code(member_id: Uuid, new_session_code: Uuid) {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    let target = auths::table.filter(column_member_id.eq(member_id));
    diesel::update(target)
        .set(column_session_code.eq(new_session_code.to_string()))
        .execute(&conn)
        .unwrap();
}

pub fn update_code(member_id: Uuid, code: Option<String>) {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    let target = auths::table.filter(column_member_id.eq(member_id));
    diesel::update(target)
        .set(column_code.eq(code))
        .execute(&conn)
        .unwrap();
}
