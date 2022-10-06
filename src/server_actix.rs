use crate::models::NewPost;
use crate::repo::{PostRepo, Repo};
use actix_web::{get, patch, post, web, HttpResponse, Responder};

pub fn service() -> actix_web::Scope {
    web::scope("/blog")
        .service(blog)
        .service(blog_detail)
        .service(blog_create)
        .service(blog_publish)
}

#[get("")]
async fn blog(repo: web::Data<PostRepo>) -> impl Responder {
    HttpResponse::Ok().json(repo.get_ref().list().unwrap())
    //HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
async fn blog_detail(id: web::Path<(i32,)>, repo: web::Data<PostRepo>) -> impl Responder {
    HttpResponse::Ok().json(repo.get_ref().show(id.0).unwrap())
    //repo.get_ref().list().map(|posts| HttpResponse::Ok().json(posts)).unwrap()
}

#[post("")]
async fn blog_create(post: web::Json<NewPost>, repo: web::Data<PostRepo>) -> impl Responder {
    let post = repo.get_ref().create(post.into_inner()).unwrap();
    HttpResponse::Ok().json(post)
}

#[patch("/{id}/publish")]
async fn blog_publish(id: web::Path<(i32,)>, repo: web::Data<PostRepo>) -> impl Responder {
    repo.get_ref().publish(id.0).unwrap();
    HttpResponse::Ok()
}
