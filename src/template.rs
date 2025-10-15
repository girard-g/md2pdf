//! HTML and CSS templating for professional PDF output
//!
//! This module provides the HTML template structure and default CSS styling
//! optimized for professional business documents with smart page breaks.

use crate::error::{Md2PdfError, Result};
use std::fs;
use std::path::Path;

/// Default CSS for professional PDF output with smart page break handling
pub const DEFAULT_CSS: &str = r#"
/* Professional PDF Styling with Smart Page Breaks */

/* Reset and base styles */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

/* Page setup for print */
@page {
    size: A4;
    margin: 2.5cm 2cm;

    @top-center {
        content: string(doctitle);
    }

    @bottom-center {
        content: counter(page);
    }
}

/* Body and typography */
body {
    font-family: 'Segoe UI', 'Helvetica Neue', Arial, sans-serif;
    font-size: 11pt;
    line-height: 1.6;
    color: #333;
    background: white;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
}

/* Headings with page break control */
h1, h2, h3, h4, h5, h6 {
    font-weight: 600;
    line-height: 1.3;
    color: #1a1a1a;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
    page-break-after: avoid;
    break-after: avoid;
    page-break-inside: avoid;
    break-inside: avoid;
}

/* Ensure content after heading stays with it */
h1 + *, h2 + *, h3 + *, h4 + *, h5 + *, h6 + * {
    page-break-before: avoid;
    break-before: avoid;
}

h1 {
    font-size: 2.2em;
    border-bottom: 3px solid #2c5aa0;
    padding-bottom: 0.3em;
    margin-top: 0;
    string-set: doctitle content();
}

h2 {
    font-size: 1.8em;
    border-bottom: 2px solid #e0e0e0;
    padding-bottom: 0.25em;
}

h3 {
    font-size: 1.5em;
    color: #2c5aa0;
}

h4 {
    font-size: 1.25em;
}

h5 {
    font-size: 1.1em;
}

h6 {
    font-size: 1em;
    font-weight: 600;
    color: #666;
}

/* Paragraphs */
p {
    margin-bottom: 1em;
    text-align: justify;
    orphans: 3;
    widows: 3;
}

/* Links */
a {
    color: #2c5aa0;
    text-decoration: none;
    border-bottom: 1px solid #2c5aa0;
}

a:hover {
    color: #1a3a70;
}

/* Print URLs after links */
@media print {
    a[href^="http"]:after {
        content: " (" attr(href) ")";
        font-size: 0.8em;
        color: #666;
    }
}

/* Lists with orphan control */
ul, ol {
    margin-bottom: 1em;
    margin-left: 2em;
    page-break-inside: avoid;
    break-inside: avoid;
}

li {
    margin-bottom: 0.3em;
    orphans: 2;
    widows: 2;
}

/* Nested lists */
ul ul, ol ul, ul ol, ol ol {
    margin-top: 0.3em;
    margin-bottom: 0.3em;
}

/* Code blocks - prevent page breaks */
pre {
    background-color: #f5f7f9;
    border: 1px solid #e0e0e0;
    border-left: 4px solid #2c5aa0;
    border-radius: 4px;
    padding: 1em;
    margin-bottom: 1em;
    overflow-x: auto;
    page-break-inside: avoid;
    break-inside: avoid;
    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 0.9em;
    line-height: 1.4;
}

code {
    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 0.9em;
    background-color: #f5f7f9;
    padding: 0.2em 0.4em;
    border-radius: 3px;
}

pre code {
    background-color: transparent;
    padding: 0;
}

/* Blockquotes */
blockquote {
    border-left: 4px solid #2c5aa0;
    padding-left: 1em;
    margin: 1em 0;
    color: #555;
    font-style: italic;
    page-break-inside: avoid;
    break-inside: avoid;
}

/* Tables - prevent page breaks */
table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 1.5em;
    page-break-inside: avoid;
    break-inside: avoid;
    font-size: 0.95em;
}

/* Keep table headers with content */
thead {
    display: table-header-group;
}

tbody {
    display: table-row-group;
}

th, td {
    padding: 0.75em;
    text-align: left;
    border: 1px solid #ddd;
}

th {
    background-color: #2c5aa0;
    color: white;
    font-weight: 600;
}

tr:nth-child(even) {
    background-color: #f9f9f9;
}

/* Horizontal rules */
hr {
    border: none;
    border-top: 2px solid #e0e0e0;
    margin: 2em 0;
    page-break-after: avoid;
    break-after: avoid;
}

/* Images */
img {
    max-width: 100%;
    height: auto;
    display: block;
    margin: 1em auto;
    page-break-inside: avoid;
    break-inside: avoid;
}

/* Task lists */
input[type="checkbox"] {
    margin-right: 0.5em;
}

/* Strong and emphasis */
strong, b {
    font-weight: 600;
    color: #1a1a1a;
}

em, i {
    font-style: italic;
}

/* Definition lists */
dl {
    margin-bottom: 1em;
}

dt {
    font-weight: 600;
    margin-top: 0.5em;
}

dd {
    margin-left: 2em;
    margin-bottom: 0.5em;
}

/* Prevent page breaks in specific elements */
figure, .no-break {
    page-break-inside: avoid;
    break-inside: avoid;
}

/* First page special styling */
body > h1:first-child {
    margin-top: 0;
    padding-top: 0;
}
"#;

/// Generate complete HTML document from content and CSS
pub fn generate_html(content: &str, css: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Markdown to PDF</title>
    <style>
{}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        css, content
    )
}

/// Load CSS from file or use default
pub fn load_css(css_path: Option<&Path>) -> Result<String> {
    match css_path {
        Some(path) => fs::read_to_string(path).map_err(|e| Md2PdfError::FileRead {
            path: path.to_path_buf(),
            source: e,
        }),
        None => Ok(DEFAULT_CSS.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_css_not_empty() {
        assert!(!DEFAULT_CSS.is_empty());
        assert!(DEFAULT_CSS.contains("@page"));
        assert!(DEFAULT_CSS.contains("page-break-inside"));
    }

    #[test]
    fn test_generate_html() {
        let content = "<h1>Test</h1><p>Content</p>";
        let css = "body { color: red; }";
        let html = generate_html(content, css);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains(content));
        assert!(html.contains(css));
    }

    #[test]
    fn test_load_css_default() {
        let css = load_css(None).unwrap();
        assert_eq!(css, DEFAULT_CSS);
    }
}
