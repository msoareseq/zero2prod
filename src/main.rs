use std::net::TcpListener;
use zero2prod::startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = "8000";
    let listener = TcpListener::bind(format!("http://127.0.0.1:{}", port))
        .expect(&format!("Failed to bind listener to port {}", port));
    startup::run(listener)?.await
}
