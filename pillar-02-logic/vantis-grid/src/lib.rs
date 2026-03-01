//! Vantis Grid - AI-powered spreadsheet application
//! 
//! Features:
//! - Neural Engine for trend prediction
//! - Support for 10GB+ datasets
//! - Real-time collaboration
//! - Advanced formulas and functions
//! - GPU-accelerated calculations

pub mod core;
pub mod engine;
pub mod formulas;
pub mod collaboration;
pub mod export;

pub use core::{Grid, Cell, Row, Column, Worksheet, Workbook, CellValue};
pub use engine::{NeuralEngine, PredictionModel, TrendAnalysis};
pub use formulas::{FormulaEngine, FunctionRegistry};
pub use collaboration::{CollaborationManager, ChangeTracker};
pub use export::{ExportFormat, Exporter};

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