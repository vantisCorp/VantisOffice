//! Export module for saving canvases in various formats
//!
//! Supports: Vantis native, PDF, PNG, SVG, PowerPoint

use crate::core::Canvas;
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Vantis,
    Pdf,
    Png,
    Svg,
    Powerpoint,
}

/// Canvas exporter
pub struct CanvasExporter {
    format: ExportFormat,
}

impl CanvasExporter {
    pub fn new(format: ExportFormat) -> Self {
        CanvasExporter { format }
    }

    /// Export canvas to file
    pub fn export(&self, canvas: &Canvas, path: &Path) -> Result<(), ExportError> {
        match self.format {
            ExportFormat::Vantis => self.export_vantis(canvas, path),
            ExportFormat::Pdf => self.export_pdf(canvas, path),
            ExportFormat::Png => self.export_png(canvas, path),
            ExportFormat::Svg => self.export_svg(canvas, path),
            ExportFormat::Powerpoint => self.export_powerpoint(canvas, path),
        }
    }

    /// Export to Vantis native format
    fn export_vantis(&self, canvas: &Canvas, path: &Path) -> Result<(), ExportError> {
        let serialized = serde_json::to_string_pretty(canvas)
            .map_err(|e| ExportError::SerializationError(e.to_string()))?;

        std::fs::write(path, serialized).map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export to PDF format
    fn export_pdf(&self, canvas: &Canvas, path: &Path) -> Result<(), ExportError> {
        // This would use a library like printpdf or genpdf
        // For now, we'll create a placeholder implementation

        let mut output = String::new();
        output.push_str("% Vantis Canvas - PDF Export\n");
        output.push_str(&format!("Canvas: {}\n", canvas.name));
        output.push_str(&format!("Slides: {}\n", canvas.slides.len()));
        output.push_str(&format!(
            "Dimensions: {}x{}\n",
            canvas.dimensions.width, canvas.dimensions.height
        ));

        for slide in &canvas.slides {
            output.push_str(&format!("\nSlide: {}\n", slide.name));
            output.push_str(&format!("  Layers: {}\n", slide.layers.len()));

            for layer in &slide.layers {
                output.push_str(&format!(
                    "    Layer: {} (visible: {})\n",
                    layer.name, layer.visible
                ));
                output.push_str(&format!("      Shapes: {}\n", layer.shapes.len()));
                output.push_str(&format!("      Texts: {}\n", layer.texts.len()));
                output.push_str(&format!("      Images: {}\n", layer.images.len()));
            }
        }

        std::fs::write(path, output).map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export to PNG format
    fn export_png(&self, canvas: &Canvas, path: &Path) -> Result<(), ExportError> {
        // This would use a library like image or png
        // For now, we'll create a placeholder implementation

        let mut output = String::new();
        output.push_str("% Vantis Canvas - PNG Export\n");
        output.push_str(&format!("Canvas: {}\n", canvas.name));
        output.push_str(&format!("Slides: {}\n", canvas.slides.len()));

        std::fs::write(path, output).map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export to SVG format
    fn export_svg(&self, canvas: &Canvas, path: &Path) -> Result<(), ExportError> {
        let mut svg = String::new();

        svg.push_str("<?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>\n");
        svg.push_str("<svg xmlns=&quot;http://www.w3.org/2000/svg&quot; ");
        svg.push_str(&format!(
            "width=&quot;{}&quot; height=&quot;{}&quot; ",
            canvas.dimensions.width, canvas.dimensions.height
        ));
        svg.push_str("viewBox=&quot;0 0 ");
        svg.push_str(&format!(
            "{} {}&quot;>\n",
            canvas.dimensions.width, canvas.dimensions.height
        ));

        // Add background
        match &canvas.background {
            crate::core::Background::Solid(color) => {
                svg.push_str(&format!("  <rect width=&quot;100%&quot; height=&quot;100%&quot; fill=&quot;{}&quot;/>\n", color));
            }
            _ => {
                svg.push_str("  <rect width=&quot;100%&quot; height=&quot;100%&quot; fill=&quot;white&quot;/>\n");
            }
        }

        // Add slides
        for slide in &canvas.slides {
            svg.push_str(&format!("  <!-- Slide: {} -->\n", slide.name));

            for layer in &slide.layers {
                if !layer.visible {
                    continue;
                }

                svg.push_str(&format!("  <!-- Layer: {} -->\n", layer.name));

                // Add shapes
                for shape in &layer.shapes {
                    self.export_shape_svg(shape, &mut svg);
                }

                // Add texts
                for text in &layer.texts {
                    self.export_text_svg(text, &mut svg);
                }

                // Add images
                for image in &layer.images {
                    self.export_image_svg(image, &mut svg);
                }
            }
        }

        svg.push_str("</svg>\n");

        std::fs::write(path, svg).map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export shape to SVG
    fn export_shape_svg(&self, shape: &crate::core::Shape, svg: &mut String) {
        let fill = match &shape.fill {
            Some(crate::core::Fill::Solid(color)) => color.clone(),
            Some(crate::core::Fill::None) => "none".to_string(),
            _ => "gray".to_string(),
        };

        let stroke = match &shape.stroke {
            Some(stroke) => &stroke.color,
            None => "none",
        };

        let stroke_width = match &shape.stroke {
            Some(stroke) => stroke.width,
            None => 0.0,
        };

        svg.push_str(&format!(
            "    <g transform=&quot;translate({}, {}) rotate({})&quot;>\n",
            shape.position.x, shape.position.y, shape.rotation
        ));

        match &shape.shape_type {
            crate::core::ShapeType::Rectangle => {
                svg.push_str(&format!("      <rect x=&quot;0&quot; y=&quot;0&quot; width=&quot;{}&quot; height=&quot;{}&quot; fill=&quot;{}&quot; stroke=&quot;{}&quot; stroke-width=&quot;{}&quot;/>\n",
                    shape.size.width, shape.size.height, fill, stroke, stroke_width));
            }
            crate::core::ShapeType::Circle => {
                let radius = shape.size.width / 2.0;
                svg.push_str(&format!("      <circle cx=&quot;{}&quot; cy=&quot;{}&quot; r=&quot;{}&quot; fill=&quot;{}&quot; stroke=&quot;{}&quot; stroke-width=&quot;{}&quot;/>\n",
                    radius, radius, radius, fill, stroke, stroke_width));
            }
            crate::core::ShapeType::Ellipse => {
                svg.push_str(&format!("      <ellipse cx=&quot;{}&quot; cy=&quot;{}&quot; rx=&quot;{}&quot; ry=&quot;{}&quot; fill=&quot;{}&quot; stroke=&quot;{}&quot; stroke-width=&quot;{}&quot;/>\n",
                    shape.size.width / 2.0, shape.size.height / 2.0,
                    shape.size.width / 2.0, shape.size.height / 2.0,
                    fill, stroke, stroke_width));
            }
            _ => {
                svg.push_str("      <!-- Unsupported shape type -->\n");
            }
        }

        svg.push_str("    </g>\n");
    }

    /// Export text to SVG
    fn export_text_svg(&self, text: &crate::core::Text, svg: &mut String) {
        let fill = match &text.fill {
            Some(crate::core::Fill::Solid(color)) => color.clone(),
            _ => "black".to_string(),
        };

        svg.push_str(&format!("    <text x=&quot;{}&quot; y=&quot;{}&quot; font-family=&quot;{}&quot; font-size=&quot;{}&quot; fill=&quot;{}&quot;>\n",
            text.position.x, text.position.y, text.font.family, text.font.size, fill));
        svg.push_str(&format!("      {}\n", text.content));
        svg.push_str("    </text>\n");
    }

    /// Export image to SVG
    fn export_image_svg(&self, image: &crate::core::Image, svg: &mut String) {
        svg.push_str(&format!("    <image x=&quot;{}&quot; y=&quot;{}&quot; width=&quot;{}&quot; height=&quot;{}&quot; href=&quot;{}&quot; opacity=&quot;{}&quot;/>\n",
            image.position.x, image.position.y, image.size.width, image.size.height,
            image.path, image.opacity));
    }

    /// Export to PowerPoint format
    fn export_powerpoint(&self, canvas: &Canvas, path: &Path) -> Result<(), ExportError> {
        // This would use a library like rust-pptx
        // For now, we'll create a placeholder implementation

        let mut output = String::new();
        output.push_str("% Vantis Canvas - PowerPoint Export\n");
        output.push_str(&format!("Canvas: {}\n", canvas.name));
        output.push_str(&format!("Slides: {}\n", canvas.slides.len()));

        for slide in &canvas.slides {
            output.push_str(&format!("\nSlide: {}\n", slide.name));
            output.push_str(&format!("  Duration: {:?}\n", slide.duration));
            output.push_str(&format!("  Transition: {:?}\n", slide.transition));
        }

        std::fs::write(path, output).map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export canvas to bytes
    pub fn export_to_bytes(&self, canvas: &Canvas) -> Result<Vec<u8>, ExportError> {
        match self.format {
            ExportFormat::Vantis => {
                let serialized = serde_json::to_vec(canvas)
                    .map_err(|e| ExportError::SerializationError(e.to_string()))?;
                Ok(serialized)
            }
            _ => Err(ExportError::UnsupportedFormat(
                "Export to bytes not supported for this format".to_string(),
            )),
        }
    }
}

/// Export errors
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Rendering error: {0}")]
    RenderingError(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Export failed: {0}")]
    ExportFailed(String),
}

/// Initialize export module
pub fn init() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_exporter_creation() {
        let exporter = CanvasExporter::new(ExportFormat::Svg);
        assert_eq!(exporter.format, ExportFormat::Svg);
    }

    #[test]
    fn test_svg_export() {
        let canvas = Canvas::new("Test Canvas".to_string());

        let temp_path = PathBuf::from("/tmp/test_canvas_export.svg");
        let exporter = CanvasExporter::new(ExportFormat::Svg);

        let result = exporter.export(&canvas, &temp_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_export() {
        let canvas = Canvas::new("Test Canvas".to_string());

        let temp_path = PathBuf::from("/tmp/test_canvas_export.json");
        let exporter = CanvasExporter::new(ExportFormat::Vantis);

        let result = exporter.export(&canvas, &temp_path);
        assert!(result.is_ok());
    }
}
