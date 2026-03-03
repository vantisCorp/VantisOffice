//! Core data structures for Vantis Canvas

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

/// Canvas - infinite canvas for presentations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
    pub name: String,
    pub slides: Vec<Slide>,
    pub active_slide: usize,
    pub dimensions: CanvasDimensions,
    pub background: Background,
    pub metadata: CanvasMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasDimensions {
    pub width: f64,
    pub height: f64,
    pub infinite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Background {
    Solid(String),
    Gradient { start: String, end: String, direction: GradientDirection },
    Image(String),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GradientDirection {
    Horizontal,
    Vertical,
    Diagonal,
    Radial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasMetadata {
    pub author: Option<String>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl Canvas {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Canvas {
            name,
            slides: Vec::new(),
            active_slide: 0,
            dimensions: CanvasDimensions {
                width: 1920.0,
                height: 1080.0,
                infinite: true,
            },
            background: Background::Solid("#FFFFFF".to_string()),
            metadata: CanvasMetadata {
                author: None,
                created: now,
                modified: now,
                description: None,
                tags: Vec::new(),
            },
        }
    }
    
    pub fn add_slide(&mut self) -> &mut Slide {
        let slide = Slide::new(self.slides.len());
        self.slides.push(slide);
        self.slides.last_mut().unwrap()
    }
    
    pub fn get_active_slide(&self) -> Option<&Slide> {
        self.slides.get(self.active_slide)
    }
    
    pub fn get_active_slide_mut(&mut self) -> Option<&mut Slide> {
        self.slides.get_mut(self.active_slide)
    }
    
    pub fn set_active_slide(&mut self, index: usize) -> Result<(), String> {
        if index < self.slides.len() {
            self.active_slide = index;
            Ok(())
        } else {
            Err(format!("Slide index {} out of range", index))
        }
    }
}

/// Slide - a single slide in the canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    pub index: usize,
    pub name: String,
    pub layers: Vec<Layer>,
    pub duration: Option<f64>, // in seconds
    pub transition: Option<TransitionType>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    None,
    Fade,
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
    ZoomIn,
    ZoomOut,
    Rotate,
    Dissolve,
}

impl Slide {
    pub fn new(index: usize) -> Self {
        Slide {
            index,
            name: format!("Slide {}", index + 1),
            layers: Vec::new(),
            duration: None,
            transition: None,
            notes: String::new(),
        }
    }
    
    pub fn add_layer(&mut self) -> &mut Layer {
        let layer = Layer::new(self.layers.len());
        self.layers.push(layer);
        self.layers.last_mut().unwrap()
    }
    
    pub fn add_shape(&mut self, shape: Shape) {
        if let Some(layer) = self.layers.last_mut() {
            layer.add_shape(shape);
        } else {
            let mut layer = Layer::new(0);
            layer.add_shape(shape);
            self.layers.push(layer);
        }
    }
    
    pub fn add_text(&mut self, text: Text) {
        if let Some(layer) = self.layers.last_mut() {
            layer.add_text(text);
        } else {
            let mut layer = Layer::new(0);
            layer.add_text(text);
            self.layers.push(layer);
        }
    }
    
    pub fn add_image(&mut self, image: Image) {
        if let Some(layer) = self.layers.last_mut() {
            layer.add_image(image);
        } else {
            let mut layer = Layer::new(0);
            layer.add_image(image);
            self.layers.push(layer);
        }
    }
}

/// Layer - a layer containing shapes, text, and images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub index: usize,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    pub blend_mode: BlendMode,
    pub shapes: Vec<Shape>,
    pub texts: Vec<Text>,
    pub images: Vec<Image>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
}

impl Layer {
    pub fn new(index: usize) -> Self {
        Layer {
            index,
            name: format!("Layer {}", index + 1),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            shapes: Vec::new(),
            texts: Vec::new(),
            images: Vec::new(),
        }
    }
    
    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
    
    pub fn add_text(&mut self, text: Text) {
        self.texts.push(text);
    }
    
    pub fn add_image(&mut self, image: Image) {
        self.images.push(image);
    }
}

/// Shape - geometric shapes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub id: String,
    pub shape_type: ShapeType,
    pub position: Position,
    pub size: Size,
    pub rotation: f64,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(PartialEq)]
pub enum ShapeType {
    Rectangle,
    RoundedRectangle { radius: f64 },
    Circle,
    Ellipse,
    Triangle,
    Polygon { sides: usize },
    Star { points: usize, inner_radius: f64 },
    Line,
    Arrow,
    Path { points: Vec<(f64, f64)> },
    Freehand { points: Vec<(f64, f64)> },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fill {
    Solid(String),
    Gradient { start: String, end: String, direction: GradientDirection },
    Pattern { pattern: String, color: String },
    Image(String),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub color: String,
    pub width: f64,
    pub style: StrokeStyle,
    pub dash_pattern: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrokeStyle {
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Shadow { offset_x: f64, offset_y: f64, blur: f64, color: String },
    Glow { blur: f64, color: String },
    Blur { radius: f64 },
    Brightness { amount: f64 },
    Contrast { amount: f64 },
    Saturation { amount: f64 },
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Self {
        Shape {
            id: uuid::Uuid::new_v4().to_string(),
            shape_type,
            position: Position { x: 0.0, y: 0.0 },
            size: Size { width: 100.0, height: 100.0 },
            rotation: 0.0,
            fill: None,
            stroke: None,
            effects: Vec::new(),
        }
    }
    
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = Position { x, y };
        self
    }
    
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = Size { width, height };
        self
    }
    
    pub fn with_fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }
    
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

/// Text - text elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub id: String,
    pub content: String,
    pub position: Position,
    pub font: Font,
    pub alignment: TextAlignment,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    pub family: String,
    pub size: f64,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub line_height: f64,
    pub letter_spacing: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

impl Text {
    pub fn new(content: String) -> Self {
        Text {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            position: Position { x: 0.0, y: 0.0 },
            font: Font {
                family: "Arial".to_string(),
                size: 24.0,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
                line_height: 1.2,
                letter_spacing: 0.0,
            },
            alignment: TextAlignment::Left,
            fill: Some(Fill::Solid("#000000".to_string())),
            stroke: None,
            effects: Vec::new(),
        }
    }
    
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = Position { x, y };
        self
    }
    
    pub fn with_font(mut self, family: String, size: f64) -> Self {
        self.font.family = family;
        self.font.size = size;
        self
    }
}

/// Image - image elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub path: String,
    pub position: Position,
    pub size: Size,
    pub rotation: f64,
    pub opacity: f64,
    pub effects: Vec<Effect>,
}

impl Image {
    pub fn new(path: String) -> Self {
        Image {
            id: uuid::Uuid::new_v4().to_string(),
            path,
            position: Position { x: 0.0, y: 0.0 },
            size: Size { width: 100.0, height: 100.0 },
            rotation: 0.0,
            opacity: 1.0,
            effects: Vec::new(),
        }
    }
    
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = Position { x, y };
        self
    }
    
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = Size { width, height };
        self
    }
}

/// Initialize core subsystem
pub fn init() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new("Test Canvas".to_string());
        assert_eq!(canvas.name, "Test Canvas");
        assert_eq!(canvas.slides.len(), 0);
    }
    
    #[test]
    fn test_slide_creation() {
        let mut canvas = Canvas::new("Test".to_string());
        canvas.add_slide();
        assert_eq!(canvas.slides.len(), 1);
    }
    
    #[test]
    fn test_shape_creation() {
        let shape = Shape::new(ShapeType::Rectangle);
        assert_eq!(shape.shape_type, ShapeType::Rectangle);
    }
    
    #[test]
    fn test_text_creation() {
        let text = Text::new("Hello World".to_string());
        assert_eq!(text.content, "Hello World");
    }
}