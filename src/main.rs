use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, AsyncConnection, RunQueryDsl};

mod schema {
    diesel::table! {
        anything (id) {
            id -> Int4,
            created -> Timestamptz,
        }
    }
}

#[tokio::main]
async fn main() {
    let conn = AsyncPgConnection::establish("").await.expect("success");
    hello_world(conn).await;
}

async fn hello_world(conn: AsyncPgConnection) {
    use schema::anything;

    diesel::update(anything::table)
        .set(anything::created.eq(diesel::dsl::now))
        .execute(&mut conn)
        .await
        .expect("no issues");
}

