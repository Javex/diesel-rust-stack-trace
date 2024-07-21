use diesel::prelude::*;

#[derive(diesel::MultiConnection)]
pub enum DbConnection {
    Sqlite(diesel::SqliteConnection),
}

impl DbConnection {
    pub fn get_result<U>(&mut self, query: impl RunQueryDsl<DbConnection>) -> QueryResult<U> {
        match self {
            DbConnection::Sqlite(conn) => query.get_result(conn),
        }
    }
}

pub fn connect_sqlite(db: &str) -> DbConnection {
    let conn = SqliteConnection::establish(&db).unwrap();
    DbConnection::Sqlite(conn)
}

pub fn establish_connection() -> DbConnection {
    let db = "test.db";
    connect_sqlite(db)
}
fn main() {
    establish_connection();
}
