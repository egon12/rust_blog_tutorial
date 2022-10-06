use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::{insert_into, update};
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use crate::models::{Post, NewPost, Error};
use crate::schema::posts::dsl::{posts, published};
use crate::schema::posts::table;

#[derive(Clone)]
pub struct PostRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

pub trait Repo {
    fn conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error>;

    fn create(&self, new_post: NewPost) -> Result<Post, Error> {
        insert_into(table)
            .values(&new_post)
            .get_result(&mut self.conn()?)
            .map_err(|e| e.into())
    }

    fn list(&self) -> Result<Vec<Post>, Error> {
        posts
            .filter(published.eq(true))
            .limit(5)
            .load::<Post>(&mut self.conn()?)
            .map_err(|e| e.into())
    }

    fn show(&self, id: i32) -> Result<Post, Error> {
        posts
            .filter(published.eq(true))
            .find(id)
            .first(&mut self.conn()?)
            .map_err(|e| e.into())
    }

    fn publish(&self, id: i32) -> Result<Post, Error> {
        update(posts.find(id))
            .set(published.eq(true))
            .get_result(&mut self.conn()?)
            .map_err(|e| e.into())
    }
}

impl Repo for PostRepo {
    fn conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        self.pool.get().map_err(|e| e.into())
    }
}

impl PostRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> PostRepo {
        PostRepo { pool }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satu() {
        assert_eq!("1", "1");
    }

    #[test]
    fn test_new() {
        let database_url = "postgres://postgres@127.0.0.1";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool.");
        let _repo = PostRepo::new(pool);
    }
}

