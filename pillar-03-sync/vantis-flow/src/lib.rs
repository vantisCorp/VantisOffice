//! Vantis Flow - Planning and Diagrams Module

pub mod collaboration;
pub mod core;
pub mod diagram;
pub mod export;
pub mod planning;

pub use core::{Canvas, Color, Connection, ConnectionType, Element, ElementType, Stroke, Style};

pub use diagram::{
    DiagramRenderer, Flowchart, FlowchartEdge, FlowchartNode, FlowchartNodeType, MindMap,
    MindMapLayout, MindMapNode,
};

pub use planning::{
    DependencyType, GanttChart, KanbanBoard, Milestone, Project, Task, TaskDependency,
    TaskPriority, TaskStatus, Timeline,
};

pub use collaboration::{ConflictResolver, FlowCRDT, FlowChange, FlowSession, FlowUser};

pub use export::{export_to_json, export_to_svg, ExportFormat, FlowExporter};

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
