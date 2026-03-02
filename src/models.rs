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

// --- Generic Model for paginated API ---

#[derive(Debug, serde::Deserialize)]
pub struct Paginated<T> {
    pub next: Option<String>,
    pub results: Vec<T>,
}

/// Model for chapters API.
#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub ourn: String,
    pub is_skippable: bool,
}

/// Model for files API.
#[derive(Debug, Deserialize)]
pub struct FileEntry {
    pub ourn: String,
    pub url: String,
    pub full_path: String,
    pub media_type: String,
    pub filename: String,
    pub filename_ext: String,
    pub kind: String,
}
