use relative_path::RelativePathBuf;
use serde::Deserialize;
use url::Url;

/// Generic Model for paginated API.
#[derive(Debug, serde::Deserialize)]
pub struct Paginated<T> {
    pub next: Option<String>,
    pub results: Vec<T>,
}

/// Model for the EPUB API.
#[derive(Debug, Deserialize)]
pub struct EpubResponse {
    pub publication_date: String,
    pub title: String,
    pub chapters: String,          // This is a URL to the chapters list
    pub files: String,             // This is a URL to the resource files
    pub spine: String,             // This is a URL to the spine list
    pub table_of_contents: String, // This is a URL to the table of contents
    pub language: String,
}

/// Model for chapters API.
#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub ourn: String,
    pub title: String,
    pub is_skippable: bool,
    pub related_assets: ChapRelAssets,
}

/// Sub-model of Chapter - related_assets.
#[derive(Debug, Deserialize)]
pub struct ChapRelAssets {
    pub stylesheets: Vec<Url>,
}

/// Model for files API.
#[derive(Debug, Deserialize)]
pub struct FileEntry {
    pub ourn: String,
    pub url: Url,
    pub full_path: RelativePathBuf,
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
