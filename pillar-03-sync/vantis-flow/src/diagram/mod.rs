//! Diagram module for Vantis Flow

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{FlowError, FlowResult, core::{Canvas, Element, ElementType, Connection, ConnectionType, Style, Color}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindMap {
    pub id: Uuid,
    pub name: String,
    pub root: MindMapNode,
    pub nodes: HashMap<Uuid, MindMapNode>,
    pub layout: MindMapLayout,
    pub canvas: Canvas,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl MindMap {
    pub fn new(name: impl Into<String>, root_text: impl Into<String>) -> Self {
        let root_id = Uuid::new_v4();
        let root = MindMapNode::new(root_id, root_text);
        let mut nodes = HashMap::new();
        nodes.insert(root_id, root.clone());
        let now = Utc::now();
        let name_str = name.into();
        let mut canvas = Canvas::new(name_str.clone());
        let root_element = Element::new(ElementType::RoundedRectangle, 960.0, 540.0)
            .with_text(root.text.clone())
            .with_size(200.0, 80.0);
        canvas.add_element(root_element).ok();
        Self {
            id: Uuid::new_v4(),
            name: name_str,
            root,
            nodes,
            layout: MindMapLayout::Radial,
            canvas,
            created_at: now,
            modified_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindMapNode {
    pub id: Uuid,
    pub text: String,
    pub children: Vec<Uuid>,
    pub color: Color,
    pub level: usize,
    pub collapsed: bool,
}

impl MindMapNode {
    pub fn new(id: Uuid, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
            children: Vec::new(),
            color: Color::WHITE,
            level: 0,
            collapsed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MindMapLayout {
    Radial,
    Tree,
    LeftRight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flowchart {
    pub id: Uuid,
    pub name: String,
    pub nodes: HashMap<Uuid, FlowchartNode>,
    pub edges: Vec<FlowchartEdge>,
    pub canvas: Canvas,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl Flowchart {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        let name_str = name.into();
        Self {
            id: Uuid::new_v4(),
            name: name_str.clone(),
            nodes: HashMap::new(),
            edges: Vec::new(),
            canvas: Canvas::new(name_str),
            created_at: now,
            modified_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowchartNode {
    pub id: Uuid,
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub element_type: ElementType,
    pub style: Style,
    pub node_type: FlowchartNodeType,
}

impl FlowchartNode {
    pub fn new(text: impl Into<String>, x: f64, y: f64, node_type: FlowchartNodeType) -> Self {
        let (element_type, width, height) = match node_type {
            FlowchartNodeType::Start => (ElementType::RoundedRectangle, 120.0, 50.0),
            FlowchartNodeType::End => (ElementType::RoundedRectangle, 120.0, 50.0),
            FlowchartNodeType::Process => (ElementType::Rectangle, 150.0, 60.0),
            FlowchartNodeType::Decision => (ElementType::Diamond, 120.0, 80.0),
            FlowchartNodeType::Input => (ElementType::Parallelogram, 150.0, 60.0),
            FlowchartNodeType::Output => (ElementType::Parallelogram, 150.0, 60.0),
            FlowchartNodeType::Connector => (ElementType::Circle, 40.0, 40.0),
        };
        Self {
            id: Uuid::new_v4(),
            text: text.into(),
            x,
            y,
            width,
            height,
            element_type,
            style: Style::default(),
            node_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FlowchartNodeType {
    Start,
    End,
    Process,
    Decision,
    Input,
    Output,
    Connector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowchartEdge {
    pub id: Uuid,
    pub source: Uuid,
    pub target: Uuid,
    pub label: Option<String>,
    pub connection_type: ConnectionType,
    pub style: crate::core::Stroke,
}

impl FlowchartEdge {
    pub fn new(source: Uuid, target: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            target,
            label: None,
            connection_type: ConnectionType::Straight,
            style: crate::core::Stroke::default(),
        }
    }
}

pub struct DiagramRenderer {
    pub options: RenderOptions,
}

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub scale: f64,
    pub show_grid: bool,
    pub show_labels: bool,
    pub anti_aliasing: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            scale: 1.0,
            show_grid: true,
            show_labels: true,
            anti_aliasing: true,
        }
    }
}

impl DiagramRenderer {
    pub fn new() -> Self {
        Self {
            options: RenderOptions::default(),
        }
    }
    
    pub fn render_to_svg(&self, canvas: &Canvas) -> FlowResult<String> {
        let mut result = String::new();
        result.push_str("<svg>");
        result.push_str("</svg>");
        Ok(result)
    }
}
