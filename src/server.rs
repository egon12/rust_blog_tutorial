use crate::repo::{PostRepo, Repo};
use warp::Filter;

pub fn the_blog(
    repo: PostRepo,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list(repo.clone())
        .or(show(repo.clone()))
        .or(publish(repo))
}

fn list(
    repo: PostRepo,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("blog").map(move || {
        let post = repo.clone().list().unwrap();
        warp::reply::json(&post)
    })
}

fn show(
    repo: PostRepo,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("blog" / i32).map(move |id: i32| {
        let post = repo.show(id).unwrap();
        warp::reply::json(&post)
    })
}

fn publish(
    repo: PostRepo,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("blog" / i32 / "publish")
        .and(warp::put())
        .map(move |id: i32| {
            let post = repo.clone().publish(id).unwrap();
            warp::reply::json(&post)
        })
}
