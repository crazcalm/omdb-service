mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let api = routes::health::register();

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}
