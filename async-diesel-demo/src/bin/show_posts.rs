use async_diesel_demo::AsyncDBConnection;

#[tokio::main]
async fn main() {
    let mut db = AsyncDBConnection::establish_connection().await;

    let results = db.show_posts().await;

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
