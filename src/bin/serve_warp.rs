use rust_blog::server::the_blog;

#[tokio::main]
async fn main() {
    let repo = rust_blog::repo::PostRepo::new(rust_blog::establish_pool());

    let the_blog = the_blog(repo);

    warp::serve(the_blog)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

