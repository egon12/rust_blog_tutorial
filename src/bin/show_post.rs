use rust_blog::repo::Repo;

fn main () {
    let repo = rust_blog::repo::PostRepo::new(rust_blog::establish_pool());

    let posts = repo.list().unwrap();

    println!("Displaying {} posts", posts.len());
    for post in posts {
        println!("{}. {}\n", post.id, post.title);
    }
}
