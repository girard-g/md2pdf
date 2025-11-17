//! # md2pdf - Professional Markdown to PDF Converter
//!
//! A high-quality CLI tool for converting Markdown files to PDF with intelligent
//! page break handling and professional styling.
//!
//! ## Features
//!
//! - Smart page breaks that avoid splitting tables, code blocks, and lists
//! - Professional CSS styling optimized for business documents
//! - Custom CSS support for branding and styling
//! - Headless Chrome rendering for accurate PDF generation
//! - Comprehensive error handling
//!
//! ## Architecture
//!
//! The library is organized into several modules:
//!
//! - `error`: Custom error types using thiserror
//! - `markdown`: Markdown file reading and validation
//! - `html`: HTML generation with semantic markup
//! - `pdf`: PDF generation using headless Chrome
//! - `template`: HTML templating and CSS styling
//!
//! ## Example
//!
//! ```rust,no_run
//! use md2pdf::{convert_markdown_to_pdf, ConversionOptions};
//! use std::path::Path;
//!
//! let options = ConversionOptions::default();
//! convert_markdown_to_pdf(
//!     Path::new("input.md"),
//!     Path::new("output.pdf"),
//!     &options
//! ).expect("Conversion failed");
//! ```

pub mod error;
pub mod html;
pub mod markdown;
pub mod pdf;
pub mod template;

use error::Result;
use log::{debug, info};
use std::path::Path;

/// Options for markdown to PDF conversion
#[derive(Debug, Clone)]
pub struct ConversionOptions {
    /// Optional custom CSS file path
    pub custom_css_path: Option<String>,
    /// PDF generation configuration
    pub pdf_config: pdf::PdfConfig,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            custom_css_path: None,
            pdf_config: pdf::PdfConfig::default(),
            verbose: false,
        }
    }
}

/// Convert a single Markdown file to PDF
///
/// This is the main entry point for the conversion process. It orchestrates
/// the entire pipeline: reading markdown, converting to HTML, applying styles,
/// and generating the PDF.
///
/// # Arguments
///
/// * `input_path` - Path to the input markdown file (.md)
/// * `output_path` - Path where the PDF should be written (.pdf)
/// * `options` - Conversion options including custom CSS and PDF config
///
/// # Errors
///
/// Returns an error if:
/// - The input file cannot be read
/// - The markdown is invalid
/// - HTML generation fails
/// - PDF generation fails
/// - The output file cannot be written
///
/// # Example
///
/// ```rust,no_run
/// use md2pdf::{convert_markdown_to_pdf, ConversionOptions};
/// use std::path::Path;
///
/// let options = ConversionOptions::default();
/// convert_markdown_to_pdf(
///     Path::new("document.md"),
///     Path::new("document.pdf"),
///     &options
/// ).expect("Failed to convert");
/// ```
pub fn convert_markdown_to_pdf(
    input_path: &Path,
    output_path: &Path,
    options: &ConversionOptions,
) -> Result<()> {
    info!(
        "Starting conversion: {} -> {}",
        input_path.display(),
        output_path.display()
    );

    let html_title = match output_path.file_stem() {
        Some(stem) => stem.to_string_lossy().to_string(),
        None => "Document".to_string(),
    };
    // .and_then(|s| s.to_str())
    // .unwrap_or("Document");

    // Step 1: Read and validate markdown file
    debug!("Reading markdown file: {}", input_path.display());
    let markdown_content = markdown::read_markdown_file(input_path)?;
    markdown::validate_markdown(&markdown_content)?;

    // Step 2: Convert markdown to HTML
    debug!("Converting markdown to HTML");
    let html_content = html::markdown_to_html(&markdown_content)?;

    // Step 3: Load CSS (custom or default)
    debug!("Loading CSS");
    let css = match &options.custom_css_path {
        Some(css_path) => template::load_css(Some(Path::new(css_path)))?,
        None => template::load_css(None)?,
    };

    // Step 4: Generate complete HTML document
    debug!("Generating complete HTML document");
    let full_html = template::generate_html(&html_content, &css, &html_title);

    // Step 5: Prepare output path
    debug!("Preparing output path: {}", output_path.display());
    pdf::prepare_output_path(output_path)?;

    // Step 6: Generate PDF
    debug!("Generating PDF");
    pdf::generate_pdf(&full_html, output_path, &options.pdf_config)?;

    info!("Conversion completed successfully");
    Ok(())
}

/// Convert multiple Markdown files to PDFs
///
/// Batch conversion that processes multiple markdown files. Each file is
/// converted independently, and errors for individual files are collected
/// and returned.
///
/// # Arguments
///
/// * `conversions` - Vector of (input_path, output_path) tuples
/// * `options` - Shared conversion options for all files
///
/// # Returns
///
/// A vector of Results, one for each conversion. Successful conversions
/// return Ok(()), while failed conversions return the error.
///
/// # Example
///
/// ```rust,no_run
/// use md2pdf::{convert_multiple_files, ConversionOptions};
/// use std::path::PathBuf;
///
/// let conversions = vec![
///     (PathBuf::from("doc1.md"), PathBuf::from("doc1.pdf")),
///     (PathBuf::from("doc2.md"), PathBuf::from("doc2.pdf")),
/// ];
///
/// let options = ConversionOptions::default();
/// let results = convert_multiple_files(&conversions, &options);
///
/// for (i, result) in results.iter().enumerate() {
///     match result {
///         Ok(_) => println!("File {} converted successfully", i),
///         Err(e) => eprintln!("File {} failed: {}", i, e),
///     }
/// }
/// ```
pub fn convert_multiple_files(
    conversions: &[(impl AsRef<Path>, impl AsRef<Path>)],
    options: &ConversionOptions,
) -> Vec<Result<()>> {
    conversions
        .iter()
        .map(|(input, output)| convert_markdown_to_pdf(input.as_ref(), output.as_ref(), options))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_conversion_options_default() {
        let options = ConversionOptions::default();
        assert!(options.custom_css_path.is_none());
        assert!(!options.verbose);
    }

    #[test]
    fn test_convert_markdown_to_pdf_invalid_input() {
        let options = ConversionOptions::default();
        let result = convert_markdown_to_pdf(
            Path::new("nonexistent.md"),
            Path::new("output.pdf"),
            &options,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_markdown_to_pdf_invalid_output_extension() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"# Test").unwrap();
        let temp_path = temp_file.path().with_extension("md");
        std::fs::copy(temp_file.path(), &temp_path).unwrap();

        let options = ConversionOptions::default();
        let result = convert_markdown_to_pdf(&temp_path, Path::new("output.txt"), &options);

        std::fs::remove_file(temp_path).unwrap();
        assert!(result.is_err());
    }
}
