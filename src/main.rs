//! md2pdf - Professional Markdown to PDF Converter
//!
//! A command-line tool for converting Markdown files to professionally formatted PDFs
//! with intelligent page break handling.

use clap::Parser;
use env_logger::Env;
use log::{error, info, warn};
use md2pdf::{convert_markdown_to_pdf, convert_multiple_files, ConversionOptions};
use std::path::{Path, PathBuf};
use std::process;
use walkdir::WalkDir;

/// Professional Markdown to PDF converter with smart page breaks
#[derive(Parser, Debug)]
#[command(
    name = "md2pdf",
    version,
    about = "Convert Markdown files to professional PDFs",
    long_about = "A professional CLI tool for converting Markdown files to PDF with intelligent \
                  page break handling, custom CSS support, and high-quality output suitable for \
                  business documents."
)]
struct Args {
    /// Input markdown file(s) or directory
    #[arg(
        value_name = "INPUT",
        help = "Input markdown file(s) or directory containing .md files"
    )]
    input: Vec<PathBuf>,

    /// Output PDF file or directory
    #[arg(
        short = 'o',
        long = "output",
        value_name = "OUTPUT",
        help = "Output PDF file or directory"
    )]
    output: Option<PathBuf>,

    /// Custom CSS file for styling
    #[arg(
        short = 'c',
        long = "css",
        value_name = "CSS_FILE",
        help = "Custom CSS file for PDF styling"
    )]
    css: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(
        short = 'v',
        long = "verbose",
        help = "Enable verbose output for debugging"
    )]
    verbose: bool,

    /// Process directories recursively
    #[arg(
        short = 'r',
        long = "recursive",
        help = "Process directories recursively"
    )]
    recursive: bool,

    /// Paper width in inches (default: 8.27 for A4)
    #[arg(
        long = "paper-width",
        value_name = "WIDTH",
        help = "Paper width in inches"
    )]
    paper_width: Option<f64>,

    /// Paper height in inches (default: 11.69 for A4)
    #[arg(
        long = "paper-height",
        value_name = "HEIGHT",
        help = "Paper height in inches"
    )]
    paper_height: Option<f64>,

    /// Top margin in inches
    #[arg(long = "margin-top", value_name = "MARGIN")]
    margin_top: Option<f64>,

    /// Bottom margin in inches
    #[arg(long = "margin-bottom", value_name = "MARGIN")]
    margin_bottom: Option<f64>,

    /// Left margin in inches
    #[arg(long = "margin-left", value_name = "MARGIN")]
    margin_left: Option<f64>,

    /// Right margin in inches
    #[arg(long = "margin-right", value_name = "MARGIN")]
    margin_right: Option<f64>,
}

fn main() {
    let args = Args::parse();

    // Initialize logger
    let filter = if args.verbose {
        "md2pdf=debug,warn"
    } else {
        "md2pdf=info,warn"
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(filter))
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    // Validate inputs
    if args.input.is_empty() {
        error!("No input files provided");
        process::exit(1);
    }

    // Build conversion options
    let mut options = ConversionOptions {
        custom_css_path: args.css.as_ref().map(|p| p.to_string_lossy().to_string()),
        pdf_config: md2pdf::pdf::PdfConfig::default(),
        verbose: args.verbose,
    };

    // Apply custom PDF configuration if provided
    if let Some(width) = args.paper_width {
        options.pdf_config.paper_width = width;
    }
    if let Some(height) = args.paper_height {
        options.pdf_config.paper_height = height;
    }
    if let Some(margin) = args.margin_top {
        options.pdf_config.margin_top = margin;
    }
    if let Some(margin) = args.margin_bottom {
        options.pdf_config.margin_bottom = margin;
    }
    if let Some(margin) = args.margin_left {
        options.pdf_config.margin_left = margin;
    }
    if let Some(margin) = args.margin_right {
        options.pdf_config.margin_right = margin;
    }

    // Collect input files
    let input_files = collect_input_files(&args.input, args.recursive);

    if input_files.is_empty() {
        error!("No markdown files found in input");
        process::exit(1);
    }

    info!("Found {} markdown file(s) to convert", input_files.len());

    // Determine conversion mode and execute
    let exit_code = if input_files.len() == 1 && args.output.is_some() {
        // Single file mode
        convert_single_file(&input_files[0], args.output.as_ref().unwrap(), &options)
    } else if input_files.len() > 1 || args.input[0].is_dir() {
        // Batch mode
        convert_batch(&input_files, args.output.as_deref(), &options)
    } else {
        // Single file, auto output
        let output = PathBuf::from(input_files[0].with_extension("pdf"));
        convert_single_file(&input_files[0], &output, &options)
    };

    process::exit(exit_code);
}

/// Collect all markdown files from input paths
fn collect_input_files(inputs: &[PathBuf], recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for input in inputs {
        if input.is_file() {
            if input
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("md"))
                .unwrap_or(false)
            {
                files.push(input.clone());
            } else {
                warn!("Skipping non-markdown file: {}", input.display());
            }
        } else if input.is_dir() {
            let walker = if recursive {
                WalkDir::new(input).follow_links(true)
            } else {
                WalkDir::new(input).max_depth(1)
            };

            for entry in walker.into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file()
                    && path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.eq_ignore_ascii_case("md"))
                        .unwrap_or(false)
                {
                    files.push(path.to_path_buf());
                }
            }
        } else {
            warn!("Input not found: {}", input.display());
        }
    }

    files
}

/// Convert a single file
fn convert_single_file(input: &Path, output: &Path, options: &ConversionOptions) -> i32 {
    info!("Converting: {} -> {}", input.display(), output.display());

    match convert_markdown_to_pdf(input, output, options) {
        Ok(_) => {
            info!("Conversion successful!");
            0
        }
        Err(e) => {
            error!("Conversion failed: {}", e);
            1
        }
    }
}

/// Convert multiple files in batch mode
fn convert_batch(
    inputs: &[PathBuf],
    output_dir: Option<&Path>,
    options: &ConversionOptions,
) -> i32 {
    // Determine output directory
    let out_dir = match output_dir {
        Some(dir) => {
            if !dir.exists() {
                if let Err(e) = std::fs::create_dir_all(dir) {
                    error!("Failed to create output directory: {}", e);
                    return 1;
                }
            }
            dir.to_path_buf()
        }
        None => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    };

    // Build conversion list
    let conversions: Vec<_> = inputs
        .iter()
        .map(|input| {
            let output = out_dir.join(
                input
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .replace(".md", ".pdf"),
            );
            (input.clone(), output)
        })
        .collect();

    // Execute conversions
    let results = convert_multiple_files(&conversions, options);

    // Report results
    let mut success_count = 0;
    let mut failure_count = 0;

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(_) => {
                info!(
                    "Success: {} -> {}",
                    conversions[i].0.display(),
                    conversions[i].1.display()
                );
                success_count += 1;
            }
            Err(e) => {
                error!("Failed: {} - {}", conversions[i].0.display(), e);
                failure_count += 1;
            }
        }
    }

    info!(
        "Batch conversion completed: {} succeeded, {} failed",
        success_count, failure_count
    );

    if failure_count > 0 {
        1
    } else {
        0
    }
}
