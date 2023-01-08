use std::net::SocketAddr;

use axum::{Router, routing::get, response::Html};



#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([0,0,0,0], 8090));
    println!("listen on {}", addr);
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();


}


async fn handler() -> Html<&'static str> {
    Html("<h1>hello world</h1>")
}