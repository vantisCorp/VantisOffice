//! Neural Engine for AI-powered spreadsheet features
//!
//! Provides trend prediction, anomaly detection, and intelligent suggestions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Neural Engine for AI-powered features
pub struct NeuralEngine {
    models: HashMap<String, PredictionModel>,
    enabled: bool,
}

impl NeuralEngine {
    pub fn new() -> Self {
        NeuralEngine {
            models: HashMap::new(),
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

    /// Train a prediction model on data
    pub fn train_model(&mut self, name: String, data: &[f64]) -> Result<(), String> {
        if !self.enabled {
            return Err("Neural engine is disabled".to_string());
        }

        let model = PredictionModel::train(data)?;
        self.models.insert(name, model);
        Ok(())
    }

    /// Predict next value in a sequence
    pub fn predict(&self, model_name: &str, context: &[f64]) -> Result<f64, String> {
        if !self.enabled {
            return Err("Neural engine is disabled".to_string());
        }

        let model = self
            .models
            .get(model_name)
            .ok_or_else(|| format!("Model '{}' not found", model_name))?;

        model.predict(context)
    }

    /// Analyze trends in data
    pub fn analyze_trends(&self, data: &[f64]) -> Result<TrendAnalysis, String> {
        if !self.enabled {
            return Err("Neural engine is disabled".to_string());
        }

        if data.len() < 2 {
            return Err("Insufficient data for trend analysis".to_string());
        }

        // Calculate linear regression
        let n = data.len() as f64;
        let sum_x: f64 = (0..data.len()).map(|i| i as f64).sum();
        let sum_y: f64 = data.iter().sum();
        let sum_xy: f64 = data.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
        let sum_x2: f64 = (0..data.len()).map(|i| (i as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate R-squared
        let mean_y = sum_y / n;
        let ss_tot: f64 = data.iter().map(|&y| (y - mean_y).powi(2)).sum();
        let ss_res: f64 = data
            .iter()
            .enumerate()
            .map(|(i, &y)| {
                let predicted = slope * i as f64 + intercept;
                (y - predicted).powi(2)
            })
            .sum();

        let r_squared = if ss_tot > 0.0 {
            1.0 - (ss_res / ss_tot)
        } else {
            0.0
        };

        // Determine trend direction
        let trend_direction = if slope.abs() < 0.01 {
            TrendDirection::Stable
        } else if slope > 0.0 {
            TrendDirection::Increasing
        } else {
            TrendDirection::Decreasing
        };

        // Detect anomalies
        let anomalies = self.detect_anomalies(data, slope, intercept)?;

        Ok(TrendAnalysis {
            slope,
            intercept,
            r_squared,
            trend_direction,
            confidence: r_squared,
            anomalies,
            data_points: data.len(),
        })
    }

    /// Detect anomalies in data
    fn detect_anomalies(
        &self,
        data: &[f64],
        slope: f64,
        intercept: f64,
    ) -> Result<Vec<Anomaly>, String> {
        let mut anomalies = Vec::new();

        // Calculate residuals and standard deviation
        let residuals: Vec<f64> = data
            .iter()
            .enumerate()
            .map(|(i, &y)| {
                let predicted = slope * i as f64 + intercept;
                y - predicted
            })
            .collect();

        let mean_residual = residuals.iter().sum::<f64>() / residuals.len() as f64;
        let std_dev = (residuals
            .iter()
            .map(|&r| (r - mean_residual).powi(2))
            .sum::<f64>()
            / residuals.len() as f64)
            .sqrt();

        // Flag points more than 2 standard deviations away
        let threshold = 2.0 * std_dev;

        for (i, &residual) in residuals.iter().enumerate() {
            if residual.abs() > threshold {
                anomalies.push(Anomaly {
                    index: i,
                    value: data[i],
                    expected: slope * i as f64 + intercept,
                    deviation: residual,
                    severity: if residual.abs() > 3.0 * std_dev {
                        AnomalySeverity::High
                    } else {
                        AnomalySeverity::Medium
                    },
                });
            }
        }

        Ok(anomalies)
    }

    /// Generate intelligent suggestions
    pub fn generate_suggestions(&self, data: &[f64]) -> Result<Vec<Suggestion>, String> {
        if !self.enabled {
            return Err("Neural engine is disabled".to_string());
        }

        let mut suggestions = Vec::new();

        // Analyze trends
        if let Ok(trend) = self.analyze_trends(data) {
            match trend.trend_direction {
                TrendDirection::Increasing => {
                    suggestions.push(Suggestion {
                        suggestion_type: SuggestionType::Trend,
                        message:
                            "Data shows an increasing trend. Consider forecasting future values."
                                .to_string(),
                        confidence: trend.confidence,
                    });
                }
                TrendDirection::Decreasing => {
                    suggestions.push(Suggestion {
                        suggestion_type: SuggestionType::Trend,
                        message: "Data shows a decreasing trend. Investigate potential causes."
                            .to_string(),
                        confidence: trend.confidence,
                    });
                }
                TrendDirection::Stable => {
                    suggestions.push(Suggestion {
                        suggestion_type: SuggestionType::Trend,
                        message: "Data is stable. No significant trend detected.".to_string(),
                        confidence: trend.confidence,
                    });
                }
            }

            // Anomaly suggestions
            if !trend.anomalies.is_empty() {
                suggestions.push(Suggestion {
                    suggestion_type: SuggestionType::Anomaly,
                    message: format!("Detected {} anomalies in the data. Review these points for potential errors.", trend.anomalies.len()),
                    confidence: 0.8,
                });
            }
        }

        // Pattern detection
        if data.len() >= 3 {
            if self.detect_seasonality(data) {
                suggestions.push(Suggestion {
                    suggestion_type: SuggestionType::Pattern,
                    message:
                        "Seasonal pattern detected. Consider using seasonal forecasting methods."
                            .to_string(),
                    confidence: 0.7,
                });
            }
        }

        Ok(suggestions)
    }

    /// Detect seasonality in data
    fn detect_seasonality(&self, data: &[f64]) -> bool {
        if data.len() < 4 {
            return false;
        }

        // Simple autocorrelation check
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;

        if variance == 0.0 {
            return false;
        }

        // Check for periodic patterns
        for period in 2..=(data.len() / 2) {
            let mut correlation = 0.0;
            let count = data.len() - period;

            for i in 0..count {
                correlation += (data[i] - mean) * (data[i + period] - mean);
            }

            correlation /= (count as f64) * variance;

            if correlation > 0.7 {
                return true;
            }
        }

        false
    }
}

impl Default for NeuralEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Prediction model for forecasting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionModel {
    pub model_type: ModelType,
    pub coefficients: Vec<f64>,
    pub accuracy: f64,
    pub trained_on: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    Linear,
    Polynomial { degree: usize },
    Exponential,
    MovingAverage { window: usize },
}

impl PredictionModel {
    pub fn train(data: &[f64]) -> Result<Self, String> {
        if data.len() < 2 {
            return Err("Insufficient data for training".to_string());
        }

        // Simple linear regression model
        let n = data.len() as f64;
        let sum_x: f64 = (0..data.len()).map(|i| i as f64).sum();
        let sum_y: f64 = data.iter().sum();
        let sum_xy: f64 = data.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
        let sum_x2: f64 = (0..data.len()).map(|i| (i as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate accuracy (R-squared)
        let mean_y = sum_y / n;
        let ss_tot: f64 = data.iter().map(|&y| (y - mean_y).powi(2)).sum();
        let ss_res: f64 = data
            .iter()
            .enumerate()
            .map(|(i, &y)| {
                let predicted = slope * i as f64 + intercept;
                (y - predicted).powi(2)
            })
            .sum();

        let accuracy = if ss_tot > 0.0 {
            1.0 - (ss_res / ss_tot)
        } else {
            0.0
        };

        Ok(PredictionModel {
            model_type: ModelType::Linear,
            coefficients: vec![intercept, slope],
            accuracy,
            trained_on: data.len(),
        })
    }

    pub fn predict(&self, context: &[f64]) -> Result<f64, String> {
        match &self.model_type {
            ModelType::Linear => {
                if self.coefficients.len() >= 2 {
                    let intercept = self.coefficients[0];
                    let slope = self.coefficients[1];
                    let x = context.last().copied().unwrap_or(0.0);
                    Ok(intercept + slope * x)
                } else {
                    Err("Invalid model coefficients".to_string())
                }
            }
            ModelType::Polynomial { degree } => {
                if self.coefficients.len() >= *degree + 1 {
                    let x = context.last().copied().unwrap_or(0.0);
                    let mut result = 0.0;
                    for (i, &coeff) in self.coefficients.iter().enumerate() {
                        result += coeff * x.powi(i as i32);
                    }
                    Ok(result)
                } else {
                    Err("Invalid model coefficients".to_string())
                }
            }
            ModelType::Exponential => {
                if self.coefficients.len() >= 2 {
                    let a = self.coefficients[0];
                    let b = self.coefficients[1];
                    let x = context.last().copied().unwrap_or(0.0);
                    Ok(a * b.powf(x))
                } else {
                    Err("Invalid model coefficients".to_string())
                }
            }
            ModelType::MovingAverage { window } => {
                if context.len() >= *window {
                    let sum: f64 = context.iter().rev().take(*window).sum();
                    Ok(sum / *window as f64)
                } else {
                    let sum: f64 = context.iter().sum();
                    Ok(sum / context.len() as f64)
                }
            }
        }
    }
}

/// Trend analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub slope: f64,
    pub intercept: f64,
    pub r_squared: f64,
    pub trend_direction: TrendDirection,
    pub confidence: f64,
    pub anomalies: Vec<Anomaly>,
    pub data_points: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub index: usize,
    pub value: f64,
    pub expected: f64,
    pub deviation: f64,
    pub severity: AnomalySeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
}

/// Intelligent suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub suggestion_type: SuggestionType,
    pub message: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    Trend,
    Anomaly,
    Pattern,
    Formula,
    DataQuality,
}

/// Initialize neural engine
pub fn init() -> Result<(), String> {
    // Initialize ML libraries and models
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neural_engine_creation() {
        let engine = NeuralEngine::new();
        assert!(engine.is_enabled());
    }

    #[test]
    fn test_trend_analysis() {
        let engine = NeuralEngine::new();
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let trend = engine.analyze_trends(&data).unwrap();
        assert!(matches!(trend.trend_direction, TrendDirection::Increasing));
        assert!(trend.r_squared > 0.9);
    }

    #[test]
    fn test_prediction_model() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let model = PredictionModel::train(&data).unwrap();

        let prediction = model.predict(&[5.0]).unwrap();
        assert!((prediction - 6.0).abs() < 0.1);
    }

    #[test]
    fn test_anomaly_detection() {
        let engine = NeuralEngine::new();
        let data = vec![1.0, 2.0, 3.0, 4.0, 100.0, 6.0, 7.0]; // 100.0 is an anomaly

        let trend = engine.analyze_trends(&data).unwrap();
        assert!(!trend.anomalies.is_empty());
        assert!(trend.anomalies.iter().any(|a| a.value == 100.0));
    }
}
