use axum::{Router, routing::get};
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!(
        "🚀 Webサーバーがポート3000で起動しました! http://localhost:3000にアクセスしてください"
    );

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello,Web World! Rust!Rust!Rust!"
}
