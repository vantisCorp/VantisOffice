//! Animation system

use std::time::Duration;
use serde::{Deserialize, Serialize};

/// Animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub duration: Duration,
    pub easing: EasingFunction,
    pub keyframes: Vec<(f32, f32)>,
}

impl Animation {
    /// Create a new animation
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            easing: EasingFunction::Linear,
            keyframes: Vec::new(),
        }
    }
    
    /// Set easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }
    
    /// Set keyframes
    pub fn with_keyframes(mut self, keyframes: Vec<(f32, f32)>) -> Self {
        self.keyframes = keyframes;
        self
    }
    
    /// Evaluate animation at time t
    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        let eased_t = self.easing.apply(t);
        
        if self.keyframes.is_empty() {
            return eased_t;
        }
        
        // Interpolate between keyframes
        let total_keyframes = self.keyframes.len() - 1;
        let segment = (eased_t * total_keyframes as f32).floor() as usize;
        let segment_t = (eased_t * total_keyframes as f32) - segment as f32;
        
        if segment >= total_keyframes {
            return self.keyframes[total_keyframes].1;
        }
        
        let (start_val, end_val) = (self.keyframes[segment].1, self.keyframes[segment + 1].1);
        start_val + (end_val - start_val) * segment_t
    }
}

/// Easing function
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
    CubicBezier(f32, f32, f32, f32),
}

impl EasingFunction {
    /// Apply easing function
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            EasingFunction::EaseInQuad => t * t,
            EasingFunction::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            EasingFunction::EaseInCubic => t * t * t,
            EasingFunction::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            EasingFunction::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            EasingFunction::EaseInQuart => t * t * t * t,
            EasingFunction::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
            EasingFunction::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
                }
            }
            EasingFunction::EaseInQuint => t * t * t * t * t,
            EasingFunction::EaseOutQuint => 1.0 - (1.0 - t).powi(5),
            EasingFunction::EaseInOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
                }
            }
            EasingFunction::EaseInSine => 1.0 - (t * std::f32::consts::PI / 2.0).cos(),
            EasingFunction::EaseOutSine => (t * std::f32::consts::PI / 2.0).sin(),
            EasingFunction::EaseInOutSine => {
                -(std::f32::consts::PI).cos() - 1.0 / 2.0
            }
            EasingFunction::EaseInExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    2.0f32.powf(10.0 * t - 10.0)
                }
            }
            EasingFunction::EaseOutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0f32.powf(-10.0 * t)
                }
            }
            EasingFunction::EaseInOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    2.0f32.powf(20.0 * t - 10.0) / 2.0
                } else {
                    (2.0 - 2.0f32.powf(-20.0 * t + 10.0)) / 2.0
                }
            }
            EasingFunction::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            EasingFunction::EaseOutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
            EasingFunction::EaseInOutCirc => {
                if t < 0.5 {
                    (1.0 - (2.0 * t).powi(2)).sqrt() / 2.0
                } else {
                    (1.0 + (-2.0 * t + 2.0).powi(2)).sqrt() / 2.0
                }
            }
            EasingFunction::EaseInElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    -(2.0f32.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * (2.0 * std::f32::consts::PI / 3.0)).sin()
                }
            }
            EasingFunction::EaseOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    2.0f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * (2.0 * std::f32::consts::PI / 3.0)).sin() + 1.0
                }
            }
            EasingFunction::EaseInOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else if t < 0.5 {
                    -(2.0f32.powf(20.0 * t - 10.0)) * ((t * 20.0 - 11.125) * (2.0 * std::f32::consts::PI / 4.5)).sin() / 2.0
                } else {
                    2.0f32.powf(-20.0 * t + 10.0) * ((t * 20.0 - 11.125) * (2.0 * std::f32::consts::PI / 4.5)).sin() / 2.0 + 1.0
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
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            EasingFunction::EaseInBounce => 1.0 - EasingFunction::EaseOutBounce.apply(1.0 - t),
            EasingFunction::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            EasingFunction::EaseInOutBounce => {
                if t < 0.5 {
                    (1.0 - EasingFunction::EaseOutBounce.apply(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + EasingFunction::EaseOutBounce.apply(2.0 * t - 1.0)) / 2.0
                }
            }
            EasingFunction::CubicBezier(x1, y1, x2, y2) => {
                // Simplified cubic bezier implementation
                let t2 = t * t;
                let t3 = t2 * t;
                let mt = 1.0 - t;
                let mt2 = mt * mt;
                let mt3 = mt2 * mt;
                3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3
            }
        }
    }
}

/// Animation engine
pub struct AnimationEngine {
    animations: Vec<Animation>,
}

impl AnimationEngine {
    /// Create a new animation engine
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
        }
    }
    
    /// Add an animation
    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }
    
    /// Update animations
    pub fn update(&mut self, _dt: f32) {
        // Update animation states
    }
    
    /// Get animations
    pub fn animations(&self) -> &[Animation] {
        &self.animations
    }
}

impl Default for AnimationEngine {
    fn default() -> Self {
        Self::new()
    }
}