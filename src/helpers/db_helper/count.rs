use diesel::{sql_query, RunQueryDsl};

use crate::db_connection::PgPooledConnection;

use diesel::types::BigInt;

#[derive(QueryableByName)]
struct Count {
    #[sql_type = "BigInt"]
    count: i64,
}

pub fn row_count(sql: String, conn: PgPooledConnection) -> i64 {
    sql_query(sql)
        .load::<Count>(&conn)
        .unwrap_or(vec![Count { count: 0 }])
        .pop()
        .unwrap_or(Count { count: 0 })
        .count
}
