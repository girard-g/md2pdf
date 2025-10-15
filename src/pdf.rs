//! PDF generation using headless Chrome
//!
//! This module handles PDF generation from HTML using headless Chrome,
//! which provides excellent CSS support including page break rules.

use crate::error::{Md2PdfError, Result};
use headless_chrome::{Browser, LaunchOptions};
use log::{debug, info};
use std::fs;
use std::path::Path;
use std::time::Duration;

/// PDF generation configuration
#[derive(Debug, Clone)]
pub struct PdfConfig {
    /// Display header and footer
    pub display_header_footer: bool,
    /// Print background graphics
    pub print_background: bool,
    /// Paper width in inches (8.27 = A4)
    pub paper_width: f64,
    /// Paper height in inches (11.69 = A4)
    pub paper_height: f64,
    /// Top margin in inches
    pub margin_top: f64,
    /// Bottom margin in inches
    pub margin_bottom: f64,
    /// Left margin in inches
    pub margin_left: f64,
    /// Right margin in inches
    pub margin_right: f64,
    /// Scale of the webpage rendering (1.0 = 100%)
    pub scale: f64,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            display_header_footer: false,
            print_background: true,
            paper_width: 8.27,  // A4 width
            paper_height: 11.69, // A4 height
            margin_top: 0.4,
            margin_bottom: 0.4,
            margin_left: 0.4,
            margin_right: 0.4,
            scale: 1.0,
        }
    }
}

/// Generate PDF from HTML content
pub fn generate_pdf(html: &str, output_path: &Path, _config: &PdfConfig) -> Result<()> {
    info!("Starting PDF generation for: {}", output_path.display());

    // Launch headless Chrome
    debug!("Launching headless Chrome browser");
    let browser = launch_browser()?;

    // Create a new tab
    debug!("Creating browser tab");
    let tab = browser
        .new_tab()
        .map_err(|e| Md2PdfError::ChromeLaunch(format!("Failed to create tab: {}", e)))?;

    // Navigate to data URL with HTML content
    debug!("Loading HTML content");
    let data_url = format!("data:text/html;charset=utf-8,{}", urlencoding::encode(html));

    tab.navigate_to(&data_url)
        .map_err(|e| Md2PdfError::ChromeNavigation(format!("Navigation failed: {}", e)))?;

    // Wait for page to load and render
    debug!("Waiting for page to render");
    tab.wait_until_navigated()
        .map_err(|e| Md2PdfError::ChromeNavigation(format!("Wait failed: {}", e)))?;

    // Give additional time for CSS to apply
    std::thread::sleep(Duration::from_millis(500));

    // Generate PDF
    debug!("Generating PDF with configured options");
    let pdf_data = tab
        .print_to_pdf(None)
        .map_err(|e| Md2PdfError::ChromePdfGeneration(format!("PDF generation failed: {}", e)))?;

    // Write PDF to file
    debug!("Writing PDF to: {}", output_path.display());
    fs::write(output_path, pdf_data).map_err(|e| Md2PdfError::FileWrite {
        path: output_path.to_path_buf(),
        source: e,
    })?;

    info!("PDF successfully generated: {}", output_path.display());
    Ok(())
}

/// Launch headless Chrome browser with appropriate options
fn launch_browser() -> Result<Browser> {
    let launch_options = LaunchOptions {
        headless: true,
        sandbox: true,
        enable_gpu: false,
        enable_logging: false,
        window_size: Some((1920, 1080)),
        idle_browser_timeout: Duration::from_secs(30),
        ..Default::default()
    };

    Browser::new(launch_options)
        .map_err(|e| Md2PdfError::ChromeLaunch(format!("Failed to launch browser: {}", e)))
}

/// Validate output path and create parent directories if needed
pub fn prepare_output_path(path: &Path) -> Result<()> {
    // Ensure output has .pdf extension
    if !path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
    {
        return Err(Md2PdfError::InvalidPath(path.to_path_buf()));
    }

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| Md2PdfError::FileWrite {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_config_default() {
        let config = PdfConfig::default();
        assert_eq!(config.paper_width, 8.27);
        assert_eq!(config.paper_height, 11.69);
        assert!(config.print_background);
    }

    #[test]
    fn test_prepare_output_path_invalid_extension() {
        let path = Path::new("/tmp/test.txt");
        let result = prepare_output_path(path);
        assert!(matches!(result, Err(Md2PdfError::InvalidPath(_))));
    }

    #[test]
    fn test_prepare_output_path_valid() {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test.pdf");
        let result = prepare_output_path(&path);
        assert!(result.is_ok());
    }
}
