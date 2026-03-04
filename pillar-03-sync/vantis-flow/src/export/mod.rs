//! Export module for Vantis Flow

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use crate::{core::Canvas, diagram::DiagramRenderer, FlowError, FlowResult};

/// Export format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportFormat {
    Svg,
    Png,
    Pdf,
    Json,
    Vml,
    Vantis,
}

/// Flow exporter
pub struct FlowExporter {
    pub options: ExportOptions,
}

/// Export options
#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub include_background: bool,
    pub include_grid: bool,
    pub scale: f64,
    pub dpi: u32,
    pub quality: u8,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            include_background: true,
            include_grid: false,
            scale: 1.0,
            dpi: 300,
            quality: 90,
        }
    }
}

impl FlowExporter {
    pub fn new() -> Self {
        Self {
            options: ExportOptions::default(),
        }
    }

    pub fn export_canvas(
        &self,
        canvas: &Canvas,
        format: ExportFormat,
        path: &str,
    ) -> FlowResult<()> {
        match format {
            ExportFormat::Svg => {
                let svg = self.export_canvas_to_svg(canvas)?;
                let mut file = File::create(path)?;
                file.write_all(svg.as_bytes())?;
            }
            ExportFormat::Json => {
                let json = self.export_canvas_to_json(canvas)?;
                let mut file = File::create(path)?;
                file.write_all(json.as_bytes())?;
            }
            _ => {
                return Err(FlowError::ExportError("Format not implemented".to_string()));
            }
        }
        Ok(())
    }

    pub fn export_canvas_to_svg(&self, canvas: &Canvas) -> FlowResult<String> {
        let renderer = DiagramRenderer::new();
        renderer.render_to_svg(canvas)
    }

    pub fn export_canvas_to_json(&self, canvas: &Canvas) -> FlowResult<String> {
        let json = serde_json::to_string_pretty(canvas)
            .map_err(|e| FlowError::ExportError(format!("JSON serialization error: {}", e)))?;
        Ok(json)
    }
}

pub fn export_to_svg(canvas: &Canvas, path: &str) -> FlowResult<()> {
    let exporter = FlowExporter::new();
    exporter.export_canvas(canvas, ExportFormat::Svg, path)
}

pub fn export_to_json(canvas: &Canvas, path: &str) -> FlowResult<()> {
    let exporter = FlowExporter::new();
    exporter.export_canvas(canvas, ExportFormat::Json, path)
}
