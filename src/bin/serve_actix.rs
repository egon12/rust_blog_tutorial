use actix_web::{middleware, web::Data, App, HttpServer};
use dotenvy::dotenv;
use rust_blog::{establish_pool, repo::PostRepo};
use rust_blog::server_actix::{service, blog_service, blog_web_service};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    HttpServer::new(|| {
        let pool = establish_pool();
        let repo = PostRepo::new(pool);
        let repo = Data::new(repo);
        let tera = Tera::new("templates/**/*").unwrap();
        let tera = Data::new(tera);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(tera)
            .app_data(repo)
            .service(blog_service())
            .service(blog_web_service())
            .service(service())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
