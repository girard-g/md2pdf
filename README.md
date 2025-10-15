# md2pdf - Professional Markdown to PDF Converter

A high-quality, production-ready CLI tool for converting Markdown files to professionally formatted PDFs with intelligent page break handling.

## Features

- **Smart Page Breaks**: Automatically prevents content splitting across pages
  - Tables stay together
  - Code blocks remain intact
  - Headings stay with their content
  - Lists don't orphan items

- **Professional Styling**: Beautiful default CSS optimized for business documents
  - Clean typography with professional fonts
  - Styled tables with alternating row colors
  - Syntax-highlighted code blocks
  - Proper heading hierarchy
  - Print-optimized spacing and margins

- **Customization**: Full control over PDF output
  - Custom CSS files for branding
  - Configurable paper size and margins
  - A4, Letter, or custom dimensions

- **Batch Processing**: Convert multiple files at once
  - Process entire directories
  - Recursive directory scanning
  - Parallel conversion support

- **Robust Error Handling**: Comprehensive error messages and logging
  - Detailed error information
  - Verbose mode for debugging
  - Clear success/failure reporting

## Installation

### Prerequisites

- Rust 1.75 or later
- Chrome/Chromium browser (for headless PDF rendering)

### From Source

```bash
git clone https://github.com/yourusername/md2pdf.git
cd md2pdf
cargo build --release
```

The binary will be available at `target/release/md2pdf`.

### Install System-wide

```bash
cargo install --path .
```

## Usage

### Basic Usage

Convert a single Markdown file to PDF:

```bash
md2pdf document.md
```

This creates `document.pdf` in the same directory.

### Specify Output File

```bash
md2pdf document.md -o output.pdf
```

### Custom CSS Styling

Use your own CSS file for custom branding:

```bash
md2pdf document.md --css custom-style.css -o branded.pdf
```

### Batch Conversion

Convert all Markdown files in a directory:

```bash
md2pdf docs/ -o output/
```

Recursively convert all Markdown files:

```bash
md2pdf docs/ -r -o output/
```

### Custom Paper Size and Margins

```bash
md2pdf document.md \
  --paper-width 8.5 \
  --paper-height 11 \
  --margin-top 1 \
  --margin-bottom 1 \
  --margin-left 0.75 \
  --margin-right 0.75 \
  -o letter-size.pdf
```

### Verbose Mode

Enable detailed logging for debugging:

```bash
md2pdf document.md -v
```

## Command-Line Options

```
md2pdf [OPTIONS] <INPUT>...

Arguments:
  <INPUT>...  Input markdown file(s) or directory containing .md files

Options:
  -o, --output <OUTPUT>           Output PDF file or directory
  -c, --css <CSS_FILE>           Custom CSS file for PDF styling
  -v, --verbose                  Enable verbose output for debugging
  -r, --recursive                Process directories recursively
      --paper-width <WIDTH>      Paper width in inches (default: 8.27 for A4)
      --paper-height <HEIGHT>    Paper height in inches (default: 11.69 for A4)
      --margin-top <MARGIN>      Top margin in inches
      --margin-bottom <MARGIN>   Bottom margin in inches
      --margin-left <MARGIN>     Left margin in inches
      --margin-right <MARGIN>    Right margin in inches
  -h, --help                     Print help
  -V, --version                  Print version
```

## Markdown Support

md2pdf supports the full CommonMark specification plus these extensions:

- Tables (GitHub-flavored)
- Strikethrough (`~~text~~`)
- Task lists (`- [ ]` and `- [x]`)
- Footnotes
- Smart punctuation
- Heading attributes

### Example Markdown Features

```markdown
# Main Heading

## Features Table

| Feature | Status |
|---------|--------|
| Tables  | ✓      |
| Code    | ✓      |

## Code Block

\`\`\`rust
fn main() {
    println!("Hello, PDF!");
}
\`\`\`

## Task List

- [x] Completed task
- [ ] Pending task

## Emphasis

**Bold text** and *italic text* and ~~strikethrough~~.

> This is a blockquote that will be styled professionally.
```

## Custom CSS

You can create a custom CSS file to match your branding. The default CSS uses these key features for page break control:

```css
/* Prevent page breaks inside these elements */
table, pre, blockquote {
    page-break-inside: avoid;
    break-inside: avoid;
}

/* Keep headings with following content */
h1, h2, h3, h4, h5, h6 {
    page-break-after: avoid;
    break-after: avoid;
}

/* Control orphans and widows */
p {
    orphans: 3;
    widows: 3;
}
```

See the embedded default CSS in `src/template.rs` for a complete example.

## Architecture

The project is structured as a library (`lib.rs`) with a CLI wrapper (`main.rs`):

```
src/
├── main.rs       # CLI entry point with clap
├── lib.rs        # Public API
├── error.rs      # Custom error types
├── markdown.rs   # Markdown file reading/validation
├── html.rs       # HTML generation with semantic markup
├── pdf.rs        # PDF generation via headless Chrome
└── template.rs   # CSS and HTML templating
```

### Key Components

- **Error Handling**: Custom error types using `thiserror` for ergonomic error handling
- **Markdown Parsing**: `pulldown-cmark` for spec-compliant parsing
- **HTML Generation**: Semantic markup with CSS classes for page break hints
- **PDF Rendering**: Headless Chrome for accurate CSS rendering and PDF generation
- **CLI**: `clap` with derive API for type-safe argument parsing

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Debug Logging

```bash
RUST_LOG=debug cargo run -- document.md -v
```

### Linting

```bash
cargo clippy -- -D warnings
```

### Formatting

```bash
cargo fmt
```

## Library Usage

md2pdf can also be used as a library in your Rust projects:

```toml
[dependencies]
md2pdf = "0.1"
```

```rust
use md2pdf::{convert_markdown_to_pdf, ConversionOptions};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ConversionOptions::default();

    convert_markdown_to_pdf(
        Path::new("input.md"),
        Path::new("output.pdf"),
        &options
    )?;

    Ok(())
}
```

## Troubleshooting

### Chrome/Chromium Not Found

If you get errors about Chrome not being found, ensure Chrome or Chromium is installed:

**Ubuntu/Debian:**
```bash
sudo apt install chromium-browser
```

**macOS:**
```bash
brew install --cask chromium
```

**Arch Linux:**
```bash
sudo pacman -S chromium
```

### Permission Denied

If you get permission errors, ensure the output directory is writable:

```bash
chmod u+w output/
```

### Out of Memory

For very large documents, you may need to increase the system memory available to Chrome. Consider splitting large documents into smaller files.

## Performance

- Single file conversion: ~500ms - 2s depending on document complexity
- Batch processing: Parallel conversion of multiple files
- Memory usage: ~100-300MB per conversion (Chrome overhead)

## Roadmap

- [ ] Table of contents generation
- [ ] Header/footer templates
- [ ] Custom font embedding
- [ ] SVG support
- [ ] Math equation rendering (LaTeX)
- [ ] Syntax highlighting themes
- [ ] PDF metadata (author, title, keywords)
- [ ] Watermark support

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure `cargo test` and `cargo clippy` pass
5. Submit a pull request

## Acknowledgments

- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown parsing
- [headless_chrome](https://github.com/atroche/rust-headless-chrome) - PDF generation
- [clap](https://github.com/clap-rs/clap) - CLI parsing

## Support

For bugs and feature requests, please open an issue on GitHub.
