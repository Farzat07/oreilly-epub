use serde::Deserialize;

/// Generic Model for paginated API.
#[derive(Debug, serde::Deserialize)]
pub struct Paginated<T> {
    pub next: Option<String>,
    pub results: Vec<T>,
}

/// Model for the Search API.
#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub authors: Vec<String>,
    pub publishers: Vec<String>,
    pub cover_url: String,
}

/// Model for the EPUB API.
#[derive(Debug, Deserialize)]
pub struct EpubResponse {
    pub publication_date: String,
    pub chapters: String,          // This is a URL to the chapters list
    pub files: String,             // This is a URL to the resource files
    pub spine: String,             // This is a URL to the spine list
    pub table_of_contents: String, // This is a URL to the table of contents
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

/// Model for spine API.
#[derive(Debug, Deserialize)]
pub struct SpineItem {
    pub ourn: String,
    pub reference_id: String,
    pub title: String,
}

/// Model for table of contents API.
#[derive(Debug, Deserialize)]
pub struct TocNode {
    pub depth: u32,
    pub reference_id: String,
    pub ourn: String,
    pub fragment: String,
    pub title: String,
    pub children: Vec<TocNode>,
}
