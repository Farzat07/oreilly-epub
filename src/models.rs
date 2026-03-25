// Copyright (C) 2026  A Farzat
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License, GPLv3, attached at the root of the project.

use relative_path::RelativePathBuf;
use serde::Deserialize;
use url::Url;

/// Generic Model for paginated API.
#[derive(Debug, serde::Deserialize)]
pub struct Paginated<T> {
    pub next: Option<Url>,
    pub results: Vec<T>,
}

/// Model for the EPUB API.
#[derive(Debug, Deserialize)]
pub struct EpubResponse {
    pub publication_date: String,
    pub title: String,
    pub descriptions: Descriptions,
    pub chapters: Url,
    pub files: Url,
    pub language: String,
}

/// Sub-model of EpubResponse - descriptions.
#[derive(Debug, Deserialize)]
pub struct Descriptions {
    #[serde(rename = "text/plain")]
    pub plain: String,
}

/// Model for chapters API.
#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub ourn: String,
    pub title: String,
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
    pub filename_ext: String,
}

// Model for download configuration
pub struct DownloadConfig
{
    pub max_concurrent: usize,
    pub sleep_ms: u64,
    pub max_retries: u32,
    pub retry_sleep_ms: u64,
}
