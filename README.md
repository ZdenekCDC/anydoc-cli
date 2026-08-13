# anydoc-cli

Standalone command-line tool that converts Office/PDF/EPUB documents to
Markdown - no Node.js, Python, LibreOffice, poppler, or API key required.
Ships as a single native binary for Windows, macOS, and Linux, so it can be
downloaded and run directly by an AI coding agent (or a human) that doesn't
have a dev environment set up.

This is an independent, unofficial wrapper. All document conversion is
performed by the [`anydoc`](https://github.com/firecrawl/anydoc) crate (MIT
licensed) - see [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md) for
attribution. This project is not affiliated with Firecrawl.

## Supported formats

`doc`, `docx`, `docm`, `odt`, `pdf`, `ppt`, `pptx`, `rtf`, `epub`, `xls`,
`xlsx`, `xlsm`, `xlsb`, `ods`, `odp`, `csv`.

Scanned/image-only PDFs (which need OCR) are not supported.

## Install

Download the binary for your platform from the
[Releases](../../releases) page and put it on your `PATH`.

### First run may be blocked by OS security warnings

The binaries are **not code-signed**. If you downloaded the file through a
browser:

- **macOS**: `xattr -d com.apple.quarantine ./anydoc-cli` before running, or
  allow it via System Settings > Privacy & Security.
- **Windows**: right-click the file > Properties > check "Unblock", or click
  "More info > Run anyway" if SmartScreen warns you. If Smart App Control is
  enabled in enforced mode, it may refuse to run unsigned binaries entirely.

## Usage

```sh
anydoc-cli report.docx                # prints Markdown to stdout
anydoc-cli report.docx -o report.md   # writes Markdown to a file
```

## Build from source

Requires Rust (edition 2024, `rustc` 1.88+):

```sh
cargo build --release
```

## License

This wrapper is MIT licensed, see [LICENSE](LICENSE). It depends on the MIT
licensed [`anydoc`](https://github.com/firecrawl/anydoc) crate; see
[THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).
