//! Comprehensive test suite for flux-vector-engine module
//! Based on GitHub Issue #11 - 85 new tests

use flux_vector_engine::*;

// ============================================================================
// Vector Engine Tests (4 tests)
// ============================================================================

#[test]
fn test_engine_creation() {
    let engine = VectorEngine::new();
    assert!(engine.is_initialized());
}

#[test]
fn test_engine_path_management() {
    let mut engine = VectorEngine::new();
    let path = Path::new();
    engine.add_path(path);
    assert_eq!(engine.path_count(), 1);
}

#[test]
fn test_engine_paint_management() {
    let mut engine = VectorEngine::new();
    let paint = Paint::default();
    engine.set_paint(paint);
    assert!(engine.has_paint());
}

#[test]
fn test_engine_clear() {
    let mut engine = VectorEngine::new();
    engine.add_path(Path::new());
    engine.clear();
    assert_eq!(engine.path_count(), 0);
}

// ============================================================================
// Path Operations Tests (8 tests)
// ============================================================================

#[test]
fn test_path_creation() {
    let path = Path::new();
    assert!(path.is_empty());
}

#[test]
fn test_path_move_to() {
    let mut path = Path::new();
    path.move_to(10.0, 20.0);
    assert!(!path.is_empty());
}

#[test]
fn test_path_line_to() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0);
    path.line_to(100.0, 100.0);
    assert_eq!(path.point_count(), 2);
}

#[test]
fn test_path_quadratic_curve() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0);
    path.quad_to(50.0, 50.0, 100.0, 0.0);
    assert!(!path.is_empty());
}

#[test]
fn test_path_cubic_curve() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0);
    path.cubic_to(25.0, 50.0, 75.0, 50.0, 100.0, 0.0);
    assert!(!path.is_empty());
}

#[test]
fn test_path_close() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0);
    path.line_to(100.0, 0.0);
    path.line_to(100.0, 100.0);
    path.close();
    assert!(path.is_closed());
}

#[test]
fn test_path_multiple_commands() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0);
    path.line_to(100.0, 0.0);
    path.line_to(100.0, 100.0);
    path.line_to(0.0, 100.0);
    path.close();
    assert!(path.is_closed());
}

#[test]
fn test_path_bounds() {
    let mut path = Path::new();
    path.move_to(10.0, 20.0);
    path.line_to(110.0, 120.0);
    let bounds = path.bounds();
    assert_eq!(bounds.min_x, 10.0);
    assert_eq!(bounds.min_y, 20.0);
    assert_eq!(bounds.max_x, 110.0);
    assert_eq!(bounds.max_y, 120.0);
}

// ============================================================================
// Color Operations Tests (6 tests)
// ============================================================================

#[test]
fn test_color_rgb_creation() {
    let color = Color::rgb(255, 128, 64);
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 128);
    assert_eq!(color.b(), 64);
}

#[test]
fn test_color_rgba_creation() {
    let color = Color::rgba(255, 128, 64, 128);
    assert_eq!(color.a(), 128);
}

#[test]
fn test_color_components() {
    let color = Color::rgb(100, 150, 200);
    assert_eq!(color.r(), 100);
    assert_eq!(color.g(), 150);
    assert_eq!(color.b(), 200);
}

#[test]
fn test_color_alpha_manipulation() {
    let mut color = Color::rgb(255, 0, 0);
    color.set_alpha(128);
    assert_eq!(color.a(), 128);
}

#[test]
fn test_color_white() {
    let white = Color::WHITE;
    assert_eq!(white.r(), 255);
    assert_eq!(white.g(), 255);
    assert_eq!(white.b(), 255);
}

#[test]
fn test_color_black() {
    let black = Color::BLACK;
    assert_eq!(black.r(), 0);
    assert_eq!(black.g(), 0);
    assert_eq!(black.b(), 0);
}

// ============================================================================
// Paint Configuration Tests (8 tests)
// ============================================================================

#[test]
fn test_paint_default() {
    let paint = Paint::default();
    assert!(paint.has_fill());
}

#[test]
fn test_paint_stroke_width() {
    let mut paint = Paint::default();
    paint.set_stroke_width(2.5);
    assert_eq!(paint.stroke_width(), 2.5);
}

#[test]
fn test_paint_stroke_cap_butt() {
    let mut paint = Paint::default();
    paint.set_stroke_cap(StrokeCap::Butt);
    assert_eq!(paint.stroke_cap(), StrokeCap::Butt);
}

#[test]
fn test_paint_stroke_cap_round() {
    let mut paint = Paint::default();
    paint.set_stroke_cap(StrokeCap::Round);
    assert_eq!(paint.stroke_cap(), StrokeCap::Round);
}

#[test]
fn test_paint_stroke_join_miter() {
    let mut paint = Paint::default();
    paint.set_stroke_join(StrokeJoin::Miter);
    assert_eq!(paint.stroke_join(), StrokeJoin::Miter);
}

#[test]
fn test_paint_stroke_join_round() {
    let mut paint = Paint::default();
    paint.set_stroke_join(StrokeJoin::Round);
    assert_eq!(paint.stroke_join(), StrokeJoin::Round);
}

#[test]
fn test_paint_fill_nonzero() {
    let mut paint = Paint::default();
    paint.set_fill_type(FillType::NonZero);
    assert_eq!(paint.fill_type(), FillType::NonZero);
}

#[test]
fn test_paint_fill_evenodd() {
    let mut paint = Paint::default();
    paint.set_fill_type(FillType::EvenOdd);
    assert_eq!(paint.fill_type(), FillType::EvenOdd);
}

// ============================================================================
// Button Tests (10 tests)
// ============================================================================

#[test]
fn test_button_creation() {
    let button = Button::new("Click Me");
    assert_eq!(button.text(), "Click Me");
}

#[test]
fn test_button_bounds() {
    let mut button = Button::new("Test");
    button.set_bounds(10.0, 20.0, 100.0, 40.0);
    let bounds = button.bounds();
    assert_eq!(bounds.x, 10.0);
    assert_eq!(bounds.y, 20.0);
    assert_eq!(bounds.width, 100.0);
    assert_eq!(bounds.height, 40.0);
}

#[test]
fn test_button_default_state() {
    let button = Button::new("Test");
    assert_eq!(button.state(), ButtonState::Normal);
}

#[test]
fn test_button_hover_state() {
    let mut button = Button::new("Test");
    button.set_state(ButtonState::Hover);
    assert_eq!(button.state(), ButtonState::Hover);
}

#[test]
fn test_button_pressed_state() {
    let mut button = Button::new("Test");
    button.set_state(ButtonState::Pressed);
    assert_eq!(button.state(), ButtonState::Pressed);
}

#[test]
fn test_button_disabled_state() {
    let mut button = Button::new("Test");
    button.set_enabled(false);
    assert!(!button.is_enabled());
}

#[test]
fn test_button_click_handler() {
    let mut clicked = false;
    let mut button = Button::new("Click");
    button.set_on_click(|| {
        clicked = true;
    });
    button.click();
    assert!(clicked);
}

#[test]
fn test_button_style() {
    let mut button = Button::new("Styled");
    button.set_background_color(Color::rgb(0, 123, 255));
    let bg = button.background_color();
    assert_eq!(bg.r(), 0);
    assert_eq!(bg.g(), 123);
    assert_eq!(bg.b(), 255);
}

#[test]
fn test_button_text_color() {
    let mut button = Button::new("Text");
    button.set_text_color(Color::WHITE);
    let tc = button.text_color();
    assert_eq!(tc.r(), 255);
    assert_eq!(tc.g(), 255);
    assert_eq!(tc.b(), 255);
}

#[test]
fn test_button_contains_point() {
    let mut button = Button::new("Test");
    button.set_bounds(0.0, 0.0, 100.0, 50.0);
    assert!(button.contains_point(50.0, 25.0));
    assert!(!button.contains_point(150.0, 25.0));
}

// ============================================================================
// TextField Tests (7 tests)
// ============================================================================

#[test]
fn test_textfield_creation() {
    let tf = TextField::new();
    assert!(tf.text().is_empty());
}

#[test]
fn test_textfield_set_text() {
    let mut tf = TextField::new();
    tf.set_text("Hello World");
    assert_eq!(tf.text(), "Hello World");
}

#[test]
fn test_textfield_focus() {
    let mut tf = TextField::new();
    tf.set_focused(true);
    assert!(tf.is_focused());
}

#[test]
fn test_textfield_placeholder() {
    let mut tf = TextField::new();
    tf.set_placeholder("Enter text...");
    assert_eq!(tf.placeholder(), "Enter text...");
}

#[test]
fn test_textfield_cursor_position() {
    let mut tf = TextField::new();
    tf.set_text("Hello");
    tf.set_cursor_position(2);
    assert_eq!(tf.cursor_position(), 2);
}

#[test]
fn test_textfield_selection() {
    let mut tf = TextField::new();
    tf.set_text("Hello World");
    tf.set_selection(0, 5);
    assert_eq!(tf.selection_start(), 0);
    assert_eq!(tf.selection_end(), 5);
}

#[test]
fn test_textfield_max_length() {
    let mut tf = TextField::new();
    tf.set_max_length(10);
    tf.set_text("This is a very long text");
    assert!(tf.text().len() <= 10);
}

// ============================================================================
// ListView Tests (11 tests)
// ============================================================================

#[test]
fn test_listview_creation() {
    let lv = ListView::new();
    assert!(lv.is_empty());
}

#[test]
fn test_listview_add_item() {
    let mut lv = ListView::new();
    lv.add_item("Item 1");
    assert_eq!(lv.item_count(), 1);
}

#[test]
fn test_listview_multiple_items() {
    let mut lv = ListView::new();
    lv.add_item("Item 1");
    lv.add_item("Item 2");
    lv.add_item("Item 3");
    assert_eq!(lv.item_count(), 3);
}

#[test]
fn test_listview_remove_item() {
    let mut lv = ListView::new();
    lv.add_item("Item 1");
    lv.add_item("Item 2");
    lv.remove_item(0);
    assert_eq!(lv.item_count(), 1);
}

#[test]
fn test_listview_clear() {
    let mut lv = ListView::new();
    lv.add_item("Item 1");
    lv.add_item("Item 2");
    lv.clear();
    assert!(lv.is_empty());
}

#[test]
fn test_listview_selection() {
    let mut lv = ListView::new();
    lv.add_item("Item 1");
    lv.add_item("Item 2");
    lv.select(1);
    assert_eq!(lv.selected_index(), Some(1));
}

#[test]
fn test_listview_no_selection() {
    let lv = ListView::new();
    assert_eq!(lv.selected_index(), None);
}

#[test]
fn test_listview_scroll() {
    let mut lv = ListView::new();
    for i in 0..20 {
        lv.add_item(format!("Item {}", i));
    }
    lv.set_scroll_position(5);
    assert_eq!(lv.scroll_position(), 5);
}

#[test]
fn test_listview_visible_range() {
    let mut lv = ListView::new();
    lv.set_visible_items(5);
    for i in 0..20 {
        lv.add_item(format!("Item {}", i));
    }
    let (start, end) = lv.visible_range();
    assert_eq!(end - start, 5);
}

#[test]
fn test_listview_item_height() {
    let mut lv = ListView::new();
    lv.set_item_height(30.0);
    assert_eq!(lv.item_height(), 30.0);
}

#[test]
fn test_listview_on_select_callback() {
    let mut selected = None;
    let mut lv = ListView::new();
    lv.add_item("Item 1");
    lv.add_item("Item 2");
    lv.set_on_select(|index| {
        selected = Some(index);
    });
    lv.select(1);
    assert_eq!(selected, Some(1));
}

// ============================================================================
// Animation System Tests (8 tests)
// ============================================================================

#[test]
fn test_animation_creation() {
    let anim = Animation::new();
    assert_eq!(anim.duration(), Duration::from_millis(0));
}

#[test]
fn test_animation_duration() {
    let mut anim = Animation::new();
    anim.set_duration(Duration::from_millis(1000));
    assert_eq!(anim.duration(), Duration::from_millis(1000));
}

#[test]
fn test_animation_keyframe() {
    let mut anim = Animation::new();
    anim.add_keyframe(Keyframe {
        time: 0.0,
        value: AnimationValue::Float(0.0),
        easing: Easing::Linear,
    });
    assert_eq!(anim.keyframe_count(), 1);
}

#[test]
fn test_animation_multiple_keyframes() {
    let mut anim = Animation::new();
    anim.add_keyframe(Keyframe {
        time: 0.0,
        value: AnimationValue::Float(0.0),
        easing: Easing::Linear,
    });
    anim.add_keyframe(Keyframe {
        time: 1.0,
        value: AnimationValue::Float(100.0),
        easing: Easing::EaseOut,
    });
    assert_eq!(anim.keyframe_count(), 2);
}

#[test]
fn test_animation_interpolation() {
    let mut anim = Animation::new();
    anim.add_keyframe(Keyframe {
        time: 0.0,
        value: AnimationValue::Float(0.0),
        easing: Easing::Linear,
    });
    anim.add_keyframe(Keyframe {
        time: 1.0,
        value: AnimationValue::Float(100.0),
        easing: Easing::Linear,
    });
    let value = anim.interpolate(0.5);
    assert_eq!(value, AnimationValue::Float(50.0));
}

#[test]
fn test_animation_engine_creation() {
    let engine = AnimationEngine::new();
    assert!(engine.is_empty());
}

#[test]
fn test_animation_engine_add_animation() {
    let mut engine = AnimationEngine::new();
    let anim = Animation::new();
    engine.add_animation("test", anim);
    assert!(!engine.is_empty());
}

#[test]
fn test_animation_engine_remove_animation() {
    let mut engine = AnimationEngine::new();
    let anim = Animation::new();
    engine.add_animation("test", anim);
    engine.remove_animation("test");
    assert!(engine.is_empty());
}

// ============================================================================
// Easing Functions Tests (8 tests)
// ============================================================================

#[test]
fn test_easing_linear() {
    assert_eq!(Easing::Linear.apply(0.5), 0.5);
}

#[test]
fn test_easing_ease_in() {
    let result = Easing::EaseIn.apply(0.5);
    assert!(result < 0.5); // Should accelerate
}

#[test]
fn test_easing_ease_out() {
    let result = Easing::EaseOut.apply(0.5);
    assert!(result > 0.5); // Should decelerate
}

#[test]
fn test_easing_ease_in_out() {
    let result = Easing::EaseInOut.apply(0.5);
    // At midpoint, should be close to 0.5
    assert!(result > 0.4 && result < 0.6);
}

#[test]
fn test_easing_quad() {
    let result = Easing::Quad.apply(0.5);
    assert!(result >= 0.0 && result <= 1.0);
}

#[test]
fn test_easing_cubic() {
    let result = Easing::Cubic.apply(0.5);
    assert!(result >= 0.0 && result <= 1.0);
}

#[test]
fn test_easing_elastic() {
    let result = Easing::Elastic.apply(0.5);
    assert!(result >= -1.0 && result <= 2.0); // Can overshoot
}

#[test]
fn test_easing_bounce() {
    let result = Easing::Bounce.apply(0.5);
    assert!(result >= 0.0 && result <= 1.0);
}

// ============================================================================
// Event Handling Tests (2 tests)
// ============================================================================

#[test]
fn test_event_creation() {
    let event = Event::MouseDown { x: 100.0, y: 200.0, button: MouseButton::Left };
    match event {
        Event::MouseDown { x, y, button } => {
            assert_eq!(x, 100.0);
            assert_eq!(y, 200.0);
            assert_eq!(button, MouseButton::Left);
        }
        _ => panic!("Wrong event type"),
    }
}

#[test]
fn test_event_context() {
    let mut ctx = EventContext::new();
    ctx.set_mouse_position(50.0, 75.0);
    assert_eq!(ctx.mouse_x(), 50.0);
    assert_eq!(ctx.mouse_y(), 75.0);
}

// ============================================================================
// Configuration Tests (7 tests)
// ============================================================================

#[test]
fn test_window_config_default() {
    let config = WindowConfig::default();
    assert!(config.width > 0);
    assert!(config.height > 0);
}

#[test]
fn test_window_config_vsync() {
    let mut config = WindowConfig::default();
    config.vsync = VSyncMode::Enabled;
    assert_eq!(config.vsync, VSyncMode::Enabled);
}

#[test]
fn test_render_config_default() {
    let config = RenderConfig::default();
    assert!(config.quality > 0);
}

#[test]
fn test_render_config_quality() {
    let mut config = RenderConfig::default();
    config.quality = 2; // High quality
    assert_eq!(config.quality, 2);
}

#[test]
fn test_texture_config() {
    let config = TextureConfig {
        generate_mipmaps: true,
        anisotropic_filtering: 16,
    };
    assert!(config.generate_mipmaps);
    assert_eq!(config.anisotropic_filtering, 16);
}

#[test]
fn test_shadow_config() {
    let config = ShadowConfig {
        enabled: true,
        resolution: 2048,
        softness: 2.0,
    };
    assert!(config.enabled);
    assert_eq!(config.resolution, 2048);
}

#[test]
fn test_antialiasing_config() {
    let config = AntiAliasingConfig {
        mode: AAMode::MSAA,
        samples: 4,
    };
    assert_eq!(config.mode, AAMode::MSAA);
    assert_eq!(config.samples, 4);
}

// ============================================================================
// Integration Tests (6 tests)
// ============================================================================

#[test]
fn test_complex_path_drawing() {
    let mut engine = VectorEngine::new();
    let mut path = Path::new();
    path.move_to(0.0, 0.0);
    path.line_to(100.0, 0.0);
    path.line_to(100.0, 100.0);
    path.quad_to(50.0, 150.0, 0.0, 100.0);
    path.close();
    engine.add_path(path);
    assert_eq!(engine.path_count(), 1);
}

#[test]
fn test_button_state_sequence() {
    let mut button = Button::new("Test");
    assert_eq!(button.state(), ButtonState::Normal);
    button.set_state(ButtonState::Hover);
    assert_eq!(button.state(), ButtonState::Hover);
    button.set_state(ButtonState::Pressed);
    assert_eq!(button.state(), ButtonState::Pressed);
    button.set_state(ButtonState::Normal);
    assert_eq!(button.state(), ButtonState::Normal);
}

#[test]
fn test_listview_multi_item_operations() {
    let mut lv = ListView::new();
    for i in 0..10 {
        lv.add_item(format!("Item {}", i));
    }
    assert_eq!(lv.item_count(), 10);
    lv.select(5);
    assert_eq!(lv.selected_index(), Some(5));
    lv.remove_item(5);
    assert_eq!(lv.item_count(), 9);
}

#[test]
fn test_animation_with_multiple_keyframes() {
    let mut anim = Animation::new();
    anim.add_keyframe(Keyframe {
        time: 0.0,
        value: AnimationValue::Float(0.0),
        easing: Easing::EaseIn,
    });
    anim.add_keyframe(Keyframe {
        time: 0.5,
        value: AnimationValue::Float(50.0),
        easing: Easing::Linear,
    });
    anim.add_keyframe(Keyframe {
        time: 1.0,
        value: AnimationValue::Float(100.0),
        easing: Easing::EaseOut,
    });
    let mid = anim.interpolate(0.5);
    assert_eq!(mid, AnimationValue::Float(50.0));
}

#[test]
fn test_complex_scene_rendering() {
    let mut engine = VectorEngine::new();
    
    // Create multiple paths
    for i in 0..5 {
        let mut path = Path::new();
        path.move_to(i as f64 * 10.0, 0.0);
        path.line_to(i as f64 * 10.0 + 50.0, 50.0);
        engine.add_path(path);
    }
    
    // Create paint
    let mut paint = Paint::default();
    paint.set_stroke_width(2.0);
    paint.set_stroke_cap(StrokeCap::Round);
    engine.set_paint(paint);
    
    assert_eq!(engine.path_count(), 5);
}

#[test]
fn test_event_handling_workflow() {
    let mut ctx = EventContext::new();
    let mut button = Button::new("Click");
    button.set_bounds(0.0, 0.0, 100.0, 50.0);
    
    // Simulate mouse move
    ctx.set_mouse_position(50.0, 25.0);
    assert!(button.contains_point(ctx.mouse_x(), ctx.mouse_y()));
    
    // Simulate click
    let mut clicked = false;
    button.set_on_click(|| clicked = true);
    button.click();
    assert!(clicked);
}