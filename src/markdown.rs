//! Markdown parsing and validation
//!
//! This module handles reading markdown files and validating their content.

use crate::error::{Md2PdfError, Result};
use std::fs;
use std::path::Path;

/// Read and validate a markdown file
pub fn read_markdown_file(path: &Path) -> Result<String> {
    // Validate file extension
    if !path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("md"))
        .unwrap_or(false)
    {
        return Err(Md2PdfError::InvalidExtension(path.to_path_buf()));
    }

    // Check if file exists
    if !path.exists() {
        return Err(Md2PdfError::InvalidPath(path.to_path_buf()));
    }

    // Read file content
    fs::read_to_string(path).map_err(|e| Md2PdfError::FileRead {
        path: path.to_path_buf(),
        source: e,
    })
}

/// Validate markdown content (basic checks)
pub fn validate_markdown(content: &str) -> Result<()> {
    if content.trim().is_empty() {
        return Err(Md2PdfError::MarkdownParse(
            "Markdown file is empty".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_validate_markdown_empty() {
        let result = validate_markdown("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_markdown_valid() {
        let result = validate_markdown("# Hello\n\nWorld");
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_markdown_file_invalid_extension() {
        let result = read_markdown_file(Path::new("test.txt"));
        assert!(matches!(result, Err(Md2PdfError::InvalidExtension(_))));
    }

    #[test]
    fn test_read_markdown_file_not_exists() {
        let result = read_markdown_file(Path::new("nonexistent.md"));
        assert!(matches!(result, Err(Md2PdfError::InvalidPath(_))));
    }

    #[test]
    fn test_read_markdown_file_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "# Test\n\nContent";
        temp_file.write_all(content.as_bytes()).unwrap();

        // Create a path with .md extension
        let temp_path = temp_file.path();
        let md_path = temp_path.with_extension("md");
        std::fs::copy(temp_path, &md_path).unwrap();

        let result = read_markdown_file(&md_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);

        std::fs::remove_file(md_path).unwrap();
    }
}
