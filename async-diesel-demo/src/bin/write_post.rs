use async_diesel_demo::*;
use std::io::{stdin, stdout, Read, Write};

#[tokio::main]
async fn main() {
    let mut db = AsyncDBConnection::establish_connection().await;

    let mut title = String::new();
    let mut body = String::new();

    print!("What would you like your title to be? ");
    stdout().flush().expect("Failed writing");
    stdin().read_line(&mut title).unwrap();

    println!("Ok! Let's write \x1B[1;31m{}\x1B[0m", title.trim_end());
    stdin().read_to_string(&mut body).unwrap();

    let post = db.create_post(title.trim_end(), &body).await;
    println!("\nSaved draft {} with id {}", title, post.id);
}
