// fn main() {
//     println!("Hello, world!");
// }

// use axum::{
//     routing::get,
//     Router,
// };

// #[tokio::main]
// async fn main() {
//     // 构建router
//     let app = Router::new().route("/", get(|| async { "Hello, World!" }));

//     // 运行hyper  http服务 localhost:3000
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
use axum::response::Html;
use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
    Json,
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // build our application with a single route

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/html", get(html))  //路径对应handler
        .route("/json", get(json).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .layer(
        // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
        // for more details
        //
        // pay attention that for some request types like posting content-type: application/json
        // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
        // or see this issue https://github.com/tokio-rs/axum/issues/849
        CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET]),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // 一个个handler
    async fn html()-> Html<&'static str>{
        Html("<p>Hello, World!</p>")
    }

    // 返回json格式
    async fn json() -> Json<Vec<String>> {
        Json(vec!["foo".to_owned(), "bar".to_owned()])
    }
    async fn post_foo() {
        
    }
    async fn foo_bar() {
        
    }
}
