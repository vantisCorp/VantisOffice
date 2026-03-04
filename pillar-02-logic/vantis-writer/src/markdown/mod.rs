//! Markdown parser and live preview

use anyhow::Result;
use pulldown_cmark::{Event, Parser, Tag};

/// Markdown parser
pub struct MarkdownParser {
    options: pulldown_cmark::Options,
}

impl MarkdownParser {
    /// Create a new markdown parser
    pub fn new() -> Self {
        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);
        options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
        options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
        options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);

        MarkdownParser { options }
    }

    /// Parse markdown to HTML
    pub fn parse_to_html(&self, markdown: &str) -> Result<String> {
        let parser = Parser::new_ext(markdown, self.options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        Ok(html_output)
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Live preview engine
pub struct LivePreview {
    parser: MarkdownParser,
}

impl LivePreview {
    /// Create a new live preview
    pub fn new(parser: MarkdownParser) -> Self {
        LivePreview { parser }
    }

    /// Render markdown to rich text
    pub fn render(&self, markdown: &str) -> Result<String> {
        self.parser.parse_to_html(markdown)
    }
}

impl Default for LivePreview {
    fn default() -> Self {
        LivePreview::new(MarkdownParser::new())
    }
}

/// Initialize markdown parser
pub fn init() -> Result<()> {
    Ok(())
}
