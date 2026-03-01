//! Vantis Canvas - 3D-accelerated presentation application
//! 
//! Main entry point for the Vantis Canvas application

use vantis_canvas::{Canvas, Shape, Text, Image, Renderer, AnimationManager, CanvasCollaboration, ExportFormat};
use vantis_canvas::core::{ShapeType, Fill, Stroke, StrokeStyle, Position, Size};
use std::path::PathBuf;

fn main() {
    println!("Vantis Canvas v{}", vantis_canvas::VERSION);
    println!("3D-accelerated presentation for VantisOffice\n");
    
    // Initialize subsystems
    if let Err(e) = vantis_canvas::init() {
        eprintln!("Initialization error: {}", e);
        std::process::exit(1);
    }
    
    println!("✓ Vantis Canvas initialized successfully\n");
    
    // Create a new canvas
    let mut canvas = Canvas::new("Demo Presentation".to_string());
    println!("✓ Created new canvas: Demo Presentation\n");
    
    // Add first slide
    let slide1 = canvas.add_slide();
    slide1.name = "Introduction".to_string();
    slide1.transition = Some(vantis_canvas::core::TransitionType::Fade);
    println!("✓ Added slide: Introduction");
    
    // Add layer to first slide
    let layer1 = slide1.add_layer();
    layer1.name = "Background".to_string();
    
    // Add background rectangle
    let bg_shape = Shape::new(ShapeType::Rectangle)
        .with_position(0.0, 0.0)
        .with_size(1920.0, 1080.0)
        .with_fill(Fill::Solid("#1a1a2e".to_string()));
    layer1.add_shape(bg_shape);
    
    // Add title text
    let title_text = Text::new("Welcome to Vantis Canvas".to_string())
        .with_position(960.0, 200.0)
        .with_font("Arial".to_string(), 72.0);
    slide1.add_text(title_text);
    
    // Add subtitle text
    let subtitle_text = Text::new("3D-accelerated presentations".to_string())
        .with_position(960.0, 300.0)
        .with_font("Arial".to_string(), 36.0);
    slide1.add_text(subtitle_text);
    
    println!("✓ Added content to slide 1\n");
    
    // Add second slide
    let slide2 = canvas.add_slide();
    slide2.name = "Features".to_string();
    slide2.transition = Some(vantis_canvas::core::TransitionType::SlideRight);
    println!("✓ Added slide: Features");
    
    // Add content to second slide
    let features = vec![
        ("Infinite Canvas", "Navigate freely in 3D space"),
        ("GPU Acceleration", "120Hz rendering with Vulkan"),
        ("Real-time Collaboration", "Work together seamlessly"),
        ("Advanced Animations", "Smooth transitions and effects"),
    ];
    
    for (i, (title, description)) in features.iter().enumerate() {
        let x = 200.0 + (i % 2) as f64 * 800.0;
        let y = 200.0 + (i / 2) as f64 * 400.0;
        
        // Add box
        let box_shape = Shape::new(ShapeType::RoundedRectangle { radius: 20.0 })
            .with_position(x, y)
            .with_size(700.0, 300.0)
            .with_fill(Fill::Solid("#16213e".to_string()))
            .with_stroke(Stroke {
                color: "#0f3460".to_string(),
                width: 3.0,
                style: StrokeStyle::Solid,
                dash_pattern: None,
            });
        slide2.add_shape(box_shape);
        
        // Add title
        let title_text = Text::new(title.to_string())
            .with_position(x + 350.0, y + 80.0)
            .with_font("Arial".to_string(), 32.0);
        slide2.add_text(title_text);
        
        // Add description
        let desc_text = Text::new(description.to_string())
            .with_position(x + 350.0, y + 150.0)
            .with_font("Arial".to_string(), 20.0);
        slide2.add_text(desc_text);
    }
    
    println!("✓ Added content to slide 2\n");
    
    // Add third slide
    let slide3 = canvas.add_slide();
    slide3.name = "Thank You".to_string();
    slide3.transition = Some(vantis_canvas::core::TransitionType::ZoomIn);
    println!("✓ Added slide: Thank You");
    
    // Add thank you text
    let thank_you_text = Text::new("Thank You!".to_string())
        .with_position(960.0, 540.0)
        .with_font("Arial".to_string(), 96.0);
    slide3.add_text(thank_you_text);
    
    println!("✓ Added content to slide 3\n");
    
    // Test Renderer
    println!("Testing Renderer:");
    let renderer = Renderer::new();
    println!("  ✓ Renderer created");
    println!("  ✓ GPU acceleration: {}", renderer.is_enabled());
    println!();
    
    // Test Animation Manager
    println!("Testing Animation Manager:");
    let mut anim_manager = AnimationManager::new();
    println!("  ✓ Animation manager created");
    
    let fade_in = vantis_canvas::animation::Animation::new(
        "fade_in".to_string(),
        vantis_canvas::animation::AnimationType::FadeIn,
        std::time::Duration::from_millis(1000)
    );
    anim_manager.add_animation(fade_in).unwrap();
    println!("  ✓ Fade-in animation added");
    println!();
    
    // Test Collaboration
    println!("Testing Collaboration Manager:");
    let collab_manager = CanvasCollaboration::new();
    
    match collab_manager.create_session("demo_canvas".to_string()) {
        Ok(session_id) => {
            println!("  ✓ Created collaboration session: {}", session_id);
            
            match collab_manager.join_session(session_id.clone(), "user1".to_string(), "Alice".to_string()) {
                Ok(_) => println!("  ✓ User 'Alice' joined the session"),
                Err(e) => println!("  ✗ Error joining session: {}", e),
            }
            
            match collab_manager.get_active_users(session_id.clone()) {
                Ok(users) => println!("  ✓ Active users: {}", users.len()),
                Err(e) => println!("  ✗ Error getting active users: {}", e),
            }
        }
        Err(e) => println!("  ✗ Error creating session: {}", e),
    }
    println!();
    
    // Test Export
    println!("Testing Export:");
    let svg_path = PathBuf::from("/tmp/demo_canvas.svg");
    let exporter = vantis_canvas::export::CanvasExporter::new(ExportFormat::Svg);
    
    match exporter.export(&canvas, &svg_path) {
        Ok(_) => println!("  ✓ Exported to SVG: {}", svg_path.display()),
        Err(e) => println!("  ✗ Export error: {}", e),
    }
    
    let json_path = PathBuf::from("/tmp/demo_canvas.json");
    let json_exporter = vantis_canvas::export::CanvasExporter::new(ExportFormat::Vantis);
    
    match json_exporter.export(&canvas, &json_path) {
        Ok(_) => println!("  ✓ Exported to JSON: {}", json_path.display()),
        Err(e) => println!("  ✗ Export error: {}", e),
    }
    println!();
    
    println!("─────────────────────────────────");
    println!("Vantis Canvas demo completed successfully!");
    println!("─────────────────────────────────");
    println!();
    println!("Canvas Statistics:");
    println!("  Total slides: {}", canvas.slides.len());
    println!("  Dimensions: {}x{}", canvas.dimensions.width, canvas.dimensions.height);
    println!("  Infinite canvas: {}", canvas.dimensions.infinite);
}