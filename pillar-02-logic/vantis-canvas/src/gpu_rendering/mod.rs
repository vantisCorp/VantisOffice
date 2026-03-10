//! GPU Rendering Module - Integration with Flux Vector Engine (Pillar 01)
//!
//! Provides hardware-accelerated rendering for canvas presentations using
//! the Flux Vector Engine's Vulkan-based rendering pipeline.
//!
//! # Architecture
//!
//! ```text
//! vantis-canvas (Pillar 02: Logic)
//!     └── gpu_rendering module
//!             └── flux-vector-engine (Pillar 01: Iron)
//!                     ├── Vulkan Backend
//!                     ├── Vector Graphics Engine
//!                     └── UI Component System
//! ```

use flux_vector_engine::{
    Color as FluxColor, EngineCapabilities, FillType, Paint, Path as FluxPath,
    WindowConfig, VSyncMode,
};
use flux_vector_engine::graphics::{StrokeCap, StrokeJoin};
use serde::{Deserialize, Serialize};

use crate::core::{Canvas, Fill, Image, Layer, Shape, ShapeType, Slide, Stroke, Text};

/// GPU rendering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuRenderConfig {
    /// Target width in pixels
    pub width: u32,
    /// Target height in pixels
    pub height: u32,
    /// VSync mode
    pub vsync: VSyncSetting,
    /// Anti-aliasing samples (1, 2, 4, 8, 16)
    pub msaa_samples: u32,
    /// Enable hardware acceleration
    pub hardware_accelerated: bool,
    /// Target frame rate
    pub target_fps: u32,
    /// Enable GPU-powered animations
    pub gpu_animations: bool,
}

impl Default for GpuRenderConfig {
    fn default() -> Self {
        GpuRenderConfig {
            width: 1920,
            height: 1080,
            vsync: VSyncSetting::Adaptive,
            msaa_samples: 4,
            hardware_accelerated: true,
            target_fps: 60,
            gpu_animations: true,
        }
    }
}

/// VSync setting for presentation rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VSyncSetting {
    Off,
    On,
    Adaptive,
    HighRefresh,
}

impl VSyncSetting {
    pub fn to_flux_vsync(&self) -> VSyncMode {
        match self {
            VSyncSetting::Off => VSyncMode::Off,
            VSyncSetting::On => VSyncMode::On,
            VSyncSetting::Adaptive | VSyncSetting::HighRefresh => VSyncMode::Adaptive,
        }
    }
}

/// Render quality preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderQuality {
    Preview,
    Standard,
    Presentation,
    Export,
}

impl RenderQuality {
    pub fn msaa_samples(&self) -> u32 {
        match self {
            RenderQuality::Preview => 1,
            RenderQuality::Standard => 4,
            RenderQuality::Presentation => 8,
            RenderQuality::Export => 16,
        }
    }

    pub fn target_fps(&self) -> u32 {
        match self {
            RenderQuality::Preview => 30,
            RenderQuality::Standard => 60,
            RenderQuality::Presentation => 120,
            RenderQuality::Export => 1,
        }
    }
}

/// GPU-accelerated canvas renderer
pub struct GpuCanvasRenderer {
    config: GpuRenderConfig,
    capabilities: EngineCapabilities,
    quality: RenderQuality,
    frame_count: u64,
    active: bool,
}

impl GpuCanvasRenderer {
    pub fn new() -> Self {
        let capabilities = flux_vector_engine::capabilities();
        GpuCanvasRenderer {
            config: GpuRenderConfig::default(),
            capabilities,
            quality: RenderQuality::Standard,
            frame_count: 0,
            active: false,
        }
    }

    pub fn with_config(config: GpuRenderConfig) -> Self {
        let capabilities = flux_vector_engine::capabilities();
        GpuCanvasRenderer {
            config,
            capabilities,
            quality: RenderQuality::Standard,
            frame_count: 0,
            active: false,
        }
    }

    pub fn with_quality(quality: RenderQuality) -> Self {
        let capabilities = flux_vector_engine::capabilities();
        let config = GpuRenderConfig {
            msaa_samples: quality.msaa_samples().min(capabilities.max_msaa_samples),
            target_fps: quality.target_fps(),
            ..GpuRenderConfig::default()
        };
        GpuCanvasRenderer {
            config,
            capabilities,
            quality,
            frame_count: 0,
            active: false,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        if self.active {
            return Ok(());
        }
        if self.config.msaa_samples > self.capabilities.max_msaa_samples {
            self.config.msaa_samples = self.capabilities.max_msaa_samples;
        }
        self.active = true;
        self.frame_count = 0;
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn capabilities(&self) -> &EngineCapabilities {
        &self.capabilities
    }

    pub fn config(&self) -> &GpuRenderConfig {
        &self.config
    }

    pub fn quality(&self) -> RenderQuality {
        self.quality
    }

    pub fn set_quality(&mut self, quality: RenderQuality) {
        self.quality = quality;
        self.config.msaa_samples = quality.msaa_samples().min(self.capabilities.max_msaa_samples);
        self.config.target_fps = quality.target_fps();
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Render a complete canvas (all slides)
    pub fn render_canvas(&mut self, canvas: &Canvas) -> Result<RenderOutput, String> {
        if !self.active {
            return Err("Renderer not initialized. Call initialize() first.".to_string());
        }

        let mut slide_outputs = Vec::new();
        for (index, slide) in canvas.slides.iter().enumerate() {
            let slide_output = self.render_slide(slide, index)?;
            slide_outputs.push(slide_output);
        }

        self.frame_count += 1;

        Ok(RenderOutput {
            slides: slide_outputs,
            frame_number: self.frame_count,
            render_time_ms: 0.0,
            quality: self.quality,
        })
    }

    /// Render a single slide
    pub fn render_slide(&self, slide: &Slide, index: usize) -> Result<SlideRenderOutput, String> {
        if !self.active {
            return Err("Renderer not initialized".to_string());
        }

        let mut layer_outputs = Vec::new();
        for layer in &slide.layers {
            if !layer.visible {
                continue;
            }
            let layer_output = self.render_layer(layer)?;
            layer_outputs.push(layer_output);
        }

        Ok(SlideRenderOutput {
            slide_index: index,
            layers: layer_outputs,
            width: self.config.width,
            height: self.config.height,
        })
    }

    fn render_layer(&self, layer: &Layer) -> Result<LayerRenderOutput, String> {
        let mut shape_count = 0;
        let mut text_count = 0;
        let mut image_count = 0;
        let mut draw_calls = Vec::new();

        for shape in &layer.shapes {
            let path = self.shape_to_flux_path(shape);
            let paint = self.shape_to_flux_paint(shape);
            draw_calls.push(DrawCall::Shape {
                path,
                paint,
                shape_type: format!("{:?}", shape.shape_type),
            });
            shape_count += 1;
        }

        for text in &layer.texts {
            let color = fill_to_hex(&text.fill);
            draw_calls.push(DrawCall::Text {
                content: text.content.clone(),
                x: text.position.x,
                y: text.position.y,
                font_size: text.font.size,
                color,
            });
            text_count += 1;
        }

        for image in &layer.images {
            draw_calls.push(DrawCall::Image {
                source: image.path.clone(),
                x: image.position.x,
                y: image.position.y,
                width: image.size.width,
                height: image.size.height,
            });
            image_count += 1;
        }

        Ok(LayerRenderOutput {
            layer_name: layer.name.clone(),
            opacity: layer.opacity,
            shape_count,
            text_count,
            image_count,
            draw_calls,
        })
    }

    /// Convert a canvas shape to a Flux Vector Engine path
    pub fn shape_to_flux_path(&self, shape: &Shape) -> FluxPath {
        let x = shape.position.x as f32;
        let y = shape.position.y as f32;
        let w = shape.size.width as f32;
        let h = shape.size.height as f32;

        let mut path = FluxPath::new();

        match &shape.shape_type {
            ShapeType::Rectangle => {
                path.move_to(x, y);
                path.line_to(x + w, y);
                path.line_to(x + w, y + h);
                path.line_to(x, y + h);
                path.close();
            }
            ShapeType::RoundedRectangle { .. } => {
                // Simplified: render as rectangle (rounded corners would use cubic_to)
                path.move_to(x, y);
                path.line_to(x + w, y);
                path.line_to(x + w, y + h);
                path.line_to(x, y + h);
                path.close();
            }
            ShapeType::Circle | ShapeType::Ellipse => {
                let cx = x + w / 2.0;
                let cy = y + h / 2.0;
                let rx = w / 2.0;
                let ry = h / 2.0;
                let k: f32 = 0.5522848;
                let kx = rx * k;
                let ky = ry * k;

                path.move_to(cx + rx, cy);
                path.cubic_to(cx + rx, cy + ky, cx + kx, cy + ry, cx, cy + ry);
                path.cubic_to(cx - kx, cy + ry, cx - rx, cy + ky, cx - rx, cy);
                path.cubic_to(cx - rx, cy - ky, cx - kx, cy - ry, cx, cy - ry);
                path.cubic_to(cx + kx, cy - ry, cx + rx, cy - ky, cx + rx, cy);
                path.close();
            }
            ShapeType::Triangle => {
                path.move_to(x + w / 2.0, y);
                path.line_to(x + w, y + h);
                path.line_to(x, y + h);
                path.close();
            }
            ShapeType::Polygon { sides } => {
                let cx = x + w / 2.0;
                let cy = y + h / 2.0;
                let r = w.min(h) / 2.0;
                let n = *sides;
                for i in 0..n {
                    let angle = 2.0 * std::f32::consts::PI * i as f32 / n as f32
                        - std::f32::consts::PI / 2.0;
                    let px = cx + r * angle.cos();
                    let py = cy + r * angle.sin();
                    if i == 0 {
                        path.move_to(px, py);
                    } else {
                        path.line_to(px, py);
                    }
                }
                path.close();
            }
            ShapeType::Star { points, inner_radius } => {
                let cx = x + w / 2.0;
                let cy = y + h / 2.0;
                let outer_r = w.min(h) / 2.0;
                let inner_r = outer_r * (*inner_radius as f32);
                let n = *points;
                for i in 0..(n * 2) {
                    let angle = std::f32::consts::PI / n as f32 * i as f32
                        - std::f32::consts::PI / 2.0;
                    let r = if i % 2 == 0 { outer_r } else { inner_r };
                    let px = cx + r * angle.cos();
                    let py = cy + r * angle.sin();
                    if i == 0 {
                        path.move_to(px, py);
                    } else {
                        path.line_to(px, py);
                    }
                }
                path.close();
            }
            ShapeType::Line => {
                path.move_to(x, y);
                path.line_to(x + w, y + h);
            }
            ShapeType::Arrow => {
                let shaft_y = y + h / 2.0;
                let head_x = x + w * 0.7;
                path.move_to(x, shaft_y);
                path.line_to(head_x, shaft_y);
                path.line_to(head_x, y);
                path.line_to(x + w, shaft_y);
                path.line_to(head_x, y + h);
                path.line_to(head_x, shaft_y);
                path.close();
            }
            ShapeType::Path { points } | ShapeType::Freehand { points } => {
                for (i, (px, py)) in points.iter().enumerate() {
                    if i == 0 {
                        path.move_to(*px as f32, *py as f32);
                    } else {
                        path.line_to(*px as f32, *py as f32);
                    }
                }
            }
        }

        path
    }

    /// Convert shape properties to Flux paint
    pub fn shape_to_flux_paint(&self, shape: &Shape) -> Paint {
        let color = match &shape.fill {
            Some(fill) => fill_color_to_flux(fill),
            None => FluxColor::new(0, 0, 0, 0),
        };
        let (stroke_color, stroke_width) = match &shape.stroke {
            Some(stroke) => (parse_hex_color(&stroke.color), stroke.width as f32),
            None => (FluxColor::new(0, 0, 0, 255), 0.0),
        };

        Paint {
            color,
            fill_type: FillType::NonZero,
            stroke_width,
            stroke_color,
            stroke_cap: StrokeCap::Round,
            stroke_join: StrokeJoin::Round,
        }
    }

    /// Create a window configuration for presentation mode
    pub fn create_presentation_window_config(&self) -> WindowConfig {
        WindowConfig {
            title: "VantisCanvas Presentation".to_string(),
            width: self.config.width,
            height: self.config.height,
            refresh_rate: self.config.target_fps,
            vsync: self.config.vsync.to_flux_vsync(),
            resizable: false,
            decorations: false,
            transparent: false,
            always_on_top: true,
        }
    }

    /// Get render statistics
    pub fn render_stats(&self) -> RenderStats {
        RenderStats {
            total_frames: self.frame_count,
            target_fps: self.config.target_fps,
            msaa_samples: self.config.msaa_samples,
            resolution: (self.config.width, self.config.height),
            hardware_accelerated: self.config.hardware_accelerated,
            gpu_animations: self.config.gpu_animations,
            max_texture_size: self.capabilities.max_texture_size,
            supports_compute_shaders: self.capabilities.supports_compute_shaders,
        }
    }
}

/// Extract hex color from a Fill enum
fn fill_to_hex(fill: &Option<Fill>) -> String {
    match fill {
        Some(Fill::Solid(color)) => color.clone(),
        _ => "#000000".to_string(),
    }
}

/// Convert a Fill to FluxColor
fn fill_color_to_flux(fill: &Fill) -> FluxColor {
    match fill {
        Fill::Solid(hex) => parse_hex_color(hex),
        Fill::Gradient { start, .. } => parse_hex_color(start),
        Fill::Pattern { color, .. } => parse_hex_color(color),
        _ => FluxColor::new(0, 0, 0, 255),
    }
}

/// Parse a hex color string to FluxColor
fn parse_hex_color(hex: &str) -> FluxColor {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        let a = if hex.len() >= 8 {
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
        } else {
            255
        };
        FluxColor::new(r, g, b, a)
    } else {
        FluxColor::new(0, 0, 0, 255)
    }
}

/// Output from rendering a complete canvas
#[derive(Debug, Clone)]
pub struct RenderOutput {
    pub slides: Vec<SlideRenderOutput>,
    pub frame_number: u64,
    pub render_time_ms: f64,
    pub quality: RenderQuality,
}

#[derive(Debug, Clone)]
pub struct SlideRenderOutput {
    pub slide_index: usize,
    pub layers: Vec<LayerRenderOutput>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct LayerRenderOutput {
    pub layer_name: String,
    pub opacity: f64,
    pub shape_count: usize,
    pub text_count: usize,
    pub image_count: usize,
    pub draw_calls: Vec<DrawCall>,
}

#[derive(Debug, Clone)]
pub enum DrawCall {
    Shape {
        path: FluxPath,
        paint: Paint,
        shape_type: String,
    },
    Text {
        content: String,
        x: f64,
        y: f64,
        font_size: f64,
        color: String,
    },
    Image {
        source: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderStats {
    pub total_frames: u64,
    pub target_fps: u32,
    pub msaa_samples: u32,
    pub resolution: (u32, u32),
    pub hardware_accelerated: bool,
    pub gpu_animations: bool,
    pub max_texture_size: u32,
    pub supports_compute_shaders: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Canvas, Layer, Shape, ShapeType, Slide};

    fn create_test_canvas() -> Canvas {
        let mut canvas = Canvas::new("Test Presentation".to_string());
        let mut slide = Slide::new();
        let mut layer = Layer::new("Main".to_string());

        layer.shapes.push(Shape::new(ShapeType::Rectangle));
        layer.shapes.push(Shape::new(ShapeType::Circle));
        layer.shapes.push(Shape::new(ShapeType::Triangle));

        slide.layers.push(layer);
        canvas.slides.push(slide);
        canvas
    }

    #[test]
    fn test_gpu_renderer_creation() {
        let renderer = GpuCanvasRenderer::new();
        assert!(!renderer.is_active());
        assert_eq!(renderer.quality(), RenderQuality::Standard);
        assert_eq!(renderer.frame_count(), 0);
    }

    #[test]
    fn test_gpu_renderer_with_config() {
        let config = GpuRenderConfig {
            width: 3840,
            height: 2160,
            msaa_samples: 8,
            target_fps: 120,
            ..GpuRenderConfig::default()
        };
        let renderer = GpuCanvasRenderer::with_config(config);
        assert_eq!(renderer.config().width, 3840);
        assert_eq!(renderer.config().height, 2160);
    }

    #[test]
    fn test_gpu_renderer_with_quality() {
        let renderer = GpuCanvasRenderer::with_quality(RenderQuality::Presentation);
        assert_eq!(renderer.quality(), RenderQuality::Presentation);
        assert_eq!(renderer.config().target_fps, 120);
    }

    #[test]
    fn test_gpu_renderer_initialize_shutdown() {
        let mut renderer = GpuCanvasRenderer::new();
        assert!(!renderer.is_active());
        renderer.initialize().unwrap();
        assert!(renderer.is_active());
        renderer.shutdown();
        assert!(!renderer.is_active());
    }

    #[test]
    fn test_render_canvas() {
        let mut renderer = GpuCanvasRenderer::new();
        renderer.initialize().unwrap();

        let canvas = create_test_canvas();
        let output = renderer.render_canvas(&canvas).unwrap();

        assert_eq!(output.slides.len(), 1);
        assert_eq!(output.frame_number, 1);
        assert_eq!(output.quality, RenderQuality::Standard);

        let slide = &output.slides[0];
        assert_eq!(slide.layers.len(), 1);

        let layer = &slide.layers[0];
        assert_eq!(layer.shape_count, 3);
        assert_eq!(layer.draw_calls.len(), 3);
    }

    #[test]
    fn test_render_without_initialization() {
        let mut renderer = GpuCanvasRenderer::new();
        let canvas = create_test_canvas();
        assert!(renderer.render_canvas(&canvas).is_err());
    }

    #[test]
    fn test_render_empty_canvas() {
        let mut renderer = GpuCanvasRenderer::new();
        renderer.initialize().unwrap();
        let canvas = Canvas::new("Empty".to_string());
        let output = renderer.render_canvas(&canvas).unwrap();
        assert_eq!(output.slides.len(), 0);
    }

    #[test]
    fn test_render_hidden_layer() {
        let mut renderer = GpuCanvasRenderer::new();
        renderer.initialize().unwrap();

        let mut canvas = Canvas::new("Test".to_string());
        let mut slide = Slide::new();

        let mut visible = Layer::new("Visible".to_string());
        visible.visible = true;
        visible.shapes.push(Shape::new(ShapeType::Rectangle));

        let mut hidden = Layer::new("Hidden".to_string());
        hidden.visible = false;
        hidden.shapes.push(Shape::new(ShapeType::Circle));

        slide.layers.push(visible);
        slide.layers.push(hidden);
        canvas.slides.push(slide);

        let output = renderer.render_canvas(&canvas).unwrap();
        assert_eq!(output.slides[0].layers.len(), 1);
        assert_eq!(output.slides[0].layers[0].layer_name, "Visible");
    }

    #[test]
    fn test_frame_counter() {
        let mut renderer = GpuCanvasRenderer::new();
        renderer.initialize().unwrap();
        let canvas = create_test_canvas();

        renderer.render_canvas(&canvas).unwrap();
        assert_eq!(renderer.frame_count(), 1);
        renderer.render_canvas(&canvas).unwrap();
        assert_eq!(renderer.frame_count(), 2);
    }

    #[test]
    fn test_quality_presets() {
        assert_eq!(RenderQuality::Preview.msaa_samples(), 1);
        assert_eq!(RenderQuality::Standard.msaa_samples(), 4);
        assert_eq!(RenderQuality::Presentation.msaa_samples(), 8);
        assert_eq!(RenderQuality::Export.msaa_samples(), 16);
    }

    #[test]
    fn test_set_quality() {
        let mut renderer = GpuCanvasRenderer::new();
        renderer.set_quality(RenderQuality::Presentation);
        assert_eq!(renderer.quality(), RenderQuality::Presentation);
    }

    #[test]
    fn test_capabilities() {
        let renderer = GpuCanvasRenderer::new();
        assert!(renderer.capabilities().max_texture_size > 0);
    }

    #[test]
    fn test_render_stats() {
        let mut renderer = GpuCanvasRenderer::new();
        renderer.initialize().unwrap();
        let canvas = create_test_canvas();
        renderer.render_canvas(&canvas).unwrap();
        let stats = renderer.render_stats();
        assert_eq!(stats.total_frames, 1);
        assert!(stats.hardware_accelerated);
    }

    #[test]
    fn test_parse_hex_color() {
        let c = parse_hex_color("#FF5733");
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 87);
        assert_eq!(c.b, 51);
        assert_eq!(c.a, 255);

        let c = parse_hex_color("invalid");
        assert_eq!(c.r, 0);
    }

    #[test]
    fn test_vsync_settings() {
        assert!(matches!(VSyncSetting::Off.to_flux_vsync(), VSyncMode::Off));
        assert!(matches!(VSyncSetting::On.to_flux_vsync(), VSyncMode::On));
        assert!(matches!(VSyncSetting::Adaptive.to_flux_vsync(), VSyncMode::Adaptive));
    }

    #[test]
    fn test_presentation_window_config() {
        let renderer = GpuCanvasRenderer::new();
        let config = renderer.create_presentation_window_config();
        assert_eq!(config.title, "VantisCanvas Presentation");
        assert_eq!(config.width, 1920);
        assert!(!config.resizable);
        assert!(config.always_on_top);
    }

    #[test]
    fn test_shape_to_flux_path_all_types() {
        let renderer = GpuCanvasRenderer::new();
        let types: Vec<ShapeType> = vec![
            ShapeType::Rectangle,
            ShapeType::RoundedRectangle { radius: 10.0 },
            ShapeType::Circle,
            ShapeType::Ellipse,
            ShapeType::Triangle,
            ShapeType::Polygon { sides: 6 },
            ShapeType::Star { points: 5, inner_radius: 0.4 },
            ShapeType::Line,
            ShapeType::Arrow,
            ShapeType::Path { points: vec![(0.0, 0.0), (50.0, 50.0), (100.0, 0.0)] },
            ShapeType::Freehand { points: vec![(0.0, 0.0), (10.0, 10.0)] },
        ];
        for st in types {
            let shape = Shape::new(st).with_position(10.0, 20.0).with_size(100.0, 80.0);
            let path = renderer.shape_to_flux_path(&shape);
            assert!(!path.commands().is_empty());
        }
    }

    #[test]
    fn test_render_stats_serialization() {
        let stats = RenderStats {
            total_frames: 100, target_fps: 60, msaa_samples: 4,
            resolution: (1920, 1080), hardware_accelerated: true,
            gpu_animations: true, max_texture_size: 16384,
            supports_compute_shaders: true,
        };
        let json = serde_json::to_string(&stats).unwrap();
        let d: RenderStats = serde_json::from_str(&json).unwrap();
        assert_eq!(d.total_frames, 100);
    }
}