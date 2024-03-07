// fn main() {
//     println!("Hello, world!");
// }

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // 构建router
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // 运行hyper  http服务 localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}