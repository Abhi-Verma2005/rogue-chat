use axum::{
    routing::{get, post},
    Json,
    Router,
};

struct SignUpPayload {
    username: String,
    password: String,
    email: String
}

#[tokio::main]
async fn main() {
   
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }));
    .route("/signup", post(|Json(payload): Json<SignUpPayload>| async move {
        
    } ));





    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}