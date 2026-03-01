//! Flux Vector Engine demo application

use flux_vector_engine::{
    init, capabilities,
    graphics::{VectorEngine, Path, Paint, Color, FillType},
    ui::{Button, TextField, ListView, Component, ComponentContext, Event, EventType, Animation, EasingFunction},
    WindowConfig, VSyncMode,
};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Flux Vector Engine Demo");
    println!("========================\n");
    
    // Initialize the engine
    init()?;
    
    // Display engine capabilities
    let caps = capabilities();
    println!("Engine Capabilities:");
    println!("  Version: {}", caps.version);
    println!("  Max Texture Size: {}", caps.max_texture_size);
    println!("  Max Framebuffer Size: {}", caps.max_framebuffer_size);
    println!("  Supports Ray Tracing: {}", caps.supports_ray_tracing);
    println!("  Supports Compute Shaders: {}", caps.supports_compute_shaders);
    println!("  Max MSAA Samples: {}", caps.max_msaa_samples);
    println!("  Max Anisotropy: {}", caps.max_anisotropy);
    println!();
    
    // Create window configuration
    let config = WindowConfig {
        width: 1920,
        height: 1080,
        title: "Flux Vector Engine Demo".to_string(),
        refresh_rate: 120,
        vsync: VSyncMode::Adaptive,
        resizable: true,
        decorations: true,
        transparent: false,
        always_on_top: false,
    };
    
    println!("Window Configuration:");
    println!("  Resolution: {}x{}", config.width, config.height);
    println!("  Title: {}", config.title);
    println!("  Refresh Rate: {} Hz", config.refresh_rate);
    println!("  VSync: {:?}", config.vsync);
    println!();
    
    // Create vector engine
    let mut vector_engine = VectorEngine::new();
    
    // Create a path
    let mut path = Path::new();
    path.move_to(100.0, 100.0);
    path.line_to(200.0, 100.0);
    path.quad_to(250.0, 150.0, 200.0, 200.0);
    path.line_to(100.0, 200.0);
    path.close();
    
    vector_engine.add_path(path);
    
    // Create a paint
    let paint = Paint {
        color: Color::rgb(0x2196F3),
        fill_type: FillType::EvenOdd,
        stroke_width: 2.0,
        stroke_color: Color::rgb(0x1976D2),
        ..Default::default()
    };
    
    vector_engine.add_paint(paint);
    
    println!("Vector Engine:");
    println!("  Paths: {}", vector_engine.paths().len());
    println!("  Paints: {}", vector_engine.paints().len());
    println!();
    
    // Create UI widgets
    let mut button = Button::new("Click Me".to_string())
        .with_bounds(100.0, 300.0, 120.0, 40.0);
    
    let mut text_field = TextField::new("Enter text...".to_string())
        .with_bounds(100.0, 360.0, 200.0, 30.0);
    
    let mut list_view = ListView::new(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
        "Item 4".to_string(),
        "Item 5".to_string(),
    ])
        .with_bounds(100.0, 410.0, 200.0, 150.0);
    
    println!("UI Widgets:");
    println!("  Button: {}", button.text);
    println!("  TextField: {}", text_field.placeholder);
    println!("  ListView: {} items", list_view.items.len());
    println!();
    
    // Test event handling
    let click_event = Event::new(EventType::Click, 150.0, 320.0);
    button.handle_event(&click_event);
    println!("Button clicked: {}", button.is_clicked());
    
    let mouse_move_event = Event::new(EventType::MouseMove, 150.0, 320.0);
    button.handle_event(&mouse_move_event);
    println!("Button hovered: {}", button.hovered);
    
    let list_click_event = Event::new(EventType::Click, 150.0, 430.0);
    list_view.handle_event(&list_click_event);
    println!("Selected item: {:?}", list_view.selected_item());
    println!();
    
    // Test animations
    let animation = Animation::new(Duration::from_secs(1))
        .with_easing(EasingFunction::EaseInOutCubic)
        .with_keyframes(vec![(0.0, 0.0), (1.0, 100.0)]);
    
    println!("Animation:");
    println!("  Duration: {:?}", animation.duration);
    println!("  Easing: {:?}", animation.easing);
    println!("  Keyframes: {:?}", animation.keyframes);
    println!();
    
    // Test animation evaluation
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let value = animation.evaluate(t);
        println!("  t={:.2}: value={:.2}", t, value);
    }
    println!();
    
    // Test all easing functions
    println!("Easing Functions (t=0.5):");
    let easing_functions = vec![
        ("Linear", EasingFunction::Linear),
        ("EaseIn", EasingFunction::EaseIn),
        ("EaseOut", EasingFunction::EaseOut),
        ("EaseInOut", EasingFunction::EaseInOut),
        ("EaseInCubic", EasingFunction::EaseInCubic),
        ("EaseOutCubic", EasingFunction::EaseOutCubic),
        ("EaseInOutCubic", EasingFunction::EaseInOutCubic),
        ("EaseOutBounce", EasingFunction::EaseOutBounce),
        ("EaseOutElastic", EasingFunction::EaseOutElastic),
    ];
    
    for (name, easing) in easing_functions {
        let value = easing.apply(0.5);
        println!("  {}: {:.4}", name, value);
    }
    println!();
    
    // Test color creation
    let blue = Color::rgb(0x2196F3);
    let red = Color::rgb(0xF44336);
    let green = Color::rgb(0x4CAF50);
    let transparent = Color::rgba(0x2196F3FF);
    
    println!("Colors:");
    println!("  Blue: R={}, G={}, B={}, A={}", blue.r, blue.g, blue.b, blue.a);
    println!("  Red: R={}, G={}, B={}, A={}", red.r, red.g, red.b, red.a);
    println!("  Green: R={}, G={}, B={}, A={}", green.r, green.g, green.b, green.a);
    println!("  Transparent: R={}, G={}, B={}, A={}", transparent.r, transparent.g, transparent.b, transparent.a);
    println!();
    
    println!("Demo completed successfully!");
    
    Ok(())
}