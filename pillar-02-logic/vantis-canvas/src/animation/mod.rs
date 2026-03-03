//! Animation module for transitions and effects
//! 
//! Provides smooth animations and transitions for presentations

use std::collections::HashMap;
use std::time::Duration;

/// Animation manager
pub struct AnimationManager {
    animations: HashMap<String, Animation>,
    timeline: Timeline,
    enabled: bool,
}

impl AnimationManager {
    pub fn new() -> Self {
        AnimationManager {
            animations: HashMap::new(),
            timeline: Timeline::new(),
            enabled: true,
        }
    }
    
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Add an animation
    pub fn add_animation(&mut self, animation: Animation) -> Result<(), String> {
        if !self.enabled {
            return Err("Animation manager is disabled".to_string());
        }
        
        let id = animation.id.clone();
        self.animations.insert(id, animation);
        Ok(())
    }
    
    /// Remove an animation
    pub fn remove_animation(&mut self, id: &str) {
        self.animations.remove(id);
    }
    
    /// Update animations
    pub fn update(&mut self, delta_time: Duration) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        
        for animation in self.animations.values_mut() {
            animation.update(delta_time)?;
        }
        
        self.timeline.update(delta_time)?;
        
        Ok(())
    }
    
    /// Get animation by ID
    pub fn get_animation(&self, id: &str) -> Option<&Animation> {
        self.animations.get(id)
    }
    
    /// Get animation by ID (mutable)
    pub fn get_animation_mut(&mut self, id: &str) -> Option<&mut Animation> {
        self.animations.get_mut(id)
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation
#[derive(Debug, Clone)]
pub struct Animation {
    pub id: String,
    pub animation_type: AnimationType,
    pub duration: Duration,
    pub easing: EasingFunction,
    pub delay: Duration,
    pub repeat: RepeatMode,
    pub state: AnimationState,
    pub elapsed: Duration,
}

#[derive(Debug, Clone)]
pub enum AnimationType {
    FadeIn,
    FadeOut,
    SlideIn { direction: SlideDirection },
    SlideOut { direction: SlideDirection },
    ZoomIn,
    ZoomOut,
    Rotate { degrees: f64 },
    Scale { from: f64, to: f64 },
    Move { from: (f64, f64), to: (f64, f64) },
    ColorChange { from: String, to: String },
}

#[derive(Debug, Clone)]
pub enum SlideDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
}

#[derive(Debug, Clone)]
pub enum RepeatMode {
    Once,
    Loop,
    Repeat { count: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationState {
    Idle,
    Running,
    Paused,
    Completed,
    Cancelled,
}

impl Animation {
    pub fn new(id: String, animation_type: AnimationType, duration: Duration) -> Self {
        Animation {
            id,
            animation_type,
            duration,
            easing: EasingFunction::Linear,
            delay: Duration::from_millis(0),
            repeat: RepeatMode::Once,
            state: AnimationState::Idle,
            elapsed: Duration::from_millis(0),
        }
    }
    
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }
    
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }
    
    pub fn with_repeat(mut self, repeat: RepeatMode) -> Self {
        self.repeat = repeat;
        self
    }
    
    pub fn start(&mut self) {
        self.state = AnimationState::Running;
        self.elapsed = Duration::from_millis(0);
    }
    
    pub fn pause(&mut self) {
        if self.state == AnimationState::Running {
            self.state = AnimationState::Paused;
        }
    }
    
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Running;
        }
    }
    
    pub fn stop(&mut self) {
        self.state = AnimationState::Cancelled;
    }
    
    pub fn update(&mut self, delta_time: Duration) -> Result<(), String> {
        if self.state != AnimationState::Running {
            return Ok(());
        }
        
        self.elapsed += delta_time;
        
        if self.elapsed >= self.duration {
            match self.repeat {
                RepeatMode::Once => {
                    self.state = AnimationState::Completed;
                }
                RepeatMode::Loop => {
                    self.elapsed = Duration::from_millis(0);
                }
                RepeatMode::Repeat { count } => {
                    // Handle repeat count
                    self.elapsed = Duration::from_millis(0);
                }
            }
        }
        
        Ok(())
    }
    
    pub fn get_progress(&self) -> f64 {
        if self.duration.as_millis() == 0 {
            return 1.0;
        }
        
        let progress = self.elapsed.as_secs_f64() / self.duration.as_secs_f64();
        progress.min(1.0).max(0.0)
    }
    
    pub fn get_eased_progress(&self) -> f64 {
        let progress = self.get_progress();
        self.apply_easing(progress)
    }
    
    fn apply_easing(&self, t: f64) -> f64 {
        match self.easing {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => t * (2.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingFunction::EaseInQuad => t * t,
            EasingFunction::EaseOutQuad => t * (2.0 - t),
            EasingFunction::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingFunction::EaseInCubic => t * t * t,
            EasingFunction::EaseOutCubic => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            EasingFunction::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t1 = t - 1.0;
                    4.0 * t1 * t1 * t1 + 1.0
                }
            }
            EasingFunction::EaseInQuart => t * t * t * t,
            EasingFunction::EaseOutQuart => {
                let t1 = t - 1.0;
                1.0 - t1 * t1 * t1 * t1
            }
            EasingFunction::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let t1 = t - 1.0;
                    1.0 - 8.0 * t1 * t1 * t1 * t1
                }
            }
            EasingFunction::EaseInQuint => t * t * t * t * t,
            EasingFunction::EaseOutQuint => {
                let t1 = t - 1.0;
                t1 * t1 * t1 * t1 * t1 + 1.0
            }
            EasingFunction::EaseInOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    let t1 = t - 1.0;
                    16.0 * t1 * t1 * t1 * t1 * t1 + 1.0
                }
            }
            EasingFunction::EaseInSine => 1.0 - (t * std::f64::consts::PI / 2.0).cos(),
            EasingFunction::EaseOutSine => (t * std::f64::consts::PI / 2.0).sin(),
            EasingFunction::EaseInOutSine => {
                -(std::f64::consts::PI * t).cos() / 2.0 + 0.5
            }
            EasingFunction::EaseInExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    2.0_f64.powf(10.0 * (t - 1.0))
                }
            }
            EasingFunction::EaseOutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f64.powf(-10.0 * t)
                }
            }
            EasingFunction::EaseInOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    2.0_f64.powf(10.0 * (2.0 * t - 1.0)) / 2.0
                } else {
                    (2.0 - 2.0_f64.powf(-10.0 * (2.0 * t - 1.0))) / 2.0
                }
            }
            EasingFunction::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            EasingFunction::EaseOutCirc => (1.0 - (t - 1.0) * (t - 1.0)).sqrt(),
            EasingFunction::EaseInOutCirc => {
                if t < 0.5 {
                    (1.0 - (1.0 - 2.0 * t * 2.0 * t).sqrt()) / 2.0
                } else {
                    (1.0 + (1.0 - 2.0 * (t - 1.0) * 2.0 * (t - 1.0)).sqrt()) / 2.0
                }
            }
            EasingFunction::EaseInElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    -2.0_f64.powf(10.0 * (t - 1.0)) * ((t - 1.1) * 2.0 * std::f64::consts::PI).sin()
                }
            }
            EasingFunction::EaseOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    2.0_f64.powf(-10.0 * t) * ((t - 0.1) * 2.0 * std::f64::consts::PI).sin() + 1.0
                }
            }
            EasingFunction::EaseInOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else if t < 0.5 {
                    -0.5 * 2.0_f64.powf(10.0 * (2.0 * t - 1.0)) * ((2.0 * t - 1.1) * 2.0 * std::f64::consts::PI).sin()
                } else {
                    0.5 * 2.0_f64.powf(-10.0 * (2.0 * t - 1.0)) * ((2.0 * t - 1.1) * 2.0 * std::f64::consts::PI).sin() + 1.0
                }
            }
            EasingFunction::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            EasingFunction::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            EasingFunction::EaseInOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    (2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            EasingFunction::EaseInBounce => 1.0 - Self::bounce_out(1.0 - t),
            EasingFunction::EaseOutBounce => Self::bounce_out(t),
            EasingFunction::EaseInOutBounce => {
                if t < 0.5 {
                    (1.0 - Self::bounce_out(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + Self::bounce_out(2.0 * t - 1.0)) / 2.0
                }
            }
        }
    }
    
    fn bounce_out(t: f64) -> f64 {
        const N1: f64 = 7.5625;
        const D1: f64 = 2.75;
        
        if t < 1.0 / D1 {
            N1 * t * t
        } else if t < 2.0 / D1 {
            let t = t - 1.5 / D1;
            N1 * t * t + 0.75
        } else if t < 2.5 / D1 {
            let t = t - 2.25 / D1;
            N1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / D1;
            N1 * t * t + 0.984375
        }
    }
}

/// Transition between slides
#[derive(Debug, Clone)]
pub struct Transition {
    pub transition_type: crate::core::TransitionType,
    pub duration: Duration,
    pub easing: EasingFunction,
}

impl Transition {
    pub fn new(transition_type: crate::core::TransitionType, duration: Duration) -> Self {
        Transition {
            transition_type,
            duration,
            easing: EasingFunction::EaseInOut,
        }
    }
}

/// Timeline for managing animations
#[derive(Debug, Clone)]
pub struct Timeline {
    pub animations: Vec<Animation>,
    pub current_time: Duration,
    pub playing: bool,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            animations: Vec::new(),
            current_time: Duration::from_millis(0),
            playing: false,
        }
    }
    
    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }
    
    pub fn play(&mut self) {
        self.playing = true;
    }
    
    pub fn pause(&mut self) {
        self.playing = false;
    }
    
    pub fn stop(&mut self) {
        self.playing = false;
        self.current_time = Duration::from_millis(0);
    }
    
    pub fn update(&mut self, delta_time: Duration) -> Result<(), String> {
        if !self.playing {
            return Ok(());
        }
        
        self.current_time += delta_time;
        
        for animation in &mut self.animations {
            animation.update(delta_time)?;
        }
        
        Ok(())
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize animation module
pub fn init() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_animation_creation() {
        let mut animation = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000)
        );
        assert_eq!(animation.id, "test");
    }
    
    #[test]
    fn test_animation_progress() {
        let mut animation = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000)
        );
        animation.start();
        animation.elapsed = Duration::from_millis(500);
        
        let progress = animation.get_progress();
        assert!((progress - 0.5).abs() < 0.01);
    }
    
    #[test]
    fn test_easing_functions() {
        let mut animation = Animation::new(
            "test".to_string(),
            AnimationType::FadeIn,
            Duration::from_millis(1000)
        );
        animation.elapsed = Duration::from_millis(500);
        
        let linear = animation.apply_easing(0.5);
        assert!((linear - 0.5).abs() < 0.01);
        
        let eased = animation.with_easing(EasingFunction::EaseIn).apply_easing(0.5);
        assert!(eased <= 0.5);
    }
    
    #[test]
    fn test_timeline() {
        let mut timeline = Timeline::new();
        assert!(!timeline.playing);
        
        timeline.play();
        assert!(timeline.playing);
        
        timeline.pause();
        assert!(!timeline.playing);
    }
}