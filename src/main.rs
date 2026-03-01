mod http_client;

use anyhow::Result;
use clap::Parser;
use http_client::build_authenticated_client;

/// Download and generate an EPUB from Safari Books Online.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The Book digits ID that you want to download.
    #[arg(required = true)]
    bookid: String,
    /// Path to the cookies.json file.
    #[arg(long, default_value = "cookies.json")]
    cookies: String,
    /// Do not delete the log file on success.
    #[arg(long = "preserve-log")]
    preserve_log: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse the command line arguments
    let args = Args::parse();

    println!("Welcome to SafariBooks Rust Port!");
    println!("Target Book ID: {}", args.bookid);

    // Initialise the HTTP client.
    println!("Loading cookies and initialising the HTTP client...");
    let client = build_authenticated_client(&args.cookies)?;

    // Quick test request to verify authentication.
    let profile_url = "https://learning.oreilly.com/profile/";
    let response = client.get(profile_url).send().await?;
    if response.status().is_success() {
        println!("Successfully authenticated!");
    } else {
        println!(
            "Authentication might have failed. Status code: {}",
            response.status()
        );
    }

    Ok(())
}
