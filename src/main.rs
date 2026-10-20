use axum::{routing::get, Router};

async fn home() -> &'static str{
    "Welcome to the SQL"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Server has started");
    axum::serve(listener, app).await.unwrap();
}

