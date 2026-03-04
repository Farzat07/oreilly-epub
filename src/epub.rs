use crate::models::{Chapter, FileEntry};
use anyhow::{Context, Result};
use relative_path::RelativePath;
use reqwest::{Client, Url};
use std::{
    collections::HashMap,
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
    chapters: &HashMap<String, Chapter>,
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

    // Prepare url path to local path mapping to clean xhtml files from external dependencies.
    let url_to_local: HashMap<String, String> = file_entries
        .iter()
        .map(url_path_to_local)
        .collect::<Result<HashMap<_, _>>>()?;

    // Add the rest of the files according to file_entries.
    let options: FileOptions<()> =
        FileOptions::default().compression_method(CompressionMethod::Deflated);
    for entry in file_entries {
        zip.start_file(&entry.full_path, options)?;
        let mut src_file = std::fs::File::open(epub_root.join(&entry.full_path))?;
        let mut buffer = Vec::new();
        src_file.read_to_end(&mut buffer)?;
        if chapters.contains_key(&entry.ourn) {
            let mut html = String::from_utf8(buffer)?;
            let chapter_dir = RelativePath::new(&entry.full_path)
                .parent()
                .unwrap_or(RelativePath::new(""));
            for (url_path, local_path) in &url_to_local {
                let rel_path = chapter_dir
                    .to_relative_path_buf()
                    .relative(RelativePath::new(local_path));
                html = html.replace(url_path, rel_path.as_str());
            }
            zip.write_all(html.as_bytes())?;
        } else {
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;

    Ok(())
}

/// Helper function. Maps FileEntry to (url path, full_path) pair.
fn url_path_to_local(entry: &FileEntry) -> Result<(String, String)> {
    let url = Url::parse(&entry.url).with_context(|| format!("Could not parse: {}", entry.url))?;
    let url_path = url.path().to_string();
    Ok((url_path, entry.full_path.clone()))
}
