use actix_web::{middleware, web::Data, App, HttpServer};
use dotenvy::dotenv;
use rust_blog::{establish_pool, repo::PostRepo, server_actix::service};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    HttpServer::new(|| {
        let pool = establish_pool();
        let repo = PostRepo::new(pool);
        let data = Data::new(repo);
        let service = service();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(data)
            .service(service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
