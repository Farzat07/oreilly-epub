use clap::Parser;

/// Download and generate an EPUB from Safari Books Online.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The Book digits ID that you want to download.
    #[arg(required = true)]
    bookid: String,

    /// Do not delete the log file on success.
    #[arg(long = "preserve-log")]
    preserve_log: bool,
}

fn main() {
    // Parse the command line arguments
    let args = Args::parse();

    println!("Welcome to SafariBooks Rust Port!");
    println!("Target Book ID: {}", args.bookid);

    if args.preserve_log {
        println!("Logs will be preserved");
    }

    // TODO: Proceed to load cookies and setup the HTTP client...
}
