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

use sea_orm::{
    Database,
    DatabaseConnection,
    ActiveValue,
    ActiveModelTrait,
    EntityTrait
};

use tokio::time::Duration;

pub mod entity;

use entity::user::ActiveModel as UserModel;

use entity::user::Entity as UserDao;

use chrono::Local;



// use core::time::Duration;
// use std::time::Duration;
// use tokio::time::Duration;

// use tracing::log::LevelFilter;
// use tracing::metadata::LevelFilter;
// use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
async fn main() {

    let db: DatabaseConnection = Database::connect("mysql://root:123456@127.0.0.1:3306/test").await.unwrap();

    let user: UserModel =  UserModel{
        id: ActiveValue::NotSet,
        username: ActiveValue::Set("你好".to_owned()),
        birthday: ActiveValue::Set(Some(Local::now().naive_local())),
        sex: ActiveValue::Set(Some("1".to_owned())),
        address: ActiveValue::Set(Some("address".to_owned())),
    };
    let result = user.insert(&db).await.unwrap();
    println!("插入成功！：{:?}",result);

    let option = UserDao::find_by_id(50).one(&db).await.unwrap();
    match option {
        None => {}
        Some(user) => println!("查询成功！：{:?}",user)
    }

    // 一个个handler
    async fn html()-> Html<&'static str>{

        Html("<p>Hello, World!</p>")
    }

    // let mut opt = ConnectOptions::new("mysql://root:root@127.0.0.1:3306/sites");
    // opt.max_connections(100)
    //     .min_connections(5)
    //     .connect_timeout(Duration::from_secs(8))
    //     .acquire_timeout(Duration::from_secs(8))
    //     .idle_timeout(Duration::from_secs(8))
    //     .max_lifetime(Duration::from_secs(8))
    //     // .sqlx_logging(true)
    //     // .sqlx_logging_level(log::LevelFilter::Info)
    //     .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    // let _db = Database::connect(opt).await;

    // // 设置全局日志级别为 info
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    // //单独设置sea_orm
    // .add_directive("sea_orm::driver=debug".parse().unwrap())
    // //关闭sqlx自带的日志
    // .add_directive("sqlx::query=off".parse().unwrap());

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



    // 返回json格式
    async fn json() -> Json<Vec<String>> {
        Json(vec!["foo".to_owned(), "bar".to_owned()])
    }
    async fn post_foo() {
        
    }
    async fn foo_bar() {
        
    }
}
