# oreilly-epub

A small CLI tool that downloads a book from **O'Reilly Learning** (given a
`bookid`) and repackages it into a **valid EPUB**. It mirrors the publisher's
layout, fixes resource links (images, etc.) so they work offline, and zips
everything into a ready‑to‑read `.epub`.

> :warning: You must have a valid O'Reilly Learning subscription and your own
session cookies. This tool is intended for personal/offline use with content
you're authorized to access.
>
> :warning: If you download too many books at once, the website will start
returning the following error: HTTP status client error (403 Forbidden). \
Download books moderately and add intervals between downloads. Remember that
the tool is intended for personal use only. I suggest not downloading in a day
more than you can realistically read in a week.

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
EPUB so images/styles render correctly.
- Accelerated file downloads via parallelization.

## Installation

### Ready-to-use binaries

You can find binaries for major Operating Systems at
[GitHub releases](https://github.com/Farzat07/oreilly-epub/releases). \
For portable Linux releases (`musl` or `ARM64`), check the
[GitLab releases](https://gitlab.com/Farzat07/oreilly-epub/-/releases).

Just plug-in and use.

### Manual build

#### Build requirements

- **Rust** (stable, 1.75+ recommended) with Cargo.

#### Build instructions

```bash
git clone <REPO_URL>
cd oreilly-epub
cargo build --release
```

The binary will be at `target/release/oreilly-epub`.

## Usage

```txt
oreilly-epub [OPTIONS] <BOOKID>

Arguments:
  <BOOKID>  The Book digits ID that you want to download

Options:
      --cookies <COOKIES>    Path to the cookies.json file.
      --skip-download        If files already downloaded in a previous run.
      --parallel <PARALLEL>  Number of files to download in parallel.
```

**Example:**

```bash
oreilly-epub 9781787782204 --cookies ./cookies.json
```

Requires:

- A **`cookies.json`** file for `learning.oreilly.com` (see below).
- Network access to O'Reilly's API while running the tool.

### Download directory

This depends on the platform, as below:

|Platform|Value|Example|
|--------|-----|-------|
|Linux|`$XDG_DOWNLOAD_DIR`/oreilly-epub or `$HOME`/Downloads/oreilly-epub|/home/alice/Downloads/oreilly-epub|
|macOS|`$HOME`/Downloads/oreilly-epub|/Users/Alice/Downloads/oreilly-epub|
|Windows|`{FOLDERID_Downloads}`\oreilly-epub|C:\Users\Alice\Downloads\oreilly-epub|

### Cookies setup (`cookies.json`)

Place a `cookies.json` file in the current working directory or config directory
(or pass `--cookies <path>`). \
The file is a **flat JSON object**: cookie name → cookie value.

**Example:**

```json
{
  "orm-session": "REDACTED",
  "orm-cred": "REDACTED",
  "another_cookie": "value"
}
```

You can follow the steps below to create the file. Make sure to keep the file private.

1. Login as usual to [https://learning.oreilly.com/](https://learning.oreilly.com/).
1. Open the developer tools with F12 or Ctrl-Shift-i.
1. Go to the Network tab in the developer tools.
1. Access the profile page in the browser: [https://learning.oreilly.com/profile/](https://learning.oreilly.com/profile/).
1. In the Network tab, click on the request to /profile/ (should be the first one).
1. Click on the Cookies tab in the request information.
1. Right-click on the Request cookies text and choose Copy All.
1. Paste this into the cookies.json file and remove the quotes surrounding the JSON.

### Config directory

This depends on the platform, as below:

|Platform|Value|Example|
|--------|-----|-------|
|Linux|`$XDG_CONFIG_HOME`/oreilly-epub or `$HOME`/.config/oreilly-epub|/home/alice/.config/oreilly-epub|
|macOS|`$HOME`/Library/Application Support/oreilly-epub|/Users/Alice/Library/Application&nbsp;Support/oreilly-epub|
|Windows|`{FOLDERID_LocalAppData}`\oreilly-epub|C:\Users\Alice\AppData\Local\oreilly-epub|

## Notes & Limitations

- This tool assumes the O'Reilly “files” API includes OPF/NCX and all
referenced assets.
- Concurrency is not enabled yet; downloads are sequential.

## Roadmap / TODO

- [ ] **Logging**: add the ability to log unexpected behaviour.
- [x] **CONTRIBUTING.md**: add architecture notes & contributor guidelines.
- [x] **Robust HTML rewriting**: replace string replacement with real XHTML
parsing to update `src`, `href`, and other attributes precisely.
- [x] **Stylesheets completeness**: ensure all CSS referenced by chapters is
included and linked properly (cross-check chapters endpoint vs files list).
- [x] **License**: add copyright notice to each file and specify it in Cargo.toml.
- [x] **XDG directories**: use XDG‑compatible defaults for config and the
download root.
- [x] **Concurrency**: implement parallel downloads with a configurable limit.
- [ ] **Progress reporting**: display per‑file and overall progress (bytes
and/or file counts).
- [x] **Richer metadata**: add metadata such as description to the OPF.
- [x] **XML generation**: build `container.xml` using an XML writer instead of
raw strings.
- [x] **Low‑memory zip**: stream files to the archive in chunks to reduce peak
memory.
- [x] **CI/CD**: add a basic pipeline (build, fmt, clippy, test, release
artifact).
- [ ] **Tests**: write actual tests to run in the CI pipeline.
