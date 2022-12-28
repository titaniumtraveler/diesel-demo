use async_diesel_demo::AsyncDBConnection;
use std::env::args;

#[tokio::main]
async fn main() {
    let mut db = AsyncDBConnection::establish_connection().await;

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    db.publish_post_by_id(id).await;
}
