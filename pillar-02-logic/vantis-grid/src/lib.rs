//! Vantis Grid - AI-powered spreadsheet application
//!
//! Features:
//! - Neural Engine for trend prediction
//! - Support for 10GB+ datasets
//! - Real-time collaboration
//! - Advanced formulas and functions
//! - GPU-accelerated calculations

pub mod collaboration;
pub mod core;
pub mod engine;
pub mod export;
pub mod formulas;

pub use collaboration::{ChangeTracker, CollaborationManager};
pub use core::{Cell, CellValue, Column, Grid, Row, Workbook, Worksheet};
pub use engine::{NeuralEngine, PredictionModel, TrendAnalysis};
pub use export::{ExportFormat, Exporter};
pub use formulas::{FormulaEngine, FunctionRegistry};

/// Vantis Grid version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize Vantis Grid
pub fn init() -> Result<(), GridError> {
    // Initialize subsystems
    core::init()?;
    engine::init()?;
    formulas::init()?;

    Ok(())
}

/// Grid-specific errors
#[derive(Debug, thiserror::Error)]
pub enum GridError {
    #[error("Cell reference error: {0}")]
    CellReference(String),

    #[error("Formula error: {0}")]
    Formula(String),

    #[error("Calculation error: {0}")]
    Calculation(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Neural engine error: {0}")]
    NeuralEngine(String),

    #[error("Collaboration error: {0}")]
    Collaboration(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("General error: {0}")]
    General(String),
}

impl From<String> for GridError {
    fn from(s: String) -> Self {
        GridError::General(s)
    }
}
