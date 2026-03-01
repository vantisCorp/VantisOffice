//! Vantis Flow - Planning and Diagrams Module

pub mod core;
pub mod diagram;
pub mod planning;
pub mod collaboration;
pub mod export;

pub use core::{
    Canvas, Element, ElementType,
    Connection, ConnectionType,
    Style, Color, Stroke,
};

pub use diagram::{
    MindMap, MindMapNode, MindMapLayout,
    Flowchart, FlowchartNode, FlowchartEdge, FlowchartNodeType,
    DiagramRenderer,
};

pub use planning::{
    Task, TaskStatus, TaskPriority,
    Project, GanttChart, KanbanBoard,
    Timeline, Milestone,
    TaskDependency, DependencyType,
};

pub use collaboration::{
    FlowSession, FlowUser, FlowChange,
    FlowCRDT, ConflictResolver,
};

pub use export::{
    FlowExporter, ExportFormat,
    export_to_svg, export_to_json,
};

use thiserror::Error;

/// Main error type for Vantis Flow
#[derive(Error, Debug)]
pub enum FlowError {
    #[error("Canvas error: {0}")]
    CanvasError(String),
    
    #[error("Element error: {0}")]
    ElementError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Diagram error: {0}")]
    DiagramError(String),
    
    #[error("Planning error: {0}")]
    PlanningError(String),
    
    #[error("Collaboration error: {0}")]
    CollaborationError(String),
    
    #[error("Export error: {0}")]
    ExportError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Result type for Vantis Flow operations
pub type FlowResult<T> = Result<T, FlowError>;

/// Version of Vantis Flow
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
