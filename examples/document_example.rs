//! Document creation example using Vantis Writer

use anyhow::Result;
use vantis_writer::{Document, Paragraph, ParagraphStyle};

fn main() -> Result<()> {
    println!("📝 Creating a new document with Vantis Writer...");
    println!();
    
    // Create a new document
    let mut document = Document::new("Welcome to VantisOffice".to_string());
    println!("✅ Created document: {}", document.title);
    println!();
    
    // Add title paragraph
    let title_style = ParagraphStyle {
        font_size: 24.0,
        font_family: "Inter".to_string(),
        bold: true,
        italic: false,
        color: "#2196F3".to_string(),
    };
    
    let title = Paragraph::new("Welcome to VantisOffice".to_string())
        .with_style(title_style);
    document.add_paragraph(title)?;
    println!("✅ Added title paragraph");
    println!();
    
    // Add content paragraph
    let content = Paragraph::new(
        "VantisOffice is a next-generation office ecosystem designed for Vantis OS. \
        It provides secure, private, and performant productivity applications."
            .to_string(),
    );
    document.add_paragraph(content)?;
    println!("✅ Added content paragraph");
    println!();
    
    // Display document statistics
    println!("📊 Document Statistics:");
    println!("  Title: {}", document.title);
    println!("  Paragraphs: {}", document.paragraphs.len());
    println!("  Word count: {}", document.metadata.word_count);
    println!("  Created: {}", document.metadata.created_at);
    println!("  Modified: {}", document.metadata.modified_at);
    println!();
    
    println!("🎉 Document created successfully!");
    
    Ok(())
}
