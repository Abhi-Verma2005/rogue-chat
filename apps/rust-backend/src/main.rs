mod database;
use axum::{
    routing::{get, post},
    Json,
    Router,
};
use dotenvy::dotenv;

struct SignUpPayload {
    username: String,
    password: String,
    email: String
}


#[tokio::main]
async fn main() {

    dotenv().ok();

    let _connect = database::connect().await;

    println!("Connected to db");
   
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));
    


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}