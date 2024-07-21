use diesel::prelude::*;

pub fn get_result<U>(
    conn: &mut diesel::SqliteConnection,
    query: impl RunQueryDsl<SqliteConnection>,
) -> QueryResult<U> {
    query.get_result(conn)
}

fn main() {}
