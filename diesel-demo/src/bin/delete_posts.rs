use diesel_demo::DBConnection;
use std::env::args;

fn main() {
    let mut db = DBConnection::establish_connection();

    let pattern = args().nth(1).expect("Expected a target to match against");

    db.delete_posts_by_title(&pattern);
}
