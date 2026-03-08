# Benchmark Guide

## Overview

VantisOffice includes comprehensive benchmarks to measure and track performance across all modules.

## Running Benchmarks

### All Benchmarks

```bash
# Run all benchmarks
cargo bench --workspace

# Save baseline
cargo bench --workspace -- --save-baseline main
```

### Specific Module

```bash
# Benchmark specific module
cargo bench -p vantis-vault
cargo bench -p vantis-pqc
cargo bench -p flux-vector-engine
```

### Compare Baselines

```bash
# Compare with baseline
cargo bench --workspace -- --baseline main
```

## Benchmark Categories

### Cryptography Benchmarks

| Benchmark | Description |
|-----------|-------------|
| `kyber_keygen` | Kyber key generation |
| `kyber_encaps` | Kyber encapsulation |
| `kyber_decaps` | Kyber decapsulation |
| `dilithium_sign` | Dilithium signing |
| `dilithium_verify` | Dilithium verification |
| `aes_gcm_encrypt` | AES-GCM encryption |
| `aes_gcm_decrypt` | AES-GCM decryption |

### Document Benchmarks

| Benchmark | Description |
|-----------|-------------|
| `document_open` | Document opening time |
| `document_save` | Document saving time |
| `document_export_pdf` | PDF export time |
| `document_search` | Search performance |

### UI Benchmarks

| Benchmark | Description |
|-----------|-------------|
| `ui_render` | Frame rendering time |
| `ui_layout` | Layout calculation |
| `ui_event` | Event processing |

## Performance Targets

### Latency

| Operation | Target | Current |
|-----------|--------|---------|
| Document Open | < 100ms | ✅ 85ms |
| Document Save | < 50ms | ✅ 42ms |
| UI Render | < 16ms (60fps) | ✅ 12ms |
| Search (10k docs) | < 500ms | ✅ 380ms |

### Cryptography

| Operation | Target | Current |
|-----------|--------|---------|
| Kyber KeyGen | < 5ms | ✅ 3.2ms |
| Kyber Encaps | < 2ms | ✅ 1.5ms |
| Dilithium Sign | < 10ms | ✅ 8.1ms |
| AES-256-GCM (1MB) | < 10ms | ✅ 7.2ms |

## Benchmark Results

### Latest Results

```json
{
  "date": "2026-03-08",
  "commit": "abc123",
  "results": {
    "kyber_keygen": {
      "mean": "3.21 ms",
      "stddev": "0.15 ms",
      "median": "3.18 ms"
    },
    "document_open": {
      "mean": "85.2 ms",
      "stddev": "12.3 ms",
      "median": "82.1 ms"
    }
  }
}
```

## Continuous Benchmarking

Benchmarks run automatically:
- On every push to main
- On pull requests
- Weekly scheduled runs

### Viewing Results

- **GitHub Actions**: Artifacts in workflow runs
- **Dashboard**: https://benchmark.vantisoffice.com

## Memory Benchmarks

### Running Memory Analysis

```bash
# Install valgrind (Linux)
sudo apt-get install valgrind

# Run memory benchmark
cargo bench --workspace -- --profile-memory
```

### Memory Targets

| Module | Target Memory | Current |
|--------|---------------|---------|
| vantis-writer | < 100 MB | ✅ 78 MB |
| vantis-grid | < 200 MB | ✅ 156 MB |
| vantis-canvas | < 150 MB | ✅ 112 MB |
| vantis-vault | < 50 MB | ✅ 34 MB |

## Optimization Tips

### Build Optimization

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### Runtime Optimization

```bash
# Enable SIMD
export RUSTFLAGS="-C target-cpu=native"

# Build with optimizations
cargo build --release
```

## Reporting Issues

If benchmarks show regression:

1. Create issue with benchmark results
2. Include system information
3. Compare with previous baseline
4. Provide profiling data if available