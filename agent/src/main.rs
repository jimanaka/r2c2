use clap::{Parser, Subcommand};
use reqwest::{Client, get};
use std::error::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tunnel {
        #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
        ip: String,

        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Tunnel { ip, port } => {
            println!("Connecting to {}:{}", ip, port);
            let url = format!("http://{}:{}", ip, port);
            let body = get(&url).await?.text().await?;

            println!("Responds {}", body);
        }
    }

    Ok(())
}
