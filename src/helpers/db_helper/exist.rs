use diesel::{sql_query, RunQueryDsl};

use crate::db_connection;
use crate::db_connection::DB_POOL;

use diesel::sql_types::Bool;

#[derive(QueryableByName)]
struct Exists {
    #[sql_type = "Bool"]
    exists: bool,
}

pub fn exist(table: String, query: String) -> bool {
    let conn = db_connection::pg_pool_handler(&DB_POOL).unwrap();
    let sql = format!(
        "\
SELECT EXISTS(SELECT 1 FROM {} WHERE {});
  ",
        table, query
    );

    sql_query(sql)
        .load::<Exists>(&conn)
        .unwrap_or(vec![Exists { exists: false }])
        .pop()
        .unwrap_or(Exists { exists: false })
        .exists
}
