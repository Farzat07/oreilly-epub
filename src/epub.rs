use crate::models::FileEntry;
use anyhow::{Context, Result};
use reqwest::Client;
use std::{
    io::{Read, Write},
    path::Path,
};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

/// Creates and writes container.xml.
fn write_container_xml_to_zip(
    zip: &mut ZipWriter<std::fs::File>,
    opf_full_path: &str,
) -> Result<()> {
    // Prepare file contents.
    let contents = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="{opf_full_path}" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>
"#
    );

    // Write down the file.
    let options: FileOptions<()> =
        FileOptions::default().compression_method(CompressionMethod::Deflated);
    zip.start_file("META-INF/container.xml", options)?;
    zip.write_all(contents.as_bytes())?;
    Ok(())
}

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

/// Creates the EPUB archive (creates zip and includes all files in it).
pub fn create_epub_archive(
    epub_root: &Path,
    output_epub: &Path,
    file_entries: &[FileEntry],
) -> Result<()> {
    let out_file = std::fs::File::create(output_epub)?;
    let mut zip = ZipWriter::new(out_file);

    // Write mimetype to zip first. It must be uncompressed.
    let options: FileOptions<()> =
        FileOptions::default().compression_method(CompressionMethod::Stored);
    zip.start_file("mimetype", options)?;
    zip.write_all(b"application/epub+zip")?;

    // Find the OPF file entry to reference it in container.xml
    let opf_entry = file_entries
        .iter()
        .find(|f| f.filename_ext == ".opf" && f.media_type == "application/oebps-package+xml")
        .context("No OPF file with the correct MIME type was found.")?;
    write_container_xml_to_zip(&mut zip, &opf_entry.full_path)?;

    // Add the rest of the files according to file_entries.
    let options: FileOptions<()> =
        FileOptions::default().compression_method(CompressionMethod::Deflated);
    for entry in file_entries {
        zip.start_file(&entry.full_path, options)?;
        let mut src_file = std::fs::File::open(epub_root.join(&entry.full_path))?;
        let mut buffer = Vec::new();
        src_file.read_to_end(&mut buffer)?;
        zip.write_all(&buffer)?;
    }

    zip.finish()?;

    Ok(())
}
