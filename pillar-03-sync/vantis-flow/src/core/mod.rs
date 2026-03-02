//! Core data structures for Vantis Flow
//! 
//! Provides the fundamental canvas, element, and connection structures
//! used across all diagram and planning features.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{FlowError, FlowResult};

/// Main canvas for diagrams and planning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
    /// Unique identifier
    pub id: Uuid,
    
    /// Canvas name
    pub name: String,
    
    /// Canvas description
    pub description: Option<String>,
    
    /// Elements on the canvas
    pub elements: HashMap<Uuid, Element>,
    
    /// Connections between elements
    pub connections: Vec<Connection>,
    
    /// Canvas dimensions
    pub width: f64,
    pub height: f64,
    
    /// Canvas background
    pub background: Background,
    
    /// Canvas style
    pub style: CanvasStyle,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
    
    /// Canvas metadata
    pub metadata: HashMap<String, String>,
}

impl Canvas {
    /// Create a new canvas
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            elements: HashMap::new(),
            connections: Vec::new(),
            width: 1920.0,
            height: 1080.0,
            background: Background::Color(Color::WHITE),
            style: CanvasStyle::default(),
            created_at: now,
            modified_at: now,
            metadata: HashMap::new(),
        }
    }
    
    /// Add an element to the canvas
    pub fn add_element(&mut self, element: Element) -> FlowResult<()> {
        self.elements.insert(element.id, element);
        self.modified_at = Utc::now();
        Ok(())
    }
    
    /// Remove an element from the canvas
    pub fn remove_element(&mut self, element_id: Uuid) -> FlowResult<()> {
        self.elements.remove(&element_id);
        // Remove connections to/from this element
        self.connections.retain(|c| c.source != element_id && c.target != element_id);
        self.modified_at = Utc::now();
        Ok(())
    }
    
    /// Get an element by ID
    pub fn get_element(&self, element_id: Uuid) -> Option<&Element> {
        self.elements.get(&element_id)
    }
    
    /// Add a connection between elements
    pub fn add_connection(&mut self, connection: Connection) -> FlowResult<()> {
        // Validate that both elements exist
        if !self.elements.contains_key(&connection.source) {
            return Err(FlowError::ConnectionError(format!(
                "Source element {:?} not found", connection.source
            )));
        }
        if !self.elements.contains_key(&connection.target) {
            return Err(FlowError::ConnectionError(format!(
                "Target element {:?} not found", connection.target
            )));
        }
        
        self.connections.push(connection);
        self.modified_at = Utc::now();
        Ok(())
    }
    
    /// Resize the canvas
    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
        self.modified_at = Utc::now();
    }
    
    /// Set canvas background
    pub fn set_background(&mut self, background: Background) {
        self.background = background;
        self.modified_at = Utc::now();
    }
}

/// Element on the canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// Unique identifier
    pub id: Uuid,
    
    /// Element type
    pub element_type: ElementType,
    
    /// Element position
    pub x: f64,
    pub y: f64,
    
    /// Element dimensions
    pub width: f64,
    pub height: f64,
    
    /// Element text content
    pub text: Option<String>,
    
    /// Element style
    pub style: Style,
    
    /// Element data (for custom properties)
    pub data: HashMap<String, serde_json::Value>,
    
    /// Element layer (z-index)
    pub layer: i32,
    
    /// Whether element is locked
    pub locked: bool,
    
    /// Whether element is visible
    pub visible: bool,
}

impl Element {
    /// Create a new element
    pub fn new(element_type: ElementType, x: f64, y: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            element_type,
            x,
            y,
            width: 100.0,
            height: 50.0,
            text: None,
            style: Style::default(),
            data: HashMap::new(),
            layer: 0,
            locked: false,
            visible: true,
        }
    }
    
    /// Set element text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    
    /// Set element size
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    
    /// Set element style
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    
    /// Move element
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    
    /// Resize element
    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}

/// Types of elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementType {
    /// Rectangle
    Rectangle,
    
    /// Circle/Ellipse
    Circle,
    
    /// Diamond (decision)
    Diamond,
    
    /// Parallelogram (input/output)
    Parallelogram,
    
    /// Rounded rectangle
    RoundedRectangle,
    
    /// Hexagon
    Hexagon,
    
    /// Triangle
    Triangle,
    
    /// Star
    Star,
    
    /// Arrow
    Arrow,
    
    /// Text only
    Text,
    
    /// Image
    Image,
    
    /// Custom shape
    Custom(String),
}

/// Connection between elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// Unique identifier
    pub id: Uuid,
    
    /// Source element ID
    pub source: Uuid,
    
    /// Target element ID
    pub target: Uuid,
    
    /// Connection type
    pub connection_type: ConnectionType,
    
    /// Connection label
    pub label: Option<String>,
    
    /// Connection style
    pub style: Stroke,
    
    /// Control points for curved connections
    pub control_points: Vec<(f64, f64)>,
    
    /// Whether connection is bidirectional
    pub bidirectional: bool,
}

impl Connection {
    /// Create a new connection
    pub fn new(source: Uuid, target: Uuid, connection_type: ConnectionType) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            target,
            connection_type,
            label: None,
            style: Stroke::default(),
            control_points: Vec::new(),
            bidirectional: false,
        }
    }
    
    /// Set connection label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    
    /// Set connection style
    pub fn with_style(mut self, style: Stroke) -> Self {
        self.style = style;
        self
    }
}

/// Types of connections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    /// Straight line
    Straight,
    
    /// Orthogonal (right angles)
    Orthogonal,
    
    /// Curved (Bezier)
    Curved,
    
    /// Step (horizontal then vertical)
    Step,
    
    /// Freeform
    Freeform,
}

/// Element style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    /// Fill color
    pub fill: Color,
    
    /// Stroke (border)
    pub stroke: Stroke,
    
    /// Font settings
    pub font: Font,
    
    /// Shadow effect
    pub shadow: Option<Shadow>,
    
    /// Corner radius (for rounded rectangles)
    pub corner_radius: f64,
    
    /// Opacity (0.0 to 1.0)
    pub opacity: f64,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fill: Color::WHITE,
            stroke: Stroke::default(),
            font: Font::default(),
            shadow: None,
            corner_radius: 0.0,
            opacity: 1.0,
        }
    }
}

/// Color representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    
    /// Green component (0-255)
    pub g: u8,
    
    /// Blue component (0-255)
    pub b: u8,
    
    /// Alpha component (0-255)
    pub a: u8,
}

impl Color {
    /// Create a new color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create a solid color (alpha = 255)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }
    
    /// Common colors
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const GRAY: Color = Color { r: 128, g: 128, b: 128, a: 255 };
    pub const LIGHT_GRAY: Color = Color { r: 211, g: 211, b: 211, a: 255 };
    pub const DARK_GRAY: Color = Color { r: 64, g: 64, b: 64, a: 255 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0, a: 255 };
    pub const ORANGE: Color = Color { r: 255, g: 165, b: 0, a: 255 };
    pub const PURPLE: Color = Color { r: 128, g: 0, b: 128, a: 255 };
    pub const CYAN: Color = Color { r: 0, g: 255, b: 255, a: 255 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };
    
    /// Parse hex color string
    pub fn from_hex(hex: &str) -> FlowResult<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 && hex.len() != 8 {
            return Err(FlowError::ElementError(format!(
                "Invalid hex color: {}", hex
            )));
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| FlowError::ElementError("Invalid hex color".to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| FlowError::ElementError("Invalid hex color".to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| FlowError::ElementError("Invalid hex color".to_string()))?;
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16)
                .map_err(|_| FlowError::ElementError("Invalid hex color".to_string()))?
        } else {
            255
        };
        
        Ok(Self::new(r, g, b, a))
    }
    
    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

/// Stroke (line) style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    /// Stroke color
    pub color: Color,
    
    /// Stroke width
    pub width: f64,
    
    /// Line style
    pub line_style: LineStyle,
    
    /// Line cap style
    pub line_cap: LineCap,
    
    /// Line join style
    pub line_join: LineJoin,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            width: 1.0,
            line_style: LineStyle::Solid,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
        }
    }
}

/// Line styles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LineStyle {
    /// Solid line
    Solid,
    
    /// Dashed line
    Dashed,
    
    /// Dotted line
    Dotted,
    
    /// Dash-dot line
    DashDot,
    
    /// Dash-dot-dot line
    DashDotDot,
}

/// Line cap styles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LineCap {
    /// Butt cap
    Butt,
    
    /// Round cap
    Round,
    
    /// Square cap
    Square,
}

/// Line join styles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LineJoin {
    /// Miter join
    Miter,
    
    /// Round join
    Round,
    
    /// Bevel join
    Bevel,
}

/// Font settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    /// Font family
    pub family: String,
    
    /// Font size in points
    pub size: f64,
    
    /// Font weight
    pub weight: FontWeight,
    
    /// Font style
    pub style: FontStyle,
    
    /// Font color
    pub color: Color,
    
    /// Text alignment
    pub alignment: TextAlignment,
    
    /// Vertical alignment
    pub vertical_alignment: VerticalAlignment,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: "Arial".to_string(),
            size: 12.0,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            color: Color::BLACK,
            alignment: TextAlignment::Center,
            vertical_alignment: VerticalAlignment::Middle,
        }
    }
}

/// Font weights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

/// Font styles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Text alignment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Vertical alignment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

/// Shadow effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadow {
    /// Shadow color
    pub color: Color,
    
    /// Horizontal offset
    pub offset_x: f64,
    
    /// Vertical offset
    pub offset_y: f64,
    
    /// Blur radius
    pub blur: f64,
}

/// Canvas background
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Background {
    /// Solid color
    Color(Color),
    
    /// Gradient
    Gradient(Gradient),
    
    /// Image
    Image(String),
    
    /// Pattern
    Pattern(String),
}

/// Gradient definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gradient {
    /// Gradient type
    pub gradient_type: GradientType,
    
    /// Gradient stops
    pub stops: Vec<GradientStop>,
    
    /// Gradient angle (for linear gradients)
    pub angle: f64,
}

/// Gradient types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GradientType {
    /// Linear gradient
    Linear,
    
    /// Radial gradient
    Radial,
}

/// Gradient stop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientStop {
    /// Stop position (0.0 to 1.0)
    pub position: f64,
    
    /// Stop color
    pub color: Color,
}

/// Canvas style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasStyle {
    /// Grid enabled
    pub grid_enabled: bool,
    
    /// Grid size
    pub grid_size: f64,
    
    /// Grid color
    pub grid_color: Color,
    
    /// Snap to grid
    pub snap_to_grid: bool,
    
    /// Show rulers
    pub show_rulers: bool,
    
    /// Show guides
    pub show_guides: bool,
}

impl Default for CanvasStyle {
    fn default() -> Self {
        Self {
            grid_enabled: true,
            grid_size: 20.0,
            grid_color: Color::LIGHT_GRAY,
            snap_to_grid: true,
            show_rulers: true,
            show_guides: true,
        }
    }
}