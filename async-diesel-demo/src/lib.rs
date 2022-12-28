use crate::models::{NewPost, Post};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub struct AsyncDBConnection(AsyncPgConnection);

impl AsyncDBConnection {
    pub async fn establish_connection() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self(
            AsyncPgConnection::establish(&database_url)
                .await
                .unwrap_or_else(|error| {
                    panic!("Error connecting to {} with {error}", database_url)
                }),
        )
    }

    pub fn connection(&mut self) -> &mut AsyncPgConnection {
        &mut self.0
    }

    pub async fn show_posts(&mut self) -> Vec<Post> {
        use self::schema::posts::dsl::*;

        posts
            .filter(published.eq(true))
            .limit(5)
            .load::<Post>(self.connection())
            .await
            .expect("Error loading posts")
    }

    pub async fn create_post(&mut self, title: &str, body: &str) -> Post {
        use crate::schema::posts;

        insert_into(posts::table)
            .values(&NewPost { title, body })
            .get_result(&mut self.0)
            .await
            .expect("Error saving new post")
    }

    pub async fn publish_post_by_id(&mut self, id: i32) -> Post {
        use self::schema::posts::dsl::{posts, published};

        update(posts.find(id))
            .set(published.eq(true))
            .get_result::<Post>(self.connection())
            .await
            .expect("Failed to publish post")
    }

    pub async fn delete_posts_by_title(&mut self, pattern: &str) -> usize {
        use self::schema::posts::dsl::*;

        delete(posts.filter(title.like(pattern)))
            .execute(self.connection())
            .await
            .expect("Failed to delete posts")
    }
}
