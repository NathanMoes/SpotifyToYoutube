use std::env;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    println!("Listening on port {}", port);
}
