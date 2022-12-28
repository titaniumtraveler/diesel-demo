use diesel_demo::DBConnection;
use std::env::args;

fn main() {
    let mut db = DBConnection::establish_connection();

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    db.publish_post_by_id(id);
}
