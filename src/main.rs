mod epub;
mod http_client;
mod models;

use std::path::Path;

use crate::epub::{create_epub_archive, download_all_files};
use crate::http_client::build_authenticated_client;
use crate::models::{Chapter, EpubResponse, FileEntry, Paginated, SpineItem, TocNode};
use anyhow::{Context, Result};
use clap::Parser;
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

/// Fetches a direct array endpoint (no pagination, simple list).
async fn fetch_direct_array<T>(client: &Client, url: &str) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let response = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<T>>()
        .await
        .context("Failed to deserialize API response")?;
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
    // Fetch from the EPUB API.
    let epub_data = fetch_epub_data(&client, &args.bookid).await?;
    println!("Publication date: {}", epub_data.publication_date);
    println!("Title: {}", epub_data.title);
    println!("Chapters URL: {}", epub_data.chapters);
    println!("Resources URL: {}", epub_data.files);
    println!("------------------\n");

    println!("Fetching book structure...");
    let chapters: Vec<Chapter> = fetch_all_pages(&client, epub_data.chapters.clone()).await?;
    let file_entries: Vec<FileEntry> = fetch_all_pages(&client, epub_data.files.clone()).await?;
    let spine_items: Vec<SpineItem> = fetch_all_pages(&client, epub_data.spine.clone()).await?;
    let toc_vec: Vec<TocNode> = fetch_direct_array(&client, &epub_data.table_of_contents).await?;

    let dest_root = format!("Books/{}/epub_root", args.bookid);
    let dest_root = Path::new(&dest_root);
    download_all_files(&client, &file_entries, dest_root).await?;
    let epub_path = format!("Books/{0}/{0}.epub", args.bookid);
    let epub_path = Path::new(&epub_path);
    create_epub_archive(dest_root, &epub_path, &file_entries)?;

    Ok(())
}
