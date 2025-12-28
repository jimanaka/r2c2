use axum::{Router, routing::get};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    ip: String,

    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    println!("[i] Starting server...");

    let args = Args::parse();

    let url = format!("{}:{}", &args.ip, &args.port);

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
