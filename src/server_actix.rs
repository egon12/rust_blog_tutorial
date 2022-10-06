use crate::models::{NewPost, PostView};
use crate::repo::{PostRepo, Repo};
use actix_web::{get, patch, post, web, web::Data, web::Path, HttpResponse, Responder};
use tera::{Context, Tera};

pub fn service() -> actix_web::Scope {
    web::scope("").service(index).service(signup).service(signin)
}

#[get("/")]
async fn index(tera: Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "My Blog");
    context.insert("name", "World");
    let s = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(s)
}

#[get("/signup")]
async fn signup(tera: Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/signin")]
async fn signin(tera: Data<Tera>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn blog_web_service() -> actix_web::Scope {
    web::scope("blog").service(blog_web_index).service(blog_web_show)
}

#[get("")]
async fn blog_web_index(tera: Data<Tera>, repo: Data<PostRepo>) -> impl Responder {
    let posts = repo.get_ref().list().unwrap();
    let posts: Vec<PostView> = posts.into_iter().map(|p| p.into()).collect();
    let mut context = Context::new();
    context.insert("title", "My Blog");
    context.insert("posts", &posts);
    let s = tera.render("blog/index.html", &context).unwrap();
    HttpResponse::Ok().body(s)
}

#[get("/{id}")]
async fn blog_web_show(id: Path<(i32,)>, tera: Data<Tera>, repo: Data<PostRepo>) -> impl Responder {
    //std::thread::sleep(std::time::Duration::from_secs(1));
    let post = repo.get_ref().show(id.0).unwrap();
    let post: PostView = post.into();
    let mut context = Context::new();
    let title = "My Blog | ".to_string() + &post.title;
    context.insert("title", &title);
    context.insert("post", &post);
    let s = tera.render("blog/show.html", &context).unwrap();
    HttpResponse::Ok().body(s)
}


pub fn blog_service() -> actix_web::Scope {
    web::scope("api/blog")
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
