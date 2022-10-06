use rust_blog::{models::NewPost, repo::Repo};
use std::io::{stdin, Read};

fn main() {
    let repo = rust_blog::repo::PostRepo::new(rust_blog::establish_pool());

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    title = title.trim_end().to_string(); // Remove the trailing newline

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let new_post = NewPost { title, body };

    let post = repo.create(new_post).unwrap();
    println!("\nSaved draft {} with id {}", post.title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
