use crate::models::FileEntry;
use anyhow::Result;
use reqwest::Client;
use std::path::Path;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

pub async fn download_all_files(
    client: &Client,
    file_entries: &[FileEntry],
    dest_root: &Path,
) -> Result<()> {
    for entry in file_entries {
        let dest_path = dest_root.join(&entry.full_path);

        if let Some(parent_dir) = dest_path.parent() {
            fs::create_dir_all(parent_dir).await?;
        }

        let mut file = File::create(dest_path).await?;
        let bytes = client
            .get(&entry.url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        file.write_all(&bytes).await?;
    }
    Ok(())
}
