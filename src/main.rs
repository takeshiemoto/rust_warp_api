use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::http::Response;
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Hero {
    name: String,
}

#[tokio::main]
async fn main() {
    // 基本形
    let example1 = warp::get()
        .and(warp::path("example1"))
        .and(warp::path::end())
        .map(|| "Hello world");

    // クエリ・ストリング
    let example2 = warp::get()
        .and(warp::path("example2"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("key") {
            Some(key) => Response::builder().body(format!("key = {}", key)),
            None => Response::builder().body(String::from("No key")),
        });

    // パス・パラメータ
    let example3 = warp::get()
        .and(warp::path("example3"))
        .and(warp::path::param::<String>())
        .map(|name: String| format!("You name is = {}", name));

    // JSONを返す
    let example4 = warp::get()
        .and(warp::path("example4"))
        .and(warp::path::end())
        .map(|| {
            let hero = Hero {
                name: String::from("Ultraman"),
            };
            warp::reply::json(&hero)
        });

    let routes = example1.or(example2).or(example3).or(example4);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
