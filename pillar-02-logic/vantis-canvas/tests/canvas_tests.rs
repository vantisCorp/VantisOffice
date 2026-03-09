//! Comprehensive tests for Vantis Canvas
//!
//! Tests cover:
//! - Canvas operations (creation, slides, layers)
//! - Element management (shapes, text, images)
//! - Animation system (animations, timeline, easing)
//! - Collaboration (sessions, users, CRDT)
//! - Export functionality (Vantis, SVG, PDF)
//! - Rendering pipeline

use std::path::PathBuf;
use std::time::Duration;

use vantis_canvas::animation::*;
use vantis_canvas::collaboration::*;
use vantis_canvas::core::*;
use vantis_canvas::export::*;
use vantis_canvas::rendering::*;

// ============================================================================
// Canvas Core Tests
// ============================================================================

mod canvas_core {
    use super::*;

    #[test]
    fn test_canvas_creation_defaults() {
        let canvas = Canvas::new("My Presentation".to_string());
        assert_eq!(canvas.name, "My Presentation");
        assert_eq!(canvas.slides.len(), 0);
        assert_eq!(canvas.active_slide, 0);
        assert_eq!(canvas.dimensions.width, 1920.0);
        assert_eq!(canvas.dimensions.height, 1080.0);
        assert!(canvas.dimensions.infinite);
        assert!(canvas.metadata.author.is_none());
        assert!(canvas.metadata.description.is_none());
        assert_eq!(canvas.metadata.tags.len(), 0);
    }

    #[test]
    fn test_canvas_add_multiple_slides() {
        let mut canvas = Canvas::new("Test".to_string());
        canvas.add_slide();
        canvas.add_slide();
        canvas.add_slide();
        assert_eq!(canvas.slides.len(), 3);
        assert_eq!(canvas.slides[0].name, "Slide 1");
        assert_eq!(canvas.slides[1].name, "Slide 2");
        assert_eq!(canvas.slides[2].name, "Slide 3");
    }

    #[test]
    fn test_canvas_active_slide_navigation() {
        let mut canvas = Canvas::new("Test".to_string());
        canvas.add_slide();
        canvas.add_slide();
        canvas.add_slide();

        assert_eq!(canvas.active_slide, 0);

        assert!(canvas.set_active_slide(1).is_ok());
        assert_eq!(canvas.active_slide, 1);

        assert!(canvas.set_active_slide(2).is_ok());
        assert_eq!(canvas.active_slide, 2);
    }

    #[test]
    fn test_canvas_active_slide_out_of_range() {
        let mut canvas = Canvas::new("Test".to_string());
        canvas.add_slide();

        let result = canvas.set_active_slide(5);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of range"));
    }

    #[test]
    fn test_canvas_get_active_slide() {
        let mut canvas = Canvas::new("Test".to_string());
        assert!(canvas.get_active_slide().is_none());

        canvas.add_slide();
        assert!(canvas.get_active_slide().is_some());
        assert_eq!(canvas.get_active_slide().unwrap().name, "Slide 1");
    }

    #[test]
    fn test_canvas_get_active_slide_mut() {
        let mut canvas = Canvas::new("Test".to_string());
        canvas.add_slide();

        if let Some(slide) = canvas.get_active_slide_mut() {
            slide.name = "Modified Slide".to_string();
        }

        assert_eq!(canvas.get_active_slide().unwrap().name, "Modified Slide");
    }

    #[test]
    fn test_canvas_serialization() {
        let canvas = Canvas::new("Serialization Test".to_string());
        let json = serde_json::to_string(&canvas).unwrap();
        let deserialized: Canvas = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "Serialization Test");
    }

    #[test]
    fn test_canvas_background_solid() {
        let canvas = Canvas::new("Test".to_string());
        match &canvas.background {
            Background::Solid(color) => assert_eq!(color, "#FFFFFF"),
            _ => panic!("Expected solid background"),
        }
    }

    #[test]
    fn test_canvas_background_gradient() {
        let bg = Background::Gradient {
            start: "#FF0000".to_string(),
            end: "#0000FF".to_string(),
            direction: GradientDirection::Horizontal,
        };
        let json = serde_json::to_string(&bg).unwrap();
        let deserialized: Background = serde_json::from_str(&json).unwrap();
        match deserialized {
            Background::Gradient { start, end, .. } => {
                assert_eq!(start, "#FF0000");
                assert_eq!(end, "#0000FF");
            }
            _ => panic!("Expected gradient background"),
        }
    }

    #[test]
    fn test_canvas_metadata_timestamps() {
        let canvas = Canvas::new("Test".to_string());
        assert!(canvas.metadata.created <= canvas.metadata.modified);
    }
}

// ============================================================================
// Slide Tests
// ============================================================================

mod slide_tests {
    use super::*;

    #[test]
    fn test_slide_creation() {
        let slide = Slide::new(0);
        assert_eq!(slide.index, 0);
        assert_eq!(slide.name, "Slide 1");
        assert_eq!(slide.layers.len(), 0);
        assert!(slide.duration.is_none());
        assert!(slide.transition.is_none());
        assert!(slide.notes.is_empty());
    }

    #[test]
    fn test_slide_add_layer() {
        let mut slide = Slide::new(0);
        slide.add_layer();
        slide.add_layer();
        assert_eq!(slide.layers.len(), 2);
        assert_eq!(slide.layers[0].name, "Layer 1");
        assert_eq!(slide.layers[1].name, "Layer 2");
    }

    #[test]
    fn test_slide_add_shape_creates_layer() {
        let mut slide = Slide::new(0);
        assert_eq!(slide.layers.len(), 0);

        let shape = Shape::new(ShapeType::Rectangle);
        slide.add_shape(shape);

        assert_eq!(slide.layers.len(), 1);
        assert_eq!(slide.layers[0].shapes.len(), 1);
    }

    #[test]
    fn test_slide_add_shape_to_existing_layer() {
        let mut slide = Slide::new(0);
        slide.add_layer();

        let shape1 = Shape::new(ShapeType::Rectangle);
        let shape2 = Shape::new(ShapeType::Circle);
        slide.add_shape(shape1);
        slide.add_shape(shape2);

        assert_eq!(slide.layers.len(), 1);
        assert_eq!(slide.layers[0].shapes.len(), 2);
    }

    #[test]
    fn test_slide_add_text_creates_layer() {
        let mut slide = Slide::new(0);
        let text = Text::new("Hello".to_string());
        slide.add_text(text);

        assert_eq!(slide.layers.len(), 1);
        assert_eq!(slide.layers[0].texts.len(), 1);
    }

    #[test]
    fn test_slide_add_image_creates_layer() {
        let mut slide = Slide::new(0);
        let image = Image::new("/path/to/image.png".to_string());
        slide.add_image(image);

        assert_eq!(slide.layers.len(), 1);
        assert_eq!(slide.layers[0].images.len(), 1);
    }

    #[test]
    fn test_slide_transition_types() {
        let mut slide = Slide::new(0);
        slide.transition = Some(TransitionType::Fade);
        assert!(slide.transition.is_some());

        slide.transition = Some(TransitionType::ZoomIn);
        assert!(slide.transition.is_some());
    }

    #[test]
    fn test_slide_notes() {
        let mut slide = Slide::new(0);
        slide.notes = "Speaker notes for this slide".to_string();
        assert_eq!(slide.notes, "Speaker notes for this slide");
    }

    #[test]
    fn test_slide_duration() {
        let mut slide = Slide::new(0);
        slide.duration = Some(5.0);
        assert_eq!(slide.duration, Some(5.0));
    }
}

// ============================================================================
// Layer Tests
// ============================================================================

mod layer_tests {
    use super::*;

    #[test]
    fn test_layer_creation_defaults() {
        let layer = Layer::new(0);
        assert_eq!(layer.index, 0);
        assert_eq!(layer.name, "Layer 1");
        assert!(layer.visible);
        assert!(!layer.locked);
        assert_eq!(layer.opacity, 1.0);
        assert_eq!(layer.shapes.len(), 0);
        assert_eq!(layer.texts.len(), 0);
        assert_eq!(layer.images.len(), 0);
    }

    #[test]
    fn test_layer_add_shape() {
        let mut layer = Layer::new(0);
        let shape = Shape::new(ShapeType::Circle);
        layer.add_shape(shape);
        assert_eq!(layer.shapes.len(), 1);
    }

    #[test]
    fn test_layer_add_text() {
        let mut layer = Layer::new(0);
        let text = Text::new("Test text".to_string());
        layer.add_text(text);
        assert_eq!(layer.texts.len(), 1);
    }

    #[test]
    fn test_layer_add_image() {
        let mut layer = Layer::new(0);
        let image = Image::new("test.png".to_string());
        layer.add_image(image);
        assert_eq!(layer.images.len(), 1);
    }

    #[test]
    fn test_layer_visibility() {
        let mut layer = Layer::new(0);
        assert!(layer.visible);
        layer.visible = false;
        assert!(!layer.visible);
    }

    #[test]
    fn test_layer_locked() {
        let mut layer = Layer::new(0);
        assert!(!layer.locked);
        layer.locked = true;
        assert!(layer.locked);
    }

    #[test]
    fn test_layer_opacity() {
        let mut layer = Layer::new(0);
        layer.opacity = 0.5;
        assert_eq!(layer.opacity, 0.5);
    }

    #[test]
    fn test_layer_blend_modes() {
        let mut layer = Layer::new(0);
        layer.blend_mode = BlendMode::Multiply;
        let json = serde_json::to_string(&layer.blend_mode).unwrap();
        assert!(json.contains("Multiply"));
    }

    #[test]
    fn test_layer_multiple_elements() {
        let mut layer = Layer::new(0);
        layer.add_shape(Shape::new(ShapeType::Rectangle));
        layer.add_shape(Shape::new(ShapeType::Circle));
        layer.add_text(Text::new("Title".to_string()));
        layer.add_text(Text::new("Subtitle".to_string()));
        layer.add_image(Image::new("bg.png".to_string()));

        assert_eq!(layer.shapes.len(), 2);
        assert_eq!(layer.texts.len(), 2);
        assert_eq!(layer.images.len(), 1);
    }
}

// ============================================================================
// Shape Tests
// ============================================================================

mod shape_tests {
    use super::*;

    #[test]
    fn test_shape_creation_defaults() {
        let shape = Shape::new(ShapeType::Rectangle);
        assert_eq!(shape.shape_type, ShapeType::Rectangle);
        assert_eq!(shape.position.x, 0.0);
        assert_eq!(shape.position.y, 0.0);
        assert_eq!(shape.size.width, 100.0);
        assert_eq!(shape.size.height, 100.0);
        assert_eq!(shape.rotation, 0.0);
        assert!(shape.fill.is_none());
        assert!(shape.stroke.is_none());
        assert_eq!(shape.effects.len(), 0);
        assert!(!shape.id.is_empty());
    }

    #[test]
    fn test_shape_builder_pattern() {
        let shape = Shape::new(ShapeType::Circle)
            .with_position(100.0, 200.0)
            .with_size(50.0, 50.0)
            .with_fill(Fill::Solid("#FF0000".to_string()));

        assert_eq!(shape.position.x, 100.0);
        assert_eq!(shape.position.y, 200.0);
        assert_eq!(shape.size.width, 50.0);
        assert_eq!(shape.size.height, 50.0);
        assert!(shape.fill.is_some());
    }

    #[test]
    fn test_shape_with_stroke() {
        let stroke = Stroke {
            color: "#000000".to_string(),
            width: 2.0,
            style: StrokeStyle::Solid,
            dash_pattern: None,
        };
        let shape = Shape::new(ShapeType::Rectangle).with_stroke(stroke);
        assert!(shape.stroke.is_some());
        assert_eq!(shape.stroke.unwrap().width, 2.0);
    }

    #[test]
    fn test_shape_types() {
        let rect = Shape::new(ShapeType::Rectangle);
        assert_eq!(rect.shape_type, ShapeType::Rectangle);

        let circle = Shape::new(ShapeType::Circle);
        assert_eq!(circle.shape_type, ShapeType::Circle);

        let ellipse = Shape::new(ShapeType::Ellipse);
        assert_eq!(ellipse.shape_type, ShapeType::Ellipse);

        let triangle = Shape::new(ShapeType::Triangle);
        assert_eq!(triangle.shape_type, ShapeType::Triangle);

        let polygon = Shape::new(ShapeType::Polygon { sides: 6 });
        assert_eq!(polygon.shape_type, ShapeType::Polygon { sides: 6 });

        let star = Shape::new(ShapeType::Star {
            points: 5,
            inner_radius: 0.5,
        });
        assert_eq!(
            star.shape_type,
            ShapeType::Star {
                points: 5,
                inner_radius: 0.5
            }
        );
    }

    #[test]
    fn test_shape_unique_ids() {
        let shape1 = Shape::new(ShapeType::Rectangle);
        let shape2 = Shape::new(ShapeType::Rectangle);
        assert_ne!(shape1.id, shape2.id);
    }

    #[test]
    fn test_shape_serialization() {
        let shape = Shape::new(ShapeType::Circle)
            .with_position(10.0, 20.0)
            .with_size(30.0, 30.0)
            .with_fill(Fill::Solid("#00FF00".to_string()));

        let json = serde_json::to_string(&shape).unwrap();
        let deserialized: Shape = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.position.x, 10.0);
        assert_eq!(deserialized.position.y, 20.0);
        assert_eq!(deserialized.size.width, 30.0);
    }

    #[test]
    fn test_shape_effects() {
        let mut shape = Shape::new(ShapeType::Rectangle);
        shape.effects.push(Effect::Shadow {
            offset_x: 2.0,
            offset_y: 2.0,
            blur: 4.0,
            color: "#00000080".to_string(),
        });
        shape.effects.push(Effect::Glow {
            blur: 8.0,
            color: "#FFFFFF".to_string(),
        });
        assert_eq!(shape.effects.len(), 2);
    }

    #[test]
    fn test_shape_fill_types() {
        let solid = Fill::Solid("#FF0000".to_string());
        let gradient = Fill::Gradient {
            start: "#FF0000".to_string(),
            end: "#0000FF".to_string(),
            direction: GradientDirection::Vertical,
        };
        let pattern = Fill::Pattern {
            pattern: "dots".to_string(),
            color: "#000000".to_string(),
        };
        let none = Fill::None;

        // Verify serialization for each fill type
        assert!(serde_json::to_string(&solid).is_ok());
        assert!(serde_json::to_string(&gradient).is_ok());
        assert!(serde_json::to_string(&pattern).is_ok());
        assert!(serde_json::to_string(&none).is_ok());
    }

    #[test]
    fn test_shape_stroke_styles() {
        let solid_stroke = Stroke {
            color: "#000".to_string(),
            width: 1.0,
            style: StrokeStyle::Solid,
            dash_pattern: None,
        };
        let dashed_stroke = Stroke {
            color: "#000".to_string(),
            width: 2.0,
            style: StrokeStyle::Dashed,
            dash_pattern: Some(vec![5.0, 3.0]),
        };

        assert!(serde_json::to_string(&solid_stroke).is_ok());
        assert!(serde_json::to_string(&dashed_stroke).is_ok());
    }

    #[test]
    fn test_shape_path_and_freehand() {
        let path = Shape::new(ShapeType::Path {
            points: vec![(0.0, 0.0), (50.0, 100.0), (100.0, 0.0)],
        });
        let freehand = Shape::new(ShapeType::Freehand {
            points: vec![(0.0, 0.0), (10.0, 5.0), (20.0, 15.0)],
        });

        assert!(serde_json::to_string(&path).is_ok());
        assert!(serde_json::to_string(&freehand).is_ok());
    }
}

// ============================================================================
// Text Tests
// ============================================================================

mod text_tests {
    use super::*;

    #[test]
    fn test_text_creation_defaults() {
        let text = Text::new("Hello World".to_string());
        assert_eq!(text.content, "Hello World");
        assert_eq!(text.position.x, 0.0);
        assert_eq!(text.position.y, 0.0);
        assert_eq!(text.font.family, "Arial");
        assert_eq!(text.font.size, 24.0);
        assert!(!text.id.is_empty());
    }

    #[test]
    fn test_text_builder_pattern() {
        let text = Text::new("Title".to_string())
            .with_position(100.0, 50.0)
            .with_font("Helvetica".to_string(), 48.0);

        assert_eq!(text.position.x, 100.0);
        assert_eq!(text.position.y, 50.0);
        assert_eq!(text.font.family, "Helvetica");
        assert_eq!(text.font.size, 48.0);
    }

    #[test]
    fn test_text_unique_ids() {
        let text1 = Text::new("A".to_string());
        let text2 = Text::new("B".to_string());
        assert_ne!(text1.id, text2.id);
    }

    #[test]
    fn test_text_serialization() {
        let text = Text::new("Serializable".to_string())
            .with_position(10.0, 20.0)
            .with_font("Courier".to_string(), 16.0);

        let json = serde_json::to_string(&text).unwrap();
        let deserialized: Text = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.content, "Serializable");
        assert_eq!(deserialized.font.family, "Courier");
        assert_eq!(deserialized.font.size, 16.0);
    }

    #[test]
    fn test_text_default_fill() {
        let text = Text::new("Test".to_string());
        assert!(text.fill.is_some());
        match text.fill.unwrap() {
            Fill::Solid(color) => assert_eq!(color, "#000000"),
            _ => panic!("Expected solid fill"),
        }
    }

    #[test]
    fn test_text_alignment_variants() {
        let alignments = vec![
            TextAlignment::Left,
            TextAlignment::Center,
            TextAlignment::Right,
            TextAlignment::Justify,
        ];
        for alignment in alignments {
            assert!(serde_json::to_string(&alignment).is_ok());
        }
    }

    #[test]
    fn test_text_font_weights() {
        let weights = vec![
            FontWeight::Thin,
            FontWeight::ExtraLight,
            FontWeight::Light,
            FontWeight::Normal,
            FontWeight::Medium,
            FontWeight::SemiBold,
            FontWeight::Bold,
            FontWeight::ExtraBold,
            FontWeight::Black,
        ];
        for weight in weights {
            assert!(serde_json::to_string(&weight).is_ok());
        }
    }

    #[test]
    fn test_text_font_styles() {
        let styles = vec![FontStyle::Normal, FontStyle::Italic, FontStyle::Oblique];
        for style in styles {
            assert!(serde_json::to_string(&style).is_ok());
        }
    }
}

// ============================================================================
// Image Tests
// ============================================================================

mod image_tests {
    use super::*;

    #[test]
    fn test_image_creation_defaults() {
        let image = Image::new("/path/to/image.png".to_string());
        assert_eq!(image.path, "/path/to/image.png");
        assert_eq!(image.position.x, 0.0);
        assert_eq!(image.position.y, 0.0);
        assert_eq!(image.size.width, 100.0);
        assert_eq!(image.size.height, 100.0);
        assert_eq!(image.rotation, 0.0);
        assert_eq!(image.opacity, 1.0);
        assert!(!image.id.is_empty());
    }

    #[test]
    fn test_image_builder_pattern() {
        let image = Image::new("photo.jpg".to_string())
            .with_position(50.0, 75.0)
            .with_size(200.0, 150.0);

        assert_eq!(image.position.x, 50.0);
        assert_eq!(image.position.y, 75.0);
        assert_eq!(image.size.width, 200.0);
        assert_eq!(image.size.height, 150.0);
    }

    #[test]
    fn test_image_unique_ids() {
        let img1 = Image::new("a.png".to_string());
        let img2 = Image::new("b.png".to_string());
        assert_ne!(img1.id, img2.id);
    }

    #[test]
    fn test_image_serialization() {
        let image = Image::new("test.png".to_string())
            .with_position(10.0, 20.0)
            .with_size(300.0, 200.0);

        let json = serde_json::to_string(&image).unwrap();
        let deserialized: Image = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.path, "test.png");
        assert_eq!(deserialized.size.width, 300.0);
    }

    #[test]
    fn test_image_opacity() {
        let mut image = Image::new("test.png".to_string());
        image.opacity = 0.5;
        assert_eq!(image.opacity, 0.5);
    }

    #[test]
    fn test_image_rotation() {
        let mut image = Image::new("test.png".to_string());
        image.rotation = 45.0;
        assert_eq!(image.rotation, 45.0);
    }

    #[test]
    fn test_image_effects() {
        let mut image = Image::new("test.png".to_string());
        image.effects.push(Effect::Blur { radius: 5.0 });
        image.effects.push(Effect::Brightness { amount: 1.2 });
        assert_eq!(image.effects.len(), 2);
    }
}

// ============================================================================
// Animation Tests
// ============================================================================

mod animation_tests {
    use super::*;

    #[test]
    fn test_animation_creation() {
        let anim = Animation::new(
            "fade-in".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(500),
        );
        assert_eq!(anim.id, "fade-in");
        assert_eq!(anim.duration, Duration::from_millis(500));
        assert_eq!(anim.state, AnimationState::Idle);
        assert_eq!(anim.elapsed, Duration::from_millis(0));
    }

    #[test]
    fn test_animation_start_stop() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        );

        assert_eq!(anim.state, AnimationState::Idle);

        anim.start();
        assert_eq!(anim.state, AnimationState::Running);

        anim.stop();
        assert_eq!(anim.state, AnimationState::Cancelled);
    }

    #[test]
    fn test_animation_pause_resume() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        );

        anim.start();
        assert_eq!(anim.state, AnimationState::Running);

        anim.pause();
        assert_eq!(anim.state, AnimationState::Paused);

        anim.resume();
        assert_eq!(anim.state, AnimationState::Running);
    }

    #[test]
    fn test_animation_pause_only_when_running() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        );

        // Pause when idle should not change state
        anim.pause();
        assert_eq!(anim.state, AnimationState::Idle);
    }

    #[test]
    fn test_animation_resume_only_when_paused() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        );

        // Resume when idle should not change state
        anim.resume();
        assert_eq!(anim.state, AnimationState::Idle);
    }

    #[test]
    fn test_animation_progress() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        );
        anim.start();
        anim.elapsed = Duration::from_millis(250);
        assert!((anim.get_progress() - 0.25).abs() < 0.01);

        anim.elapsed = Duration::from_millis(500);
        assert!((anim.get_progress() - 0.5).abs() < 0.01);

        anim.elapsed = Duration::from_millis(750);
        assert!((anim.get_progress() - 0.75).abs() < 0.01);

        anim.elapsed = Duration::from_millis(1000);
        assert!((anim.get_progress() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_animation_progress_clamped() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        );
        anim.start();
        anim.elapsed = Duration::from_millis(2000);
        assert!((anim.get_progress() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_animation_zero_duration() {
        let anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(0),
        );
        assert_eq!(anim.get_progress(), 1.0);
    }

    #[test]
    fn test_animation_update_completes() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(100),
        );
        anim.start();
        anim.update(Duration::from_millis(150)).unwrap();
        assert_eq!(anim.state, AnimationState::Completed);
    }

    #[test]
    fn test_animation_update_loop() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(100),
        )
        .with_repeat(RepeatMode::Loop);

        anim.start();
        anim.update(Duration::from_millis(150)).unwrap();
        assert_eq!(anim.state, AnimationState::Running);
        assert_eq!(anim.elapsed, Duration::from_millis(0));
    }

    #[test]
    fn test_animation_update_not_running() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        );
        // Not started, update should be no-op
        let result = anim.update(Duration::from_millis(100));
        assert!(result.is_ok());
        assert_eq!(anim.state, AnimationState::Idle);
    }

    #[test]
    fn test_animation_builder_pattern() {
        let anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        )
        .with_easing(EasingFunction::EaseInOut)
        .with_delay(Duration::from_millis(200))
        .with_repeat(RepeatMode::Loop);

        assert_eq!(anim.delay, Duration::from_millis(200));
    }

    #[test]
    fn test_easing_linear() {
        let anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        );
        let result = anim.get_eased_progress();
        assert_eq!(result, 0.0); // elapsed is 0
    }

    #[test]
    fn test_easing_ease_in_boundaries() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        )
        .with_easing(EasingFunction::EaseIn);

        anim.elapsed = Duration::from_millis(0);
        assert!((anim.get_eased_progress() - 0.0).abs() < 0.01);

        anim.elapsed = Duration::from_millis(1000);
        assert!((anim.get_eased_progress() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_easing_ease_out_boundaries() {
        let mut anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000),
        )
        .with_easing(EasingFunction::EaseOut);

        anim.elapsed = Duration::from_millis(0);
        assert!((anim.get_eased_progress() - 0.0).abs() < 0.01);

        anim.elapsed = Duration::from_millis(1000);
        assert!((anim.get_eased_progress() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_animation_types() {
        let types = vec![
            AnimationType::FadeIn,
            AnimationType::FadeOut,
            AnimationType::ZoomIn,
            AnimationType::ZoomOut,
            AnimationType::SlideIn {
                direction: SlideDirection::Left,
            },
            AnimationType::SlideOut {
                direction: SlideDirection::Right,
            },
            AnimationType::Rotate { degrees: 360.0 },
            AnimationType::Scale {
                from: 0.0,
                to: 1.0,
            },
            AnimationType::Move {
                from: (0.0, 0.0),
                to: (100.0, 100.0),
            },
            AnimationType::ColorChange {
                from: "#000".to_string(),
                to: "#FFF".to_string(),
            },
        ];

        for anim_type in types {
            let anim = Animation::new("test".to_string(), anim_type, Duration::from_millis(500));
            assert_eq!(anim.id, "test");
        }
    }
}

// ============================================================================
// Animation Manager Tests
// ============================================================================

mod animation_manager_tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = AnimationManager::new();
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_manager_enable_disable() {
        let mut manager = AnimationManager::new();
        manager.disable();
        assert!(!manager.is_enabled());
        manager.enable();
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_manager_add_animation() {
        let mut manager = AnimationManager::new();
        let anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(500),
        );
        assert!(manager.add_animation(anim).is_ok());
        assert!(manager.get_animation("test").is_some());
    }

    #[test]
    fn test_manager_add_animation_disabled() {
        let mut manager = AnimationManager::new();
        manager.disable();
        let anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(500),
        );
        assert!(manager.add_animation(anim).is_err());
    }

    #[test]
    fn test_manager_remove_animation() {
        let mut manager = AnimationManager::new();
        let anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(500),
        );
        manager.add_animation(anim).unwrap();
        manager.remove_animation("test");
        assert!(manager.get_animation("test").is_none());
    }

    #[test]
    fn test_manager_get_animation_mut() {
        let mut manager = AnimationManager::new();
        let anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(500),
        );
        manager.add_animation(anim).unwrap();

        if let Some(anim) = manager.get_animation_mut("test") {
            anim.start();
        }

        assert_eq!(
            manager.get_animation("test").unwrap().state,
            AnimationState::Running
        );
    }

    #[test]
    fn test_manager_update() {
        let mut manager = AnimationManager::new();
        let result = manager.update(Duration::from_millis(16));
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_update_disabled() {
        let mut manager = AnimationManager::new();
        manager.disable();
        let result = manager.update(Duration::from_millis(16));
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_default() {
        let manager = AnimationManager::default();
        assert!(manager.is_enabled());
    }
}

// ============================================================================
// Timeline Tests
// ============================================================================

mod timeline_tests {
    use super::*;

    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline::new();
        assert!(!timeline.playing);
        assert_eq!(timeline.current_time, Duration::from_millis(0));
        assert_eq!(timeline.animations.len(), 0);
    }

    #[test]
    fn test_timeline_play_pause_stop() {
        let mut timeline = Timeline::new();

        timeline.play();
        assert!(timeline.playing);

        timeline.pause();
        assert!(!timeline.playing);

        timeline.play();
        timeline.stop();
        assert!(!timeline.playing);
        assert_eq!(timeline.current_time, Duration::from_millis(0));
    }

    #[test]
    fn test_timeline_add_animation() {
        let mut timeline = Timeline::new();
        let anim = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(500),
        );
        timeline.add_animation(anim);
        assert_eq!(timeline.animations.len(), 1);
    }

    #[test]
    fn test_timeline_update_when_playing() {
        let mut timeline = Timeline::new();
        timeline.play();
        timeline.update(Duration::from_millis(100)).unwrap();
        assert_eq!(timeline.current_time, Duration::from_millis(100));
    }

    #[test]
    fn test_timeline_update_when_paused() {
        let mut timeline = Timeline::new();
        // Not playing, update should be no-op
        timeline.update(Duration::from_millis(100)).unwrap();
        assert_eq!(timeline.current_time, Duration::from_millis(0));
    }

    #[test]
    fn test_timeline_default() {
        let timeline = Timeline::default();
        assert!(!timeline.playing);
    }
}

// ============================================================================
// Collaboration Tests
// ============================================================================

mod collaboration_tests {
    use super::*;

    #[test]
    fn test_collaboration_creation() {
        let collab = CanvasCollaboration::new();
        assert!(collab.is_enabled());
    }

    #[test]
    fn test_collaboration_enable_disable() {
        let mut collab = CanvasCollaboration::new();
        collab.disable();
        assert!(!collab.is_enabled());
        collab.enable();
        assert!(collab.is_enabled());
    }

    #[test]
    fn test_collaboration_default() {
        let collab = CanvasCollaboration::default();
        assert!(collab.is_enabled());
    }

    #[test]
    fn test_create_session() {
        let collab = CanvasCollaboration::new();
        let session_id = collab.create_session("canvas1".to_string()).unwrap();
        assert!(!session_id.is_empty());
    }

    #[test]
    fn test_create_session_disabled() {
        let mut collab = CanvasCollaboration::new();
        collab.disable();
        let result = collab.create_session("canvas1".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_join_session() {
        let collab = CanvasCollaboration::new();
        let session_id = collab.create_session("canvas1".to_string()).unwrap();

        let token = collab
            .join_session(session_id.clone(), "user1".to_string(), "Alice".to_string())
            .unwrap();

        assert_eq!(token.user_id, "user1");
        assert_eq!(token.session_id, session_id);
    }

    #[test]
    fn test_join_session_not_found() {
        let collab = CanvasCollaboration::new();
        let result = collab.join_session(
            "nonexistent".to_string(),
            "user1".to_string(),
            "Alice".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_join_session_duplicate_user() {
        let collab = CanvasCollaboration::new();
        let session_id = collab.create_session("canvas1".to_string()).unwrap();

        collab
            .join_session(session_id.clone(), "user1".to_string(), "Alice".to_string())
            .unwrap();

        let result = collab.join_session(
            session_id.clone(),
            "user1".to_string(),
            "Alice Again".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_leave_session() {
        let collab = CanvasCollaboration::new();
        let session_id = collab.create_session("canvas1".to_string()).unwrap();

        collab
            .join_session(session_id.clone(), "user1".to_string(), "Alice".to_string())
            .unwrap();

        let result = collab.leave_session(session_id.clone(), "user1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_active_users() {
        let collab = CanvasCollaboration::new();
        let session_id = collab.create_session("canvas1".to_string()).unwrap();

        collab
            .join_session(session_id.clone(), "user1".to_string(), "Alice".to_string())
            .unwrap();
        collab
            .join_session(session_id.clone(), "user2".to_string(), "Bob".to_string())
            .unwrap();

        let users = collab.get_active_users(session_id).unwrap();
        assert_eq!(users.len(), 2);
    }

    #[test]
    fn test_update_cursor() {
        let collab = CanvasCollaboration::new();
        let session_id = collab.create_session("canvas1".to_string()).unwrap();

        collab
            .join_session(session_id.clone(), "user1".to_string(), "Alice".to_string())
            .unwrap();

        let cursor = Cursor {
            x: 100.0,
            y: 200.0,
            slide_index: 0,
        };

        let result = collab.update_cursor(session_id, "user1".to_string(), cursor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_change() {
        let collab = CanvasCollaboration::new();
        let session_id = collab.create_session("canvas1".to_string()).unwrap();

        let shape = Shape::new(ShapeType::Rectangle);
        let change = CanvasChange::new(
            "user1".to_string(),
            "elem1".to_string(),
            CanvasChangeType::AddShape { shape },
        );

        let result = collab.apply_change(session_id, change);
        assert!(result.is_ok());
    }

    #[test]
    fn test_crdt_state() {
        let mut crdt = CRDTState::new();
        assert_eq!(crdt.version_vector.len(), 0);

        let change = CanvasChange::new(
            "user1".to_string(),
            "elem1".to_string(),
            CanvasChangeType::DeleteShape {
                shape_id: "s1".to_string(),
            },
        );

        crdt.merge(change).unwrap();
        assert_eq!(*crdt.version_vector.get("user1").unwrap(), 1);
    }

    #[test]
    fn test_crdt_multiple_users() {
        let mut crdt = CRDTState::new();

        let change1 = CanvasChange::new(
            "user1".to_string(),
            "elem1".to_string(),
            CanvasChangeType::DeleteShape {
                shape_id: "s1".to_string(),
            },
        );
        let change2 = CanvasChange::new(
            "user2".to_string(),
            "elem2".to_string(),
            CanvasChangeType::DeleteShape {
                shape_id: "s2".to_string(),
            },
        );

        crdt.merge(change1).unwrap();
        crdt.merge(change2).unwrap();

        assert_eq!(*crdt.version_vector.get("user1").unwrap(), 1);
        assert_eq!(*crdt.version_vector.get("user2").unwrap(), 1);
    }

    #[test]
    fn test_session_color_assignment() {
        let mut session = CollaborationSession::new("canvas1".to_string(), "session1".to_string());

        session
            .add_user("user1".to_string(), "Alice".to_string())
            .unwrap();
        session
            .add_user("user2".to_string(), "Bob".to_string())
            .unwrap();

        let users = session.get_active_users();
        assert_eq!(users.len(), 2);

        // Each user should have a color
        for user in &users {
            assert!(!user.color.is_empty());
        }
    }
}

// ============================================================================
// Export Tests
// ============================================================================

mod export_tests {
    use super::*;

    #[test]
    fn test_exporter_creation() {
        let exporter = CanvasExporter::new(ExportFormat::Vantis);
        assert_eq!(exporter.format, ExportFormat::Vantis);
    }

    #[test]
    fn test_export_vantis_format() {
        let mut canvas = Canvas::new("Export Test".to_string());
        canvas.add_slide();

        let temp_path = PathBuf::from("/tmp/test_vantis_export.json");
        let exporter = CanvasExporter::new(ExportFormat::Vantis);
        let result = exporter.export(&canvas, &temp_path);
        assert!(result.is_ok());

        // Verify file content is valid JSON
        let content = std::fs::read_to_string(&temp_path).unwrap();
        let parsed: Canvas = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.name, "Export Test");
        assert_eq!(parsed.slides.len(), 1);

        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_export_svg_format() {
        let mut canvas = Canvas::new("SVG Test".to_string());
        let slide = canvas.add_slide();
        let mut layer = Layer::new(0);
        layer.add_shape(
            Shape::new(ShapeType::Rectangle)
                .with_position(10.0, 10.0)
                .with_size(100.0, 50.0)
                .with_fill(Fill::Solid("#FF0000".to_string())),
        );
        layer.add_text(Text::new("Hello SVG".to_string()).with_position(20.0, 30.0));

        // We need to add the layer to the slide directly
        let mut canvas2 = Canvas::new("SVG Test".to_string());
        canvas2.add_slide();
        if let Some(slide) = canvas2.get_active_slide_mut() {
            slide.layers.push(layer);
        }

        let temp_path = PathBuf::from("/tmp/test_svg_export.svg");
        let exporter = CanvasExporter::new(ExportFormat::Svg);
        let result = exporter.export(&canvas2, &temp_path);
        assert!(result.is_ok());

        let content = std::fs::read_to_string(&temp_path).unwrap();
        assert!(content.contains("svg"));

        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_export_pdf_format() {
        let mut canvas = Canvas::new("PDF Test".to_string());
        canvas.add_slide();

        let temp_path = PathBuf::from("/tmp/test_pdf_export.pdf");
        let exporter = CanvasExporter::new(ExportFormat::Pdf);
        let result = exporter.export(&canvas, &temp_path);
        assert!(result.is_ok());

        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_export_png_format() {
        let canvas = Canvas::new("PNG Test".to_string());

        let temp_path = PathBuf::from("/tmp/test_png_export.png");
        let exporter = CanvasExporter::new(ExportFormat::Png);
        let result = exporter.export(&canvas, &temp_path);
        assert!(result.is_ok());

        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_export_powerpoint_format() {
        let mut canvas = Canvas::new("PPTX Test".to_string());
        canvas.add_slide();

        let temp_path = PathBuf::from("/tmp/test_pptx_export.pptx");
        let exporter = CanvasExporter::new(ExportFormat::Powerpoint);
        let result = exporter.export(&canvas, &temp_path);
        assert!(result.is_ok());

        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_export_to_bytes_vantis() {
        let canvas = Canvas::new("Bytes Test".to_string());
        let exporter = CanvasExporter::new(ExportFormat::Vantis);
        let result = exporter.export_to_bytes(&canvas);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_export_to_bytes_unsupported() {
        let canvas = Canvas::new("Bytes Test".to_string());
        let exporter = CanvasExporter::new(ExportFormat::Pdf);
        let result = exporter.export_to_bytes(&canvas);
        assert!(result.is_err());
    }

    #[test]
    fn test_export_formats_equality() {
        assert_eq!(ExportFormat::Vantis, ExportFormat::Vantis);
        assert_eq!(ExportFormat::Pdf, ExportFormat::Pdf);
        assert_ne!(ExportFormat::Vantis, ExportFormat::Pdf);
    }
}

// ============================================================================
// Rendering Tests
// ============================================================================

mod rendering_tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new();
        assert!(renderer.is_enabled());
    }

    #[test]
    fn test_renderer_enable_disable() {
        let mut renderer = Renderer::new();
        renderer.disable();
        assert!(!renderer.is_enabled());
        renderer.enable();
        assert!(renderer.is_enabled());
    }

    #[test]
    fn test_renderer_default() {
        let renderer = Renderer::default();
        assert!(renderer.is_enabled());
    }

    #[test]
    fn test_render_context_creation() {
        let ctx = RenderContext::new();
        assert_eq!(ctx.width, 1920.0);
        assert_eq!(ctx.height, 1080.0);
        assert_eq!(ctx.dpi, 96.0);
    }

    #[test]
    fn test_render_context_custom_dimensions() {
        let ctx = RenderContext::with_dimensions(3840.0, 2160.0);
        assert_eq!(ctx.width, 3840.0);
        assert_eq!(ctx.height, 2160.0);
    }

    #[test]
    fn test_render_context_default() {
        let ctx = RenderContext::default();
        assert_eq!(ctx.width, 1920.0);
    }

    #[test]
    fn test_render_empty_canvas() {
        let renderer = Renderer::new();
        let canvas = Canvas::new("Empty".to_string());
        let target = RenderTarget::Screen;
        let result = renderer.render(&canvas, &target);
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_canvas_with_content() {
        let renderer = Renderer::new();
        let mut canvas = Canvas::new("Content".to_string());
        canvas.add_slide();
        if let Some(slide) = canvas.get_active_slide_mut() {
            let mut layer = Layer::new(0);
            layer.add_shape(Shape::new(ShapeType::Rectangle));
            layer.add_text(Text::new("Hello".to_string()));
            layer.add_image(Image::new("test.png".to_string()));
            slide.layers.push(layer);
        }

        let target = RenderTarget::Screen;
        let result = renderer.render(&canvas, &target);
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_disabled() {
        let mut renderer = Renderer::new();
        renderer.disable();
        let canvas = Canvas::new("Test".to_string());
        let target = RenderTarget::Screen;
        let result = renderer.render(&canvas, &target);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_hidden_layer_skipped() {
        let renderer = Renderer::new();
        let mut canvas = Canvas::new("Hidden Layer".to_string());
        canvas.add_slide();
        if let Some(slide) = canvas.get_active_slide_mut() {
            let mut layer = Layer::new(0);
            layer.visible = false;
            layer.add_shape(Shape::new(ShapeType::Rectangle));
            slide.layers.push(layer);
        }

        let target = RenderTarget::Screen;
        let result = renderer.render(&canvas, &target);
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_all_shape_types() {
        let renderer = Renderer::new();
        let mut canvas = Canvas::new("All Shapes".to_string());
        canvas.add_slide();
        if let Some(slide) = canvas.get_active_slide_mut() {
            let mut layer = Layer::new(0);
            layer.add_shape(Shape::new(ShapeType::Rectangle));
            layer.add_shape(Shape::new(ShapeType::RoundedRectangle { radius: 10.0 }));
            layer.add_shape(Shape::new(ShapeType::Circle));
            layer.add_shape(Shape::new(ShapeType::Ellipse));
            layer.add_shape(Shape::new(ShapeType::Triangle));
            layer.add_shape(Shape::new(ShapeType::Polygon { sides: 6 }));
            layer.add_shape(Shape::new(ShapeType::Star {
                points: 5,
                inner_radius: 0.5,
            }));
            layer.add_shape(Shape::new(ShapeType::Line));
            layer.add_shape(Shape::new(ShapeType::Arrow));
            layer.add_shape(Shape::new(ShapeType::Path {
                points: vec![(0.0, 0.0), (50.0, 50.0)],
            }));
            layer.add_shape(Shape::new(ShapeType::Freehand {
                points: vec![(0.0, 0.0), (10.0, 10.0)],
            }));
            slide.layers.push(layer);
        }

        let target = RenderTarget::Screen;
        let result = renderer.render(&canvas, &target);
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_targets() {
        let screen = RenderTarget::Screen;
        let texture = RenderTarget::Texture {
            id: "tex1".to_string(),
        };
        let offscreen = RenderTarget::OffscreenBuffer {
            width: 1920,
            height: 1080,
        };
        let file = RenderTarget::File {
            path: "output.png".to_string(),
            format: ImageFormat::Png,
        };

        // Verify all targets can be created
        assert!(matches!(screen, RenderTarget::Screen));
        assert!(matches!(texture, RenderTarget::Texture { .. }));
        assert!(matches!(offscreen, RenderTarget::OffscreenBuffer { .. }));
        assert!(matches!(file, RenderTarget::File { .. }));
    }

    #[test]
    fn test_render_context_operations() {
        let ctx = RenderContext::new();
        assert!(ctx.apply_transform(0.0, 0.0, 100.0, 100.0, 0.0).is_ok());
        assert!(ctx.set_fill_color("#FF0000").is_ok());
        assert!(ctx
            .set_fill_gradient("#FF0000", "#0000FF", &GradientDirection::Horizontal)
            .is_ok());
        assert!(ctx.set_fill_pattern("dots", "#000").is_ok());
        assert!(ctx.set_fill_image("test.png").is_ok());
        assert!(ctx
            .set_stroke("#000", 1.0, &StrokeStyle::Solid, &None)
            .is_ok());
        assert!(ctx.apply_shadow(2.0, 2.0, 4.0, "#000").is_ok());
        assert!(ctx.apply_glow(4.0, "#FFF").is_ok());
        assert!(ctx.apply_blur(2.0).is_ok());
        assert!(ctx.apply_brightness(1.0).is_ok());
        assert!(ctx.apply_contrast(1.0).is_ok());
        assert!(ctx.apply_saturation(1.0).is_ok());
        assert!(ctx.set_opacity(0.5).is_ok());
        assert!(ctx
            .set_font(
                "Arial",
                24.0,
                &FontWeight::Normal,
                &FontStyle::Normal,
                1.2,
                0.0
            )
            .is_ok());
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

mod integration_tests {
    use super::*;

    #[test]
    fn test_full_presentation_workflow() {
        // Create canvas
        let mut canvas = Canvas::new("Full Workflow Test".to_string());

        // Add title slide
        let slide1 = canvas.add_slide();
        slide1.add_layer();
        slide1.add_text(
            Text::new("Welcome to VantisOffice".to_string())
                .with_position(960.0, 400.0)
                .with_font("Helvetica".to_string(), 72.0),
        );
        slide1.add_text(
            Text::new("Next-Gen Office Suite".to_string())
                .with_position(960.0, 500.0)
                .with_font("Helvetica".to_string(), 36.0),
        );
        slide1.transition = Some(TransitionType::Fade);

        // Add content slide
        let slide2 = canvas.add_slide();
        slide2.add_shape(
            Shape::new(ShapeType::Rectangle)
                .with_position(100.0, 100.0)
                .with_size(400.0, 300.0)
                .with_fill(Fill::Solid("#4ECDC4".to_string())),
        );
        slide2.add_text(
            Text::new("Key Features".to_string())
                .with_position(300.0, 200.0)
                .with_font("Arial".to_string(), 48.0),
        );
        slide2.transition = Some(TransitionType::SlideLeft);

        // Verify structure
        assert_eq!(canvas.slides.len(), 2);
        assert_eq!(canvas.slides[0].layers.len(), 1);
        assert!(canvas.slides[0].layers[0].texts.len() >= 2);

        // Navigate slides
        assert!(canvas.set_active_slide(1).is_ok());
        assert_eq!(canvas.active_slide, 1);

        // Export to Vantis format
        let temp_path = PathBuf::from("/tmp/test_full_workflow.json");
        let exporter = CanvasExporter::new(ExportFormat::Vantis);
        assert!(exporter.export(&canvas, &temp_path).is_ok());

        // Verify export
        let content = std::fs::read_to_string(&temp_path).unwrap();
        let loaded: Canvas = serde_json::from_str(&content).unwrap();
        assert_eq!(loaded.name, "Full Workflow Test");
        assert_eq!(loaded.slides.len(), 2);

        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_collaboration_workflow() {
        let collab = CanvasCollaboration::new();

        // Create session
        let session_id = collab.create_session("canvas1".to_string()).unwrap();

        // Users join
        let token1 = collab
            .join_session(session_id.clone(), "user1".to_string(), "Alice".to_string())
            .unwrap();
        let token2 = collab
            .join_session(session_id.clone(), "user2".to_string(), "Bob".to_string())
            .unwrap();

        // Verify users
        let users = collab.get_active_users(session_id.clone()).unwrap();
        assert_eq!(users.len(), 2);

        // Update cursors
        collab
            .update_cursor(
                session_id.clone(),
                "user1".to_string(),
                Cursor {
                    x: 100.0,
                    y: 200.0,
                    slide_index: 0,
                },
            )
            .unwrap();

        // Apply changes
        let shape = Shape::new(ShapeType::Circle)
            .with_position(50.0, 50.0)
            .with_size(100.0, 100.0);

        let change = CanvasChange::new(
            "user1".to_string(),
            "shape1".to_string(),
            CanvasChangeType::AddShape { shape },
        );

        let applied = collab.apply_change(session_id.clone(), change).unwrap();
        assert_eq!(applied.conflicts_resolved, 0);

        // User leaves
        collab
            .leave_session(session_id.clone(), "user2".to_string())
            .unwrap();
        let users = collab.get_active_users(session_id).unwrap();
        assert_eq!(users.len(), 1);
    }

    #[test]
    fn test_animation_workflow() {
        let mut manager = AnimationManager::new();

        // Create animations
        let fade_in = Animation::new(
            "title-fade".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(500),
        )
        .with_easing(EasingFunction::EaseOut);

        let slide_in = Animation::new(
            "content-slide".to_string(),
            AnimationType::SlideIn {
                direction: SlideDirection::Left,
            },
            Duration::from_millis(300),
        )
        .with_delay(Duration::from_millis(200))
        .with_easing(EasingFunction::EaseInOut);

        manager.add_animation(fade_in).unwrap();
        manager.add_animation(slide_in).unwrap();

        // Start animations
        manager.get_animation_mut("title-fade").unwrap().start();
        manager.get_animation_mut("content-slide").unwrap().start();

        // Simulate frame updates
        for _ in 0..30 {
            manager.update(Duration::from_millis(16)).unwrap();
        }

        // Check progress
        let fade = manager.get_animation("title-fade").unwrap();
        assert!(fade.get_progress() > 0.0);
    }

    #[test]
    fn test_render_and_export_workflow() {
        // Create canvas with content
        let mut canvas = Canvas::new("Render Export Test".to_string());
        canvas.add_slide();
        if let Some(slide) = canvas.get_active_slide_mut() {
            let mut layer = Layer::new(0);
            layer.add_shape(
                Shape::new(ShapeType::Rectangle)
                    .with_position(0.0, 0.0)
                    .with_size(1920.0, 1080.0)
                    .with_fill(Fill::Solid("#FFFFFF".to_string())),
            );
            layer.add_text(
                Text::new("Rendered & Exported".to_string())
                    .with_position(960.0, 540.0)
                    .with_font("Arial".to_string(), 64.0),
            );
            slide.layers.push(layer);
        }

        // Render
        let renderer = Renderer::new();
        let result = renderer.render(&canvas, &RenderTarget::Screen);
        assert!(result.is_ok());

        // Export to multiple formats
        let svg_path = PathBuf::from("/tmp/test_render_export.svg");
        let json_path = PathBuf::from("/tmp/test_render_export.json");

        let svg_exporter = CanvasExporter::new(ExportFormat::Svg);
        assert!(svg_exporter.export(&canvas, &svg_path).is_ok());

        let json_exporter = CanvasExporter::new(ExportFormat::Vantis);
        assert!(json_exporter.export(&canvas, &json_path).is_ok());

        std::fs::remove_file(&svg_path).ok();
        std::fs::remove_file(&json_path).ok();
    }
}

// ============================================================================
// Module Init Tests
// ============================================================================

mod init_tests {
    use super::*;

    #[test]
    fn test_canvas_init() {
        let result = vantis_canvas::init();
        assert!(result.is_ok());
    }

    #[test]
    fn test_canvas_version() {
        assert!(!vantis_canvas::VERSION.is_empty());
    }

    #[test]
    fn test_canvas_error_types() {
        let err = vantis_canvas::CanvasError::Rendering("test".to_string());
        assert!(format!("{}", err).contains("Rendering"));

        let err = vantis_canvas::CanvasError::Animation("test".to_string());
        assert!(format!("{}", err).contains("Animation"));

        let err = vantis_canvas::CanvasError::Export("test".to_string());
        assert!(format!("{}", err).contains("Export"));

        let err = vantis_canvas::CanvasError::Collaboration("test".to_string());
        assert!(format!("{}", err).contains("Collaboration"));

        let err = vantis_canvas::CanvasError::Serialization("test".to_string());
        assert!(format!("{}", err).contains("Serialization"));

        let err = vantis_canvas::CanvasError::General("test".to_string());
        assert!(format!("{}", err).contains("General"));

        let err: vantis_canvas::CanvasError = "string error".to_string().into();
        assert!(format!("{}", err).contains("string error"));
    }
}