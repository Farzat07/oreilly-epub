mod http_client;
mod models;

use anyhow::{Context, Result};
use clap::Parser;
use http_client::build_authenticated_client;
use models::{Chapter, EpubResponse, FileEntry, Paginated, SearchResult};
use reqwest::Client;

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

/// Fetches EPUB structural data (like the chapters URL).
async fn fetch_epub_data(client: &Client, bookid: &str) -> Result<EpubResponse> {
    let url = format!("https://learning.oreilly.com/api/v2/epubs/urn:orm:book:{bookid}/");
    let response = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json::<EpubResponse>()
        .await
        .context("Failed to deserialize EPUB API response")?;
    Ok(response)
}

/// Fetch a paginated API.
async fn fetch_all_pages<T>(client: &reqwest::Client, mut url: String) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let mut items = Vec::new();
    loop {
        // GET current URL and deserialize into Paginated<T>.
        let response = client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<Paginated<T>>()
            .await
            .context("Failed to deserialize API response.")?;
        // Extend items with the page's results.
        items.extend(response.results);
        // Set url to next page if available, else break.
        if let Some(next) = response.next {
            url = next;
        } else {
            break;
        }
    }
    Ok(items)
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

    println!("Fetching book metadata...");
    // Fetch from the search API.
    let search_url = format!(
        "https://learning.oreilly.com/api/v2/search/?query={}",
        args.bookid
    );
    let search_data: Vec<SearchResult> = fetch_all_pages(&client, search_url).await?;
    if let Some(book) = search_data.first() {
        println!("\n--- Book Found ---");
        println!("Title: {}", book.title);
        println!("Authors: {}", book.authors.join(", "));
        println!("Publisher: {}", book.publishers.join(", "));
        println!("Cover URL: {}", book.cover_url);
    } else {
        anyhow::bail!("Could not find book metadata for ID: {}", args.bookid);
    }
    // Fetch from the EPUB API.
    let epub_data = fetch_epub_data(&client, &args.bookid).await?;
    println!("Publication date: {}", epub_data.publication_date);
    println!("Chapters URL: {}", epub_data.chapters);
    println!("Resources URL: {}", epub_data.files);
    println!("------------------\n");

    println!("Fetching book structure...");
    let chapters: Vec<Chapter> = fetch_all_pages(&client, epub_data.chapters.clone()).await?;
    let file_entries: Vec<FileEntry> = fetch_all_pages(&client, epub_data.files.clone()).await?;

    Ok(())
}
