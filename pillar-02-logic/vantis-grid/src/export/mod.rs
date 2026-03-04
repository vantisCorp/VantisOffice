//! Export module for saving spreadsheets in various formats
//!
//! Supports: Vantis native, Excel (.xlsx), CSV, PDF, JSON

use crate::core::{CellValue, Workbook};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Vantis,
    Excel,
    Csv,
    Pdf,
    Json,
}

/// Exporter for saving workbooks
pub struct Exporter {
    format: ExportFormat,
}

impl Exporter {
    pub fn new(format: ExportFormat) -> Self {
        Exporter { format }
    }

    /// Export workbook to file
    pub fn export(&self, workbook: &Workbook, path: &Path) -> Result<(), ExportError> {
        match self.format {
            ExportFormat::Vantis => self.export_vantis(workbook, path),
            ExportFormat::Excel => self.export_excel(workbook, path),
            ExportFormat::Csv => self.export_csv(workbook, path),
            ExportFormat::Pdf => self.export_pdf(workbook, path),
            ExportFormat::Json => self.export_json(workbook, path),
        }
    }

    /// Export to Vantis native format
    fn export_vantis(&self, workbook: &Workbook, path: &Path) -> Result<(), ExportError> {
        let serialized = serde_json::to_string_pretty(workbook)
            .map_err(|e| ExportError::SerializationError(e.to_string()))?;

        std::fs::write(path, serialized).map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export to Excel format
    fn export_excel(&self, workbook: &Workbook, path: &Path) -> Result<(), ExportError> {
        // This would use a library like rust_xlsxwriter or calamine
        // For now, we'll create a placeholder implementation

        let mut output = String::new();
        output.push_str("Vantis Grid - Excel Export\n");
        output.push_str(&format!("Workbook: {}\n", workbook.name));
        output.push_str(&format!("Worksheets: {}\n", workbook.worksheets.len()));

        for worksheet in &workbook.worksheets {
            output.push_str(&format!("\nWorksheet: {}\n", worksheet.name));

            // Find dimensions
            let max_row = worksheet.cells.keys().map(|(r, _)| *r).max().unwrap_or(0);
            let max_col = worksheet.cells.keys().map(|(_, c)| *c).max().unwrap_or(0);

            // Export cells
            for row in 0..=max_row {
                for col in 0..=max_col {
                    if let Some(cell) = worksheet.get_cell(row, col) {
                        let value = match &cell.value {
                            CellValue::Number(n) => n.to_string(),
                            CellValue::Text(s) => s.clone(),
                            CellValue::Boolean(b) => b.to_string(),
                            CellValue::Date(d) => d.to_string(),
                            CellValue::DateTime(dt) => dt.to_string(),
                            CellValue::Formula(f) => format!("={}", f),
                            CellValue::Empty => String::new(),
                            CellValue::Error(e) => format!("ERROR: {}", e),
                        };

                        if !value.is_empty() {
                            output.push_str(&format!(
                                "  {}{}: {}\n",
                                crate::core::column_to_letter(col),
                                row + 1,
                                value
                            ));
                        }
                    }
                }
            }
        }

        std::fs::write(path, output).map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export to CSV format
    fn export_csv(&self, workbook: &Workbook, path: &Path) -> Result<(), ExportError> {
        // Export only the active worksheet
        let worksheet = workbook
            .get_active_worksheet()
            .ok_or_else(|| ExportError::NoActiveWorksheet)?;

        let file = std::fs::File::create(path).map_err(|e| ExportError::IoError(e.to_string()))?;
        let mut writer = BufWriter::new(file);

        // Find dimensions
        let max_row = worksheet.cells.keys().map(|(r, _)| *r).max().unwrap_or(0);
        let max_col = worksheet.cells.keys().map(|(_, c)| *c).max().unwrap_or(0);

        // Write CSV rows
        for row in 0..=max_row {
            let mut row_values = Vec::new();

            for col in 0..=max_col {
                let value = if let Some(cell) = worksheet.get_cell(row, col) {
                    match &cell.value {
                        CellValue::Number(n) => n.to_string(),
                        CellValue::Text(s) => {
                            // Escape quotes and wrap in quotes if contains comma
                            let escaped = s.replace("&quot;", "&quot;&quot;");
                            if s.contains(',') || s.contains('"') || s.contains('\n') {
                                format!("&quot;{}&quot;", escaped)
                            } else {
                                s.clone()
                            }
                        }
                        CellValue::Boolean(b) => b.to_string(),
                        CellValue::Date(d) => d.to_string(),
                        CellValue::DateTime(dt) => dt.to_string(),
                        CellValue::Formula(f) => format!("={}", f),
                        CellValue::Empty => String::new(),
                        CellValue::Error(e) => format!("ERROR: {}", e),
                    }
                } else {
                    String::new()
                };

                row_values.push(value);
            }

            let row_str = row_values.join(",");
            writeln!(writer, "{}", row_str).map_err(|e| ExportError::IoError(e.to_string()))?;
        }

        writer
            .flush()
            .map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export to PDF format
    fn export_pdf(&self, workbook: &Workbook, path: &Path) -> Result<(), ExportError> {
        // This would use a library like printpdf or genpdf
        // For now, we'll create a placeholder implementation

        let mut output = String::new();
        output.push_str("% Vantis Grid - PDF Export\n");
        output.push_str(&format!("Workbook: {}\n", workbook.name));
        output.push_str(&format!("Worksheets: {}\n", workbook.worksheets.len()));

        for worksheet in &workbook.worksheets {
            output.push_str(&format!("\nWorksheet: {}\n", worksheet.name));

            // Find dimensions
            let max_row = worksheet.cells.keys().map(|(r, _)| *r).max().unwrap_or(0);
            let max_col = worksheet.cells.keys().map(|(_, c)| *c).max().unwrap_or(0);

            // Export cells as table
            for row in 0..=max_row {
                for col in 0..=max_col {
                    if let Some(cell) = worksheet.get_cell(row, col) {
                        let value = match &cell.value {
                            CellValue::Number(n) => n.to_string(),
                            CellValue::Text(s) => s.clone(),
                            CellValue::Boolean(b) => b.to_string(),
                            CellValue::Date(d) => d.to_string(),
                            CellValue::DateTime(dt) => dt.to_string(),
                            CellValue::Formula(f) => format!("={}", f),
                            CellValue::Empty => String::new(),
                            CellValue::Error(e) => format!("ERROR: {}", e),
                        };

                        if !value.is_empty() {
                            output.push_str(&format!(
                                "  [{}{}] {}\n",
                                crate::core::column_to_letter(col),
                                row + 1,
                                value
                            ));
                        }
                    }
                }
            }
        }

        std::fs::write(path, output).map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export to JSON format
    fn export_json(&self, workbook: &Workbook, path: &Path) -> Result<(), ExportError> {
        let serialized = serde_json::to_string_pretty(workbook)
            .map_err(|e| ExportError::SerializationError(e.to_string()))?;

        std::fs::write(path, serialized).map_err(|e| ExportError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Export workbook to bytes
    pub fn export_to_bytes(&self, workbook: &Workbook) -> Result<Vec<u8>, ExportError> {
        match self.format {
            ExportFormat::Vantis | ExportFormat::Json => {
                let serialized = serde_json::to_vec(workbook)
                    .map_err(|e| ExportError::SerializationError(e.to_string()))?;
                Ok(serialized)
            }
            _ => Err(ExportError::UnsupportedFormat(
                "Export to bytes not supported for this format".to_string(),
            )),
        }
    }
}

/// Importer for loading workbooks
pub struct Importer {
    format: ExportFormat,
}

impl Importer {
    pub fn new(format: ExportFormat) -> Self {
        Importer { format }
    }

    /// Import workbook from file
    pub fn import(&self, path: &Path) -> Result<Workbook, ImportError> {
        match self.format {
            ExportFormat::Vantis | ExportFormat::Json => self.import_json(path),
            ExportFormat::Csv => self.import_csv(path),
            _ => Err(ImportError::UnsupportedFormat(
                "Import not supported for this format".to_string(),
            )),
        }
    }

    /// Import from JSON/Vantis format
    fn import_json(&self, path: &Path) -> Result<Workbook, ImportError> {
        let content =
            std::fs::read_to_string(path).map_err(|e| ImportError::IoError(e.to_string()))?;

        let workbook: Workbook = serde_json::from_str(&content)
            .map_err(|e| ImportError::DeserializationError(e.to_string()))?;

        Ok(workbook)
    }

    /// Import from CSV format
    fn import_csv(&self, path: &Path) -> Result<Workbook, ImportError> {
        let content =
            std::fs::read_to_string(path).map_err(|e| ImportError::IoError(e.to_string()))?;

        let mut workbook = Workbook::new(
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Imported")
                .to_string(),
        );

        let worksheet = workbook.add_worksheet("Sheet1".to_string());

        // Parse CSV
        for (row_idx, line) in content.lines().enumerate() {
            let mut col_idx = 0;
            let mut in_quotes = false;
            let mut current = String::new();

            for c in line.chars() {
                match c {
                    '"' => {
                        in_quotes = !in_quotes;
                        current.push(c);
                    }
                    ',' if !in_quotes => {
                        // Parse value
                        if let Some(value) = self.parse_csv_value(&current) {
                            worksheet.set_cell_value(row_idx, col_idx, value);
                        }
                        current.clear();
                        col_idx += 1;
                    }
                    _ => {
                        current.push(c);
                    }
                }
            }

            // Last value in row
            if let Some(value) = self.parse_csv_value(&current) {
                worksheet.set_cell_value(row_idx, col_idx, value);
            }
        }

        Ok(workbook)
    }

    /// Parse CSV value
    fn parse_csv_value(&self, value: &str) -> Option<CellValue> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return None;
        }

        // Remove quotes if present
        let unquoted = if trimmed.starts_with('"') && trimmed.ends_with('"') {
            &trimmed[1..trimmed.len() - 1]
        } else {
            trimmed
        };

        // Try to parse as number
        if let Ok(num) = unquoted.parse::<f64>() {
            return Some(CellValue::Number(num));
        }

        // Try to parse as boolean
        match unquoted.to_uppercase().as_str() {
            "TRUE" => return Some(CellValue::Boolean(true)),
            "FALSE" => return Some(CellValue::Boolean(false)),
            _ => {}
        }

        // Treat as text
        Some(CellValue::Text(unquoted.to_string()))
    }
}

/// Export errors
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("No active worksheet")]
    NoActiveWorksheet,

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Export failed: {0}")]
    ExportFailed(String),
}

/// Import errors
#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Import failed: {0}")]
    ImportFailed(String),
}

/// Initialize export module
pub fn init() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{CellValue, Grid};
    use std::path::PathBuf;

    #[test]
    fn test_exporter_creation() {
        let exporter = Exporter::new(ExportFormat::Csv);
        assert_eq!(exporter.format, ExportFormat::Csv);
    }

    #[test]
    fn test_csv_export() {
        let grid = Grid::new("Test".to_string());
        let workbook = grid.get_workbook();
        let workbook = workbook.read().unwrap();

        let temp_path = PathBuf::from("/tmp/test_export.csv");
        let exporter = Exporter::new(ExportFormat::Csv);

        let result = exporter.export(&workbook, &temp_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_export() {
        let grid = Grid::new("Test".to_string());
        let workbook = grid.get_workbook();
        let workbook = workbook.read().unwrap();

        let temp_path = PathBuf::from("/tmp/test_export.json");
        let exporter = Exporter::new(ExportFormat::Json);

        let result = exporter.export(&workbook, &temp_path);
        assert!(result.is_ok());
    }
}
