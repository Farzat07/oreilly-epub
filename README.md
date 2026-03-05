# oreilly-epub

A small CLI tool that downloads a book from **O'Reilly Learning** (given a
`bookid`) and repackages it into a **valid EPUB**. It mirrors the publisher's
layout, fixes resource links (images, etc.) so they work offline, and zips
everything into a ready‑to‑read `.epub`.

> :warning: You must have a valid O'Reilly Learning subscription and your own
> session cookies. This tool is intended for personal/offline use with content
> you're authorized to access.

## Features

- **Deterministic by `bookid`**: no fuzzy search—just pass a known identifier.
- **Authenticated requests** via `cookies.json` (flat key‑value).
- **Exploded tree download**: mirrors O'Reilly's `full_path` hierarchy under a
local `epub_root/`.
- **Valid EPUB packaging**:
  - Writes the `mimetype` entry first, **uncompressed** (EPUB requirement).
  - Generates `META-INF/container.xml` pointing to the publisher's OPF.
  - Zips all assets (HTML, images, CSS, fonts, OPF, NCX, …).
- **HTML link rewriting during zip**:
  - Converts absolute O'Reilly API URLs (e.g.,
`/api/v2/epubs/.../files/images/foo.jpg`) to **relative paths** inside the
EPUB so images/styles render correctly offline.

## Requirements

- **Rust** (stable, 1.75+ recommended) with Cargo.
- A **`cookies.json`** file for `learning.oreilly.com` (see below).
- Network access to O'Reilly's API while running the tool.

## Installation

```bash
git clone <REPO_URL>
cd oreilly-epub
cargo build --release
```

The binary will be at `target/release/oreilly-epub`.

## Cookies setup (`cookies.json`)

Place a `cookies.json` file in the project root (or pass `--cookies <path>`). \
The file is a **flat JSON object**: cookie name → cookie value.

**Example:**

```json
{
  "orm-session": "REDACTED",
  "orm-cred": "REDACTED",
  "another_cookie": "value"
}
```

> Tip: You can obtain the cookies for `learning.oreilly.com` from your
> browser's developer tools by visiting the website and running the command
> below in the console. Write down the output to `cookies.json` and keep the
> file private.

```js
JSON.stringify(document.cookie.split(";").map(c=>c.split("=")).reduce((r,[k,v])=>({...r,[k.trim()]:v?.trim()}),{}))
```

## Usage

```bash
# Basic:
oreilly-epub <bookid>

# With a custom cookies file:
oreilly-epub <bookid> --cookies /path/to/cookies.json
```

**Example:**

```bash
target/release/oreilly-epub 9781787782204 --cookies ./cookies.json
```

## Notes & Limitations

- This tool assumes the O'Reilly “files” API includes OPF/NCX and all
referenced assets.
- HTML rewriting currently uses simple string replacement guided by a URL→local
mapping; it is **not** a full DOM parse (see Roadmap).
- File streaming during zip currently buffers each file into memory (OK for
most books; see Roadmap for planned streaming improvements).
- Concurrency is not enabled yet; downloads are sequential.

## Roadmap / TODO

- [ ] **CONTRIBUTING.md**: add architecture notes & contributor guidelines.
- [ ] **Robust HTML rewriting**: replace string replacement with real HTML
parsing to update `src`, `href`, and other attributes precisely.
- [ ] **Stylesheets completeness**: ensure all CSS referenced by chapters is
included and linked properly (cross-check chapters endpoint vs files list).
- [ ] **License**: add a LICENSE file.
- [ ] **XDG directories**: use XDG‑compatible defaults for config and the
download root.
- [ ] **Concurrency**: implement parallel downloads with a configurable limit.
- [ ] **Progress reporting**: display per‑file and overall progress (bytes
and/or file counts).
- [ ] **Richer metadata**: pull extended book metadata from O'Reilly's metadata
endpoint and embed into the EPUB.
- [ ] **XML generation**: build `container.xml` using an XML writer instead of
raw strings.
- [ ] **Low‑memory zip**: stream files to the archive in chunks to reduce peak
memory.
- [ ] **CI/CD**: add a basic pipeline (build, fmt, clippy, test, release
artifact).
