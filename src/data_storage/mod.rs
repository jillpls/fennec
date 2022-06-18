extern crate rocket_db_pools;
extern crate sqlx;

use rocket_db_pools as rdp;
use rocket_db_pools::Database;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::Executor;

pub struct QueryResult {}

impl From<SqliteQueryResult> for QueryResult {
    fn from(_: SqliteQueryResult) -> Self {
        todo!()
    }
}

pub async fn insert_user<'a, T: 'a>(
    login: &String,
    hash: &String,
    conn: &mut T,
) -> Result<QueryResult, sqlx::Error>
where
    &'a mut T: Executor<'a>,
{
    let query = "
insert into users
    ( login, hash, activated )
values
    ($1, $2, 1)";

    match sqlx::query(&*query)
        .bind(login)
        .bind(hash)
        .execute(conn)
        .await
    {
        Ok(qr) => Ok(qr.into()),
        Err(err) => Err(err),
    }
}