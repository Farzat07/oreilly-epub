use serde::Deserialize;

// --- Models for the Search API ---

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub authors: Vec<String>,
    pub publishers: Vec<String>,
    pub cover_url: String,
}

// --- Models for the EPUB API ---

#[derive(Debug, Deserialize)]
pub struct EpubResponse {
    pub publication_date: String,
    pub chapters: String, // This is a URL to the chapters list
    pub files: String,    // This is a URL to the resource files
}
