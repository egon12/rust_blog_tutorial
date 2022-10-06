use diesel::prelude::*;
use crate::schema::posts;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Serialize)]
pub struct PostView {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub link: String,
}

impl From<Post> for PostView {
    fn from(post: Post) -> Self {
        PostView {
            id: post.id,
            title: post.title,
            body: post.body,
            published: post.published,
            link: format!("/blog/{}", post.id),
        }
    }
}

//#[derive(Insertable, Deserialize)]
//#[diesel(table_name = posts)]
//pub struct NewPost<'a> {
//    pub title: &'a str,
//    pub body: &'a str,
//}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}


#[derive(Debug)]
pub enum Error {
    Unknown,
    BadRequest(String),
    SystemError(String),
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => Error::BadRequest("Not found".to_string()),
            _ => Error::SystemError(e.to_string()),
        }
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::SystemError(e.to_string())
    }
}

