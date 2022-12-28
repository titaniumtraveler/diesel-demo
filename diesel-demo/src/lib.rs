use crate::models::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub struct DBConnection(PgConnection);

impl DBConnection {
    pub fn establish_connection() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self(
            PgConnection::establish(&database_url).unwrap_or_else(|error| {
                panic!("Error connecting to {} with {error}", database_url)
            }),
        )
    }

    pub fn connection(&mut self) -> &mut PgConnection {
        &mut self.0
    }

    pub fn show_posts(&mut self) -> Vec<Post> {
        use self::schema::posts::dsl::*;

        posts
            .filter(published.eq(true))
            .limit(5)
            .load::<Post>(self.connection())
            .expect("Error loading posts")
    }

    pub fn create_post(&mut self, title: &str, body: &str) -> Post {
        use crate::schema::posts;

        diesel::insert_into(posts::table)
            .values(&NewPost { title, body })
            .get_result(&mut self.0)
            .expect("Error saving new post")
    }

    pub fn publish_post_by_id(&mut self, id: i32) -> Post {
        use self::schema::posts::dsl::{posts, published};

        diesel::update(posts.find(id))
            .set(published.eq(true))
            .get_result::<Post>(self.connection())
            .expect("Failed to publish post")
    }

    pub fn delete_posts_by_title(&mut self, pattern: &str) -> usize {
        use self::schema::posts::dsl::*;

        diesel::delete(posts.filter(title.like(pattern)))
            .execute(self.connection())
            .expect("Failed to delete posts")
    }
}
