use async_diesel_demo::AsyncDBConnection;
use std::env::args;

#[tokio::main]
async fn main() {
    let mut db = AsyncDBConnection::establish_connection().await;

    let pattern = args().nth(1).expect("Expected a target to match against");

    let num_deleted = db.delete_posts_by_title(&pattern).await;
    println!("Deleted {} posts", num_deleted);
}
