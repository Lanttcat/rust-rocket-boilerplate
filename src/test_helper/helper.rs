use chrono::Utc;
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::db_connection;
use crate::db_connection::DB_POOL;
use crate::domains::member::member::Member;
use crate::domains::session::domain::Auth;
use crate::schema::auths::dsl::auths;
use crate::schema::members::dsl::members;

pub fn mock_member(email: Option<String>) -> (Member, Uuid) {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    clear_member();

    let new_uuid = Uuid::new_v4();
    let new_session_code = Uuid::new_v4();

    let new_email = match email {
        None => "test@email.com".to_string(),
        Some(v) => v.to_string(),
    };

    let mock_new_member = Member {
        id: new_uuid,
        email: new_email.to_string(),
        nickname: "test_user_1".to_string(),
        bio: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let mock_new_auth = Auth {
        member_id: new_uuid,
        code: Option::from("member_password".to_string()),
        session_code: Option::from(new_session_code.to_string()),
        created_at: Utc::now().naive_utc(),
        active_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(members)
        .values(&mock_new_member)
        .execute(&conn)
        .unwrap();
    diesel::insert_into(auths)
        .values(&mock_new_auth)
        .execute(&conn)
        .unwrap();

    (mock_new_member, new_session_code)
}

pub fn clear_member() {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    diesel::delete(auths).execute(&conn).unwrap();
    diesel::delete(members).execute(&conn).unwrap();
}
