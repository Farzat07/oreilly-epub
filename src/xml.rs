use anyhow::{Context, Result};
use ogrim::{Document, xml};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use relative_path::{RelativePath, RelativePathBuf};
use std::collections::HashMap;
use std::io::Cursor;
use url::Url;

use crate::models::{Chapter, EpubResponse, FileEntry};

/// Checks if a tag is a standard HTML void element that shouldn't have a closing tag.
fn is_html_void_tag(name: &[u8]) -> bool {
    matches!(
        name,
        b"area"
            | b"base"
            | b"br"
            | b"col"
            | b"embed"
            | b"hr"
            | b"img"
            | b"input"
            | b"link"
            | b"meta"
            | b"param"
            | b"source"
            | b"track"
            | b"wbr"
    )
}

/// Processes the fragment and outputs a complete, EPUB-ready XHTML document.
pub fn build_epub_chapter(
    epub_data: &EpubResponse,
    chapter: &Chapter,
    chapter_dir: &RelativePath,
    fragment: &str,
    url_to_file: &HashMap<&Url, &FileEntry>,
    url_path_to_local: &HashMap<&str, &RelativePathBuf>,
) -> Result<String> {
    // Setup the XML Reader and Writer.
    let mut reader = Reader::from_str(fragment);
    // Preserve spacing for EPUB text formatting.
    reader.config_mut().trim_text(false);
    // Fragments could have unmatched tags - tell the parser not to panic if so.
    reader.config_mut().check_end_names = false;
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    // Loop through the XML events and rewrite tags.
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag_data)) => {
                // If it is a void tag, convert it to a self-closing XML tag.
                let tag_type = if is_html_void_tag(tag_data.name().as_ref()) {
                    Event::Empty
                } else {
                    Event::Start
                };
                writer.write_event(tag_type(rewrite_attributes(
                    &tag_data,
                    url_path_to_local,
                    chapter_dir,
                )))?;
            }
            Ok(Event::Empty(tag_data)) => {
                // If tags are already empty, leave them as-is.
                writer.write_event(Event::Empty(rewrite_attributes(
                    &tag_data,
                    url_path_to_local,
                    chapter_dir,
                )))?;
            }
            Ok(Event::End(tag_data)) => {
                // Silently drop closing tags for void elements if they exist (e.g. <img></img>).
                if !is_html_void_tag(tag_data.name().as_ref()) {
                    writer.write_event(Event::End(tag_data))?;
                }
            }
            Ok(Event::Eof) => break,
            Ok(tag_data) => writer.write_event(tag_data)?, // Pass through text, comments, etc. unmodified.
            Err(e) => anyhow::bail!(e),
        }
    }

    // Extract the modified fragment
    let processed_fragment = String::from_utf8(writer.into_inner().into_inner())?;

    // Wrap in EPUB XHTML Boilerplate.
    // EPUBs strictly require the w3 and idpf namespaces to validate properly.
    let wrapper_xhtml = xml!(
        <?xml version="1.0" encoding="UTF-8"?>
        <html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops"
            lang={epub_data.language} xml:lang={epub_data.language}>
        <head>
            <title>{chapter.title}</title>
            {|doc| make_stylesheet_links(doc, chapter, chapter_dir, url_to_file)}
        </head>
        <body>
        </body>
        </html>
    );
    let wrapper_suffix = "</body></html>";
    let wrapper_prefix = wrapper_xhtml.as_str().strip_suffix(wrapper_suffix).context("Wrapper must end with </body></html>")?;

    Ok(format!("{}\n{}\n{}", wrapper_prefix, processed_fragment, wrapper_suffix))
}

/// Helper function add link elements for stylesheets to an xml Document.
fn make_stylesheet_links(
    doc: &mut Document,
    chapter: &Chapter,
    chapter_dir: &RelativePath,
    url_to_file: &HashMap<&Url, &FileEntry>,
) {
    chapter
        .related_assets
        .stylesheets
        .iter()
        .filter_map(|u| url_to_file.get(u))
        .for_each(|e| {
            let rel_path = chapter_dir.relative(&e.full_path);
            xml!(doc,
                <link rel="stylesheet" type={e.media_type} href={rel_path} />
            );
        })
}

/// Helper function to inspect tags and rewrite the elements' attributes.
fn rewrite_attributes<'a>(
    tag_data: &BytesStart<'a>,
    url_path_to_local: &HashMap<&str, &RelativePathBuf>,
    chapter_dir: &RelativePath,
) -> BytesStart<'static> {
    let name = String::from_utf8_lossy(tag_data.name().as_ref()).into_owned();
    let mut new_elem = BytesStart::new(name);

    for attr in tag_data.attributes().filter_map(Result::ok) {
        let key = attr.key.as_ref();

        // Intercept element attributes which could contain links to resources.
        if matches!(
            (tag_data.name().as_ref(), key),
            (b"img", b"src")
                | (b"source", b"src")
                | (b"video", b"src")
                | (b"audio", b"src")
                | (b"a", b"href")
                | (b"link", b"href")
                | (b"object", b"data")
                | (b"embed", b"src")
                | (b"iframe", b"src")
                | (b"track", b"src")
        ) {
            let url = String::from_utf8_lossy(&attr.value);

            // If we have a local path, inject it instead of the absolute URL.
            if let Some(local_path) = url_path_to_local.get(url.as_ref()) {
                let key = String::from_utf8_lossy(key);
                new_elem.push_attribute((key.as_ref(), chapter_dir.relative(local_path).as_str()));
                continue;
            }
        }

        // Keep all other attributes intact.
        new_elem.push_attribute(attr);
    }

    new_elem
}
