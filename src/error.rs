//! Error types for md2pdf
//!
//! This module defines custom error types using thiserror for ergonomic error handling.
//! All errors implement std::error::Error and can be converted to anyhow::Error.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for md2pdf operations
#[derive(Error, Debug)]
pub enum Md2PdfError {
    #[error("Failed to read file: {path}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file: {path}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid file path: {0}")]
    InvalidPath(PathBuf),

    #[error("Markdown parsing error: {0}")]
    MarkdownParse(String),

    #[error("HTML generation error: {0}")]
    HtmlGeneration(String),

    #[error("PDF generation error: {0}")]
    PdfGeneration(String),

    #[error("CSS file not found: {0}")]
    CssNotFound(PathBuf),

    #[error("Failed to launch Chrome browser: {0}")]
    ChromeLaunch(String),

    #[error("Failed to navigate to page: {0}")]
    ChromeNavigation(String),

    #[error("Failed to generate PDF from Chrome: {0}")]
    ChromePdfGeneration(String),

    #[error("Template error: {0}")]
    Template(String),

    #[error("No input files provided")]
    NoInputFiles,

    #[error("Input file must have .md extension: {0}")]
    InvalidExtension(PathBuf),
}

/// Type alias for Results using Md2PdfError
pub type Result<T> = std::result::Result<T, Md2PdfError>;
