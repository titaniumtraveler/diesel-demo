use diesel_demo::DBConnection;

fn main() {
    let mut db = DBConnection::establish_connection();

    let results = db.show_posts();

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
