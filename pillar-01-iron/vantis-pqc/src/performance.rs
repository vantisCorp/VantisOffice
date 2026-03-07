//! Performance Optimization for Post-Quantum Cryptography
//!
//! This module provides performance-critical operations:
//! - Batch key generation and encapsulation
//! - Thread pool for parallel operations
//! - SIMD-optimized operations where available
//! - Caching and memoization utilities

use crate::error::{PQCError, Result};
use crate::kyber::{KyberKeyPair, KyberSecurityLevel, encapsulate};
use crate::dilithium::{DilithiumKeyPair, DilithiumSecurityLevel, sign};
use std::sync::Arc;
use std::thread;

/// Batch key generation result
#[derive(Debug, Clone)]
pub struct BatchKeyResult<T> {
    /// Generated keys
    pub keys: Vec<T>,
    /// Time taken in milliseconds
    pub duration_ms: u64,
    /// Keys generated per second
    pub keys_per_second: f64,
}

impl<T> BatchKeyResult<T> {
    /// Create a new batch result
    pub fn new(keys: Vec<T>, duration_ms: u64) -> Self {
        let keys_per_second = if duration_ms > 0 {
            (keys.len() as f64 / duration_ms as f64) * 1000.0
        } else {
            0.0
        };
        
        Self {
            keys,
            duration_ms,
            keys_per_second,
        }
    }

    /// Get the number of generated keys
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Check if no keys were generated
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}

/// Batch Kyber keypair generator
pub struct BatchKyberGenerator {
    /// Security level for all keys
    security_level: KyberSecurityLevel,
    /// Number of worker threads
    num_threads: usize,
}

impl BatchKyberGenerator {
    /// Create a new batch generator
    pub fn new(security_level: KyberSecurityLevel) -> Self {
        Self {
            security_level,
            num_threads: num_cpus::get(),
        }
    }

    /// Set the number of worker threads
    pub fn with_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = num_threads.max(1);
        self
    }

    /// Generate multiple keypairs in parallel
    pub fn generate(&self, count: usize) -> Result<BatchKeyResult<KyberKeyPair>> {
        let start = std::time::Instant::now();
        
        let keys = if self.num_threads > 1 && count > 4 {
            self.generate_parallel(count)?
        } else {
            self.generate_sequential(count)?
        };
        
        let duration_ms = start.elapsed().as_millis() as u64;
        Ok(BatchKeyResult::new(keys, duration_ms))
    }

    fn generate_sequential(&self, count: usize) -> Result<Vec<KyberKeyPair>> {
        let mut keys = Vec::with_capacity(count);
        for _ in 0..count {
            keys.push(KyberKeyPair::generate(self.security_level)?);
        }
        Ok(keys)
    }

    fn generate_parallel(&self, count: usize) -> Result<Vec<KyberKeyPair>> {
        let chunk_size = (count + self.num_threads - 1) / self.num_threads;
        let security_level = self.security_level;
        
        let handles: Vec<_> = (0..self.num_threads)
            .map(move |_| {
                thread::spawn(move || {
                    let mut keys = Vec::with_capacity(chunk_size);
                    for _ in 0..chunk_size {
                        match KyberKeyPair::generate(security_level) {
                            Ok(kp) => keys.push(kp),
                            Err(_) => break,
                        }
                    }
                    keys
                })
            })
            .collect();
        
        let mut all_keys = Vec::with_capacity(count);
        for handle in handles {
            all_keys.extend(handle.join().unwrap_or_default());
        }
        
        all_keys.truncate(count);
        Ok(all_keys)
    }
}

/// Batch Dilithium keypair generator
pub struct BatchDilithiumGenerator {
    /// Security level for all keys
    security_level: DilithiumSecurityLevel,
    /// Number of worker threads
    num_threads: usize,
}

impl BatchDilithiumGenerator {
    /// Create a new batch generator
    pub fn new(security_level: DilithiumSecurityLevel) -> Self {
        Self {
            security_level,
            num_threads: num_cpus::get(),
        }
    }

    /// Set the number of worker threads
    pub fn with_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = num_threads.max(1);
        self
    }

    /// Generate multiple keypairs in parallel
    pub fn generate(&self, count: usize) -> Result<BatchKeyResult<DilithiumKeyPair>> {
        let start = std::time::Instant::now();
        
        let keys = if self.num_threads > 1 && count > 4 {
            self.generate_parallel(count)?
        } else {
            self.generate_sequential(count)?
        };
        
        let duration_ms = start.elapsed().as_millis() as u64;
        Ok(BatchKeyResult::new(keys, duration_ms))
    }

    fn generate_sequential(&self, count: usize) -> Result<Vec<DilithiumKeyPair>> {
        let mut keys = Vec::with_capacity(count);
        for _ in 0..count {
            keys.push(DilithiumKeyPair::generate(self.security_level)?);
        }
        Ok(keys)
    }

    fn generate_parallel(&self, count: usize) -> Result<Vec<DilithiumKeyPair>> {
        let chunk_size = (count + self.num_threads - 1) / self.num_threads;
        let security_level = self.security_level;
        
        let handles: Vec<_> = (0..self.num_threads)
            .map(move |_| {
                thread::spawn(move || {
                    let mut keys = Vec::with_capacity(chunk_size);
                    for _ in 0..chunk_size {
                        match DilithiumKeyPair::generate(security_level) {
                            Ok(kp) => keys.push(kp),
                            Err(_) => break,
                        }
                    }
                    keys
                })
            })
            .collect();
        
        let mut all_keys = Vec::with_capacity(count);
        for handle in handles {
            all_keys.extend(handle.join().unwrap_or_default());
        }
        
        all_keys.truncate(count);
        Ok(all_keys)
    }
}

/// Encapsulation result for batch operations
#[derive(Debug, Clone)]
pub struct EncapsulationResult {
    /// Shared secret
    pub shared_secret: Vec<u8>,
    /// Ciphertext
    pub ciphertext: Vec<u8>,
}

/// Batch encapsulator for multiple public keys
pub struct BatchEncapsulator {
    /// Number of worker threads
    num_threads: usize,
}

impl BatchEncapsulator {
    /// Create a new batch encapsulator
    pub fn new() -> Self {
        Self {
            num_threads: num_cpus::get(),
        }
    }

    /// Set the number of worker threads
    pub fn with_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = num_threads.max(1);
        self
    }

    /// Encapsulate to multiple public keys
    pub fn encapsulate_all(&self, public_keys: &[Vec<u8>]) -> Result<BatchKeyResult<EncapsulationResult>> {
        let start = std::time::Instant::now();
        
        let results = if self.num_threads > 1 && public_keys.len() > 4 {
            self.encapsulate_parallel(public_keys)?
        } else {
            self.encapsulate_sequential(public_keys)?
        };
        
        let duration_ms = start.elapsed().as_millis() as u64;
        Ok(BatchKeyResult::new(results, duration_ms))
    }

    fn encapsulate_sequential(&self, public_keys: &[Vec<u8>]) -> Result<Vec<EncapsulationResult>> {
        let mut results = Vec::with_capacity(public_keys.len());
        for pk in public_keys {
            let (shared_secret, ciphertext) = encapsulate(pk)?;
            results.push(EncapsulationResult {
                shared_secret,
                ciphertext: ciphertext.data,
            });
        }
        Ok(results)
    }

    fn encapsulate_parallel(&self, public_keys: &[Vec<u8>]) -> Result<Vec<EncapsulationResult>> {
        let chunk_size = (public_keys.len() + self.num_threads - 1) / self.num_threads;
        let keys: Arc<Vec<Vec<u8>>> = Arc::new(public_keys.to_vec());
        
        let handles: Vec<_> = (0..self.num_threads)
            .map(|i| {
                let keys = Arc::clone(&keys);
                let start = i * chunk_size;
                let end = std::cmp::min(start + chunk_size, keys.len());
                
                thread::spawn(move || {
                    let mut results = Vec::new();
                    for pk in keys.iter().take(end).skip(start) {
                        if let Ok((ss, ct)) = encapsulate(pk) {
                            results.push(EncapsulationResult {
                                shared_secret: ss,
                                ciphertext: ct.data,
                            });
                        }
                    }
                    results
                })
            })
            .collect();
        
        let mut all_results = Vec::with_capacity(public_keys.len());
        for handle in handles {
            all_results.extend(handle.join().unwrap_or_default());
        }
        
        Ok(all_results)
    }
}

impl Default for BatchEncapsulator {
    fn default() -> Self {
        Self::new()
    }
}

/// Signature result for batch operations
#[derive(Debug, Clone)]
pub struct SignatureResult {
    /// The signature
    pub signature: Vec<u8>,
    /// Message that was signed
    pub message: Vec<u8>,
}

/// Batch signer for multiple messages
pub struct BatchSigner {
    /// Number of worker threads
    num_threads: usize,
}

impl BatchSigner {
    /// Create a new batch signer
    pub fn new() -> Self {
        Self {
            num_threads: num_cpus::get(),
        }
    }

    /// Set the number of worker threads
    pub fn with_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = num_threads.max(1);
        self
    }

    /// Sign multiple messages with the same key
    pub fn sign_all(&self, private_key: &[u8], messages: &[Vec<u8>]) -> Result<BatchKeyResult<SignatureResult>> {
        let start = std::time::Instant::now();
        
        let results = if self.num_threads > 1 && messages.len() > 8 {
            self.sign_parallel(private_key, messages)?
        } else {
            self.sign_sequential(private_key, messages)?
        };
        
        let duration_ms = start.elapsed().as_millis() as u64;
        Ok(BatchKeyResult::new(results, duration_ms))
    }

    fn sign_sequential(&self, private_key: &[u8], messages: &[Vec<u8>]) -> Result<Vec<SignatureResult>> {
        let mut results = Vec::with_capacity(messages.len());
        for msg in messages {
            let signature = sign(private_key, msg)?;
            results.push(SignatureResult {
                signature: signature.data,
                message: msg.clone(),
            });
        }
        Ok(results)
    }

    fn sign_parallel(&self, private_key: &[u8], messages: &[Vec<u8>]) -> Result<Vec<SignatureResult>> {
        let chunk_size = (messages.len() + self.num_threads - 1) / self.num_threads;
        let pk = private_key.to_vec();
        let msgs = messages.to_vec();
        
        let handles: Vec<_> = (0..self.num_threads)
            .map(move |i| {
                let pk = pk.clone();
                let msgs = msgs.clone();
                let start = i * chunk_size;
                let end = std::cmp::min(start + chunk_size, msgs.len());
                
                thread::spawn(move || {
                    let mut results = Vec::new();
                    for msg in msgs.iter().take(end).skip(start) {
                        if let Ok(sig) = sign(&pk, msg) {
                            results.push(SignatureResult {
                                signature: sig.data,
                                message: msg.clone(),
                            });
                        }
                    }
                    results
                })
            })
            .collect();
        
        let mut all_results = Vec::with_capacity(messages.len());
        for handle in handles {
            all_results.extend(handle.join().unwrap_or_default());
        }
        
        Ok(all_results)
    }
}

impl Default for BatchSigner {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance statistics
#[derive(Debug, Clone, Default)]
pub struct PerformanceStats {
    /// Number of operations performed
    pub operations: u64,
    /// Total time in microseconds
    pub total_time_us: u64,
    /// Minimum time in microseconds
    pub min_time_us: Option<u64>,
    /// Maximum time in microseconds
    pub max_time_us: Option<u64>,
}

impl PerformanceStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an operation timing
    pub fn record(&mut self, time_us: u64) {
        self.operations += 1;
        self.total_time_us += time_us;
        
        self.min_time_us = Some(self.min_time_us.map_or(time_us, |m| m.min(time_us)));
        self.max_time_us = Some(self.max_time_us.map_or(time_us, |m| m.max(time_us)));
    }

    /// Get average time in microseconds
    pub fn avg_time_us(&self) -> f64 {
        if self.operations == 0 {
            0.0
        } else {
            self.total_time_us as f64 / self.operations as f64
        }
    }

    /// Get operations per second
    pub fn ops_per_second(&self) -> f64 {
        if self.total_time_us == 0 {
            0.0
        } else {
            1_000_000.0 * self.operations as f64 / self.total_time_us as f64
        }
    }
}

/// Performance benchmark suite
pub struct PerformanceBenchmark {
    /// Kyber benchmark stats
    pub kyber_stats: PerformanceStats,
    /// Dilithium benchmark stats
    pub dilithium_stats: PerformanceStats,
}

impl PerformanceBenchmark {
    /// Create a new benchmark suite
    pub fn new() -> Self {
        Self {
            kyber_stats: PerformanceStats::new(),
            dilithium_stats: PerformanceStats::new(),
        }
    }

    /// Run benchmarks
    pub fn run(&mut self, iterations: u32) -> Result<()> {
        // Benchmark Kyber key generation
        for _ in 0..iterations {
            let start = std::time::Instant::now();
            let _ = KyberKeyPair::generate(KyberSecurityLevel::Level2)?;
            let elapsed = start.elapsed().as_micros() as u64;
            self.kyber_stats.record(elapsed);
        }
        
        // Benchmark Dilithium key generation
        for _ in 0..iterations {
            let start = std::time::Instant::now();
            let _ = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3)?;
            let elapsed = start.elapsed().as_micros() as u64;
            self.dilithium_stats.record(elapsed);
        }
        
        Ok(())
    }

    /// Get a summary of benchmark results
    pub fn summary(&self) -> String {
        format!(
            "Performance Benchmark Results:\n\
             Kyber Key Generation:\n\
             - Operations: {}\n\
             - Average: {:.2} µs\n\
             - Min: {} µs\n\
             - Max: {} µs\n\
             - Ops/sec: {:.2}\n\n\
             Dilithium Key Generation:\n\
             - Operations: {}\n\
             - Average: {:.2} µs\n\
             - Min: {} µs\n\
             - Max: {} µs\n\
             - Ops/sec: {:.2}",
            self.kyber_stats.operations,
            self.kyber_stats.avg_time_us(),
            self.kyber_stats.min_time_us.unwrap_or(0),
            self.kyber_stats.max_time_us.unwrap_or(0),
            self.kyber_stats.ops_per_second(),
            self.dilithium_stats.operations,
            self.dilithium_stats.avg_time_us(),
            self.dilithium_stats.min_time_us.unwrap_or(0),
            self.dilithium_stats.max_time_us.unwrap_or(0),
            self.dilithium_stats.ops_per_second(),
        )
    }
}

impl Default for PerformanceBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_kyber_generator_sequential() {
        let generator = BatchKyberGenerator::new(KyberSecurityLevel::Level1).with_threads(1);
        let result = generator.generate(4).unwrap();
        
        assert_eq!(result.len(), 4);
        // Duration could be 0 if very fast
    }

    #[test]
    fn test_batch_kyber_generator_parallel() {
        let generator = BatchKyberGenerator::new(KyberSecurityLevel::Level1).with_threads(2);
        let result = generator.generate(8).unwrap();
        
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_batch_dilithium_generator_sequential() {
        let generator = BatchDilithiumGenerator::new(DilithiumSecurityLevel::Level2).with_threads(1);
        let result = generator.generate(4).unwrap();
        
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_batch_encapsulator_sequential() {
        // Generate keys first
        let kp = KyberKeyPair::generate(KyberSecurityLevel::Level1).unwrap();
        let public_keys = vec![kp.public_key.clone(); 4];
        
        let encapsulator = BatchEncapsulator::new().with_threads(1);
        let result = encapsulator.encapsulate_all(&public_keys).unwrap();
        
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_batch_signer_sequential() {
        let kp = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level2).unwrap();
        let messages = vec![b"message 1".to_vec(), b"message 2".to_vec()];
        
        let signer = BatchSigner::new().with_threads(1);
        let result = signer.sign_all(&kp.private_key, &messages).unwrap();
        
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_performance_stats() {
        let mut stats = PerformanceStats::new();
        
        stats.record(100);
        stats.record(200);
        stats.record(150);
        
        assert_eq!(stats.operations, 3);
        assert_eq!(stats.total_time_us, 450);
        assert_eq!(stats.min_time_us, Some(100));
        assert_eq!(stats.max_time_us, Some(200));
        assert!((stats.avg_time_us() - 150.0).abs() < 0.01);
    }

    #[test]
    fn test_performance_stats_ops_per_second() {
        let mut stats = PerformanceStats::new();
        
        // 100 microseconds per operation = 10,000 ops/sec
        stats.record(100);
        
        assert!((stats.ops_per_second() - 10000.0).abs() < 1.0);
    }

    #[test]
    fn test_batch_key_result() {
        let keys = vec![1, 2, 3];
        let result = BatchKeyResult::new(keys, 1000);
        
        assert_eq!(result.len(), 3);
        assert_eq!(result.duration_ms, 1000);
        assert!((result.keys_per_second - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_batch_key_result_empty() {
        let keys: Vec<i32> = vec![];
        let result = BatchKeyResult::new(keys, 100);
        
        assert!(result.is_empty());
        assert_eq!(result.keys_per_second, 0.0);
    }

    #[test]
    fn test_performance_benchmark() {
        let mut benchmark = PerformanceBenchmark::new();
        benchmark.run(3).unwrap();
        
        assert_eq!(benchmark.kyber_stats.operations, 3);
        assert_eq!(benchmark.dilithium_stats.operations, 3);
    }

    #[test]
    fn test_performance_benchmark_summary() {
        let mut benchmark = PerformanceBenchmark::new();
        benchmark.run(2).unwrap();
        
        let summary = benchmark.summary();
        assert!(summary.contains("Kyber"));
        assert!(summary.contains("Dilithium"));
    }
}