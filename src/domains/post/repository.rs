use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db_connection;
use crate::db_connection::DB_POOL;
use crate::domains::post::domain::Post;
use crate::schema::posts;
use crate::schema::posts::{anti_votes_received, id as column_post_id, votes_received};

pub fn get_by_id(id: Uuid) -> Post {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    posts::table.find(id).first::<Post>(&conn).unwrap()
}

pub fn insert(new_post: Post) {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    diesel::insert_into(posts::table)
        .values(new_post)
        .on_conflict_do_nothing()
        .execute(&conn)
        .unwrap();
}

pub fn update_post_vote(
    id: Uuid,
    votes_received_input: &Vec<Uuid>,
    anti_votes_received_input: &Vec<Uuid>,
) {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();

    diesel::update(posts::table.filter(column_post_id.eq(id)))
        .set((
            votes_received.eq(votes_received_input),
            anti_votes_received.eq(anti_votes_received_input),
        ))
        .execute(&conn)
        .unwrap();
}
