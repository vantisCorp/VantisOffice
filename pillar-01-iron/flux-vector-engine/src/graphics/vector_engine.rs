//! Vector graphics engine

use serde::{Deserialize, Serialize};

/// Vector engine for rendering vector graphics
pub struct VectorEngine {
    paths: Vec<Path>,
    paints: Vec<Paint>,
}

impl VectorEngine {
    /// Create a new vector engine
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            paints: Vec::new(),
        }
    }
    
    /// Add a path
    pub fn add_path(&mut self, path: Path) {
        self.paths.push(path);
    }
    
    /// Add a paint
    pub fn add_paint(&mut self, paint: Paint) {
        self.paints.push(paint);
    }
    
    /// Clear all paths and paints
    pub fn clear(&mut self) {
        self.paths.clear();
        self.paints.clear();
    }
    
    /// Get all paths
    pub fn paths(&self) -> &[Path] {
        &self.paths
    }
    
    /// Get all paints
    pub fn paints(&self) -> &[Paint] {
        &self.paints
    }
}

/// Vector path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    commands: Vec<PathCommand>,
    closed: bool,
}

impl Path {
    /// Create a new path
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            closed: false,
        }
    }
    
    /// Move to position
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::MoveTo(x, y));
    }
    
    /// Line to position
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::LineTo(x, y));
    }
    
    /// Quadratic curve
    pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::QuadTo(cx, cy, x, y));
    }
    
    /// Cubic curve
    pub fn cubic_to(&mut self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::CubicTo(c1x, c1y, c2x, c2y, x, y));
    }
    
    /// Close the path
    pub fn close(&mut self) {
        self.closed = true;
    }
    
    /// Get path commands
    pub fn commands(&self) -> &[PathCommand] {
        &self.commands
    }
    
    /// Check if path is closed
    pub fn is_closed(&self) -> bool {
        self.closed
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

/// Path command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    QuadTo(f32, f32, f32, f32),
    CubicTo(f32, f32, f32, f32, f32, f32),
    Arc(f32, f32, f32, f32, f32, bool, bool),
    Close,
}

/// Paint for filling and stroking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paint {
    pub color: Color,
    pub fill_type: FillType,
    pub stroke_width: f32,
    pub stroke_color: Color,
    pub stroke_cap: StrokeCap,
    pub stroke_join: StrokeJoin,
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            color: Color::rgb(0x000000),
            fill_type: FillType::NonZero,
            stroke_width: 1.0,
            stroke_color: Color::rgb(0x000000),
            stroke_cap: StrokeCap::Butt,
            stroke_join: StrokeJoin::Miter,
        }
    }
}

/// Color
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create RGB color
    pub fn rgb(rgb: u32) -> Self {
        Self {
            r: ((rgb >> 16) & 0xFF) as u8,
            g: ((rgb >> 8) & 0xFF) as u8,
            b: (rgb & 0xFF) as u8,
            a: 255,
        }
    }
    
    /// Create RGBA color
    pub fn rgba(rgba: u32) -> Self {
        Self {
            r: ((rgba >> 24) & 0xFF) as u8,
            g: ((rgba >> 16) & 0xFF) as u8,
            b: ((rgba >> 8) & 0xFF) as u8,
            a: (rgba & 0xFF) as u8,
        }
    }
    
    /// Create color from components
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Fill type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FillType {
    NonZero,
    EvenOdd,
}

/// Stroke cap
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StrokeCap {
    Butt,
    Round,
    Square,
}

/// Stroke join
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StrokeJoin {
    Miter,
    Round,
    Bevel,
}