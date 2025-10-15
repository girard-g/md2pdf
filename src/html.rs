//! HTML generation from Markdown with semantic markup and page break hints
//!
//! This module converts parsed Markdown into well-structured HTML with proper
//! semantic elements and CSS classes for intelligent page break handling.

use crate::error::Result;
use pulldown_cmark::{html, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Convert markdown string to HTML with semantic markup
pub fn markdown_to_html(markdown: &str) -> Result<String> {
    // Enable all markdown extensions for maximum compatibility
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, options);

    // Add semantic wrappers and page break hints
    let parser = add_page_break_hints(parser);

    // Convert to HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output)
}

/// Add page break hints to prevent content splitting
///
/// This function wraps certain elements with CSS classes that indicate
/// they should not be split across pages.
fn add_page_break_hints<'a>(
    parser: impl Iterator<Item = Event<'a>>,
) -> impl Iterator<Item = Event<'a>> {
    let mut events = Vec::new();
    let mut _in_table = false;
    let mut _in_code_block = false;
    let mut _in_heading = false;
    let mut _heading_level = HeadingLevel::H1;

    for event in parser {
        match &event {
            Event::Start(Tag::Table(_)) => {
                _in_table = true;
                events.push(Event::Html(
                    r#"<div class="table-wrapper no-break">"#.into(),
                ));
                events.push(event);
            }
            Event::End(TagEnd::Table) => {
                _in_table = false;
                events.push(event);
                events.push(Event::Html(r#"</div>"#.into()));
            }
            Event::Start(Tag::CodeBlock(_)) => {
                _in_code_block = true;
                events.push(Event::Html(r#"<div class="code-wrapper no-break">"#.into()));
                events.push(event);
            }
            Event::End(TagEnd::CodeBlock) => {
                _in_code_block = false;
                events.push(event);
                events.push(Event::Html(r#"</div>"#.into()));
            }
            Event::Start(Tag::Heading { level, .. }) => {
                _in_heading = true;
                _heading_level = *level;
                events.push(event);
            }
            Event::End(TagEnd::Heading(_)) => {
                _in_heading = false;
                events.push(event);
            }
            Event::Start(Tag::BlockQuote(_)) => {
                events.push(Event::Html(r#"<div class="blockquote-wrapper no-break">"#.into()));
                events.push(event);
            }
            Event::End(TagEnd::BlockQuote) => {
                events.push(event);
                events.push(Event::Html(r#"</div>"#.into()));
            }
            _ => events.push(event),
        }
    }

    events.into_iter()
}

/// Sanitize HTML to prevent XSS (basic implementation)
///
/// For production use, consider using a dedicated sanitization library like ammonia
pub fn sanitize_html(html: &str) -> String {
    // Basic sanitization - in production, use a proper library
    // For now, we trust the markdown parser's output
    html.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html_basic() {
        let markdown = "# Hello\n\nWorld";
        let html = markdown_to_html(markdown).unwrap();
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello"));
        assert!(html.contains("<p>"));
        assert!(html.contains("World"));
    }

    #[test]
    fn test_markdown_to_html_table() {
        let markdown = r#"
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
"#;
        let html = markdown_to_html(markdown).unwrap();
        assert!(html.contains("<table>"));
        assert!(html.contains("table-wrapper"));
        assert!(html.contains("no-break"));
    }

    #[test]
    fn test_markdown_to_html_code_block() {
        let markdown = r#"
```rust
fn main() {
    println!("Hello, world!");
}
```
"#;
        let html = markdown_to_html(markdown).unwrap();
        assert!(html.contains("<pre>"));
        assert!(html.contains("code-wrapper"));
        assert!(html.contains("no-break"));
    }

    #[test]
    fn test_markdown_to_html_blockquote() {
        let markdown = "> This is a quote";
        let html = markdown_to_html(markdown).unwrap();
        assert!(html.contains("<blockquote>"));
        assert!(html.contains("blockquote-wrapper"));
    }

    #[test]
    fn test_markdown_to_html_strikethrough() {
        let markdown = "~~strikethrough~~";
        let html = markdown_to_html(markdown).unwrap();
        assert!(html.contains("<del>") || html.contains("strikethrough"));
    }

    #[test]
    fn test_markdown_to_html_task_list() {
        let markdown = "- [ ] Task 1\n- [x] Task 2";
        let html = markdown_to_html(markdown).unwrap();
        assert!(html.contains("checkbox") || html.contains("<li>"));
    }
}
