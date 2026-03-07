# Vantis PQC Migration Guide

## Overview

This guide helps organizations migrate from classical cryptography (RSA, ECC) to post-quantum cryptography (PQC) using the Vantis PQC module.

## Why Migrate to PQC?

### Quantum Threat Timeline

- **Current**: Classical algorithms (RSA, ECC) are secure against classical computers
- **Near-term (5-10 years)**: Quantum computers may break RSA-2048 and ECC-256
- **Long-term**: "Harvest now, decrypt later" attacks are already a threat

### Regulatory Requirements

- CNSA 2.0 (NSA) requires PQC for certain systems by 2030
- NIST PQC standards (FIPS 203, 204, 205) are finalized
- Many industries are mandating PQC adoption

## Migration Strategy

### Phase 1: Assessment (1-2 months)

1. **Inventory cryptographic assets**
   - List all RSA/ECC keys and their purposes
   - Document key sizes and algorithms used
   - Identify data protection requirements

2. **Risk assessment**
   - Classify data by sensitivity and lifetime
   - Identify "harvest now, decrypt later" targets
   - Prioritize high-value, long-lived data

3. **Compatibility check**
   - Verify system requirements (memory, CPU)
   - Test PQC library compatibility
   - Assess performance impact

### Phase 2: Planning (1 month)

1. **Algorithm selection**
   - Key encapsulation: Kyber768 (recommended) or Kyber1024
   - Signatures: Dilithium3 (recommended) or Dilithium5
   - Hybrid mode for transition period

2. **Key management planning**
   - Design key generation procedures
   - Plan key storage (HSM, software)
   - Define key rotation policies

3. **Testing environment**
   - Set up non-production test environment
   - Create test data sets
   - Define success criteria

### Phase 3: Pilot (1-2 months)

1. **Deploy in test environment**
   - Implement PQC in isolated system
   - Run full test suite
   - Measure performance impact

2. **Integration testing**
   - Test with real data (sanitized)
   - Verify interoperability
   - Document issues and resolutions

3. **Security review**
   - Conduct internal security audit
   - Fix identified vulnerabilities
   - Update documentation

### Phase 4: Production Deployment (2-3 months)

1. **Staged rollout**
   - Start with low-risk systems
   - Monitor for issues
   - Gradually expand to critical systems

2. **Parallel operation**
   - Run classical and PQC side by side
   - Verify both produce correct results
   - Build confidence in PQC

3. **Cutover**
   - Switch primary to PQC
   - Maintain classical fallback temporarily
   - Eventually remove classical support

### Phase 5: Maintenance (Ongoing)

1. **Monitoring**
   - Track key usage and rotation
   - Monitor performance metrics
   - Watch for security advisories

2. **Updates**
   - Keep libraries updated
   - Apply security patches promptly
   - Plan for algorithm upgrades if needed

## Migration Patterns

### Pattern 1: Hybrid Encryption

Use both classical and PQC for maximum security during transition:

```rust
use vantis_pqc::{hybrid_key_exchange, HybridAlgorithm};

// Hybrid key exchange provides security from both algorithms
let (shared_secret, ciphertext) = hybrid_key_exchange(
    &classical_public_key,
    &pqc_public_key,
    HybridAlgorithm::X25519Kyber768,
)?;
```

**Benefits**: 
- Security if either algorithm is broken
- Smooth transition path
- Backward compatibility

### Pattern 2: Algorithm Replacement

Direct replacement of classical with PQC:

```rust
// Before (RSA)
// let encrypted = rsa_encrypt(&public_key, data)?;

// After (Kyber)
use vantis_pqc::{KyberKeyPair, KyberSecurityLevel, encapsulate};
let kp = KyberKeyPair::generate(KyberSecurityLevel::Level2)?;
let (shared_secret, ciphertext) = encapsulate(&kp.public_key)?;
```

**Benefits**:
- Simpler implementation
- Smaller message sizes (vs hybrid)
- Better long-term solution

### Pattern 3: Signature Migration

Replace RSA/ECDSA with Dilithium:

```rust
// Before (ECDSA)
// let signature = ecdsa_sign(&private_key, message)?;

// After (Dilithium)
use vantis_pqc::{DilithiumKeyPair, DilithiumSecurityLevel, sign};
let kp = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3)?;
let signature = sign(&kp.private_key, message)?;
```

## Key Size Impact

### Storage Requirements

| Algorithm | Public Key | Private Key | Signature/Ciphertext |
|-----------|------------|-------------|---------------------|
| RSA-2048 | 256 bytes | 2.4 KB | 256 bytes |
| ECDSA P-256 | 64 bytes | 32 bytes | 64 bytes |
| Kyber768 | 1.2 KB | 2.4 KB | 1.1 KB |
| Dilithium3 | 2.0 KB | 4.0 KB | 3.3 KB |

**Impact**: Expect 5-50x larger keys, plan storage accordingly.

### Bandwidth Impact

| Operation | Classical | PQC | Increase |
|-----------|-----------|-----|----------|
| Key exchange | 64 bytes | 2.3 KB | 35x |
| Signature | 64 bytes | 3.3 KB | 50x |

**Mitigation**: Use compression where possible, plan bandwidth upgrades.

## Performance Optimization

### Batch Operations

Process multiple operations in parallel:

```rust
use vantis_pqc::BatchKyberGenerator;

// Generate 100 keypairs in parallel
let generator = BatchKyberGenerator::new(KyberSecurityLevel::Level2);
let keypairs = generator.generate_parallel(100)?;
```

### Hardware Acceleration

- Enable AVX2 if available (automatic in pqcrypto)
- Use hardware RNG for key generation
- Consider GPU acceleration for batch operations

### Caching Strategies

- Cache frequently used public keys
- Pre-generate keypairs for high-volume operations
- Use session keys to minimize KEM operations

## Key Management Migration

### Key Generation

```rust
use vantis_pqc::{KeyManager, KeyStorage, KyberSecurityLevel};

// Create key manager
let manager = KeyManager::new(KeyStorage::Database(db_connection));

// Generate PQC keys
let key_id = manager.generate_key("migration-key", KyberSecurityLevel::Level2)?;
```

### Key Rotation Policy

```rust
use vantis_pqc::{KeyRotationManager, RotationPolicy};

// Configure rotation policy
let policy = RotationPolicy {
    max_age_days: 365,      // Rotate annually
    max_encryptions: 100000, // Rotate after 100k uses
    max_decryptions: 100000,
    ..Default::default()
};

let rotation_manager = KeyRotationManager::new(policy);
```

### Migration Plan Tracking

```rust
use vantis_pqc::MigrationPlan;

// Create migration plan
let mut plan = MigrationPlan::to_kyber768();

// Track progress
plan.record_migrated("system-1");
plan.record_migrated("system-2");
println!("Migration progress: {}%", plan.progress_percentage());
```

## Compatibility Matrix

### Platform Support

| Platform | Kyber | Dilithium | Hybrid |
|----------|-------|-----------|--------|
| Linux x64 | ✅ | ✅ | ✅ |
| Linux ARM64 | ✅ | ✅ | ✅ |
| macOS x64 | ✅ | ✅ | ✅ |
| macOS ARM64 | ✅ | ✅ | ✅ |
| Windows x64 | ✅ | ✅ | ✅ |
| iOS | ✅ | ✅ | ⚠️ (X25519 limitation) |
| Android | ✅ | ✅ | ✅ |

### Language Bindings

| Language | FFI Support | Native Support |
|----------|-------------|----------------|
| C/C++ | ✅ Full | N/A |
| Python | ✅ via ctypes | Planned |
| Java | ✅ via JNI | Planned |
| Swift | ✅ | Planned |
| Kotlin | ✅ via JNI | Planned |

## Troubleshooting

### Common Migration Issues

1. **Performance degradation**
   - Enable batch operations
   - Use hardware acceleration
   - Optimize key caching

2. **Storage constraints**
   - Compress keys when storing
   - Use key references instead of full keys
   - Archive old keys

3. **Interoperability**
   - Verify algorithm parameters match
   - Check endianness compatibility
   - Validate key formats

### Rollback Plan

Always have a rollback plan:

1. Keep classical keys available
2. Document all changes
3. Test rollback procedures
4. Monitor for issues post-migration

## Compliance Checklist

- [ ] All RSA-2048 keys replaced with Kyber768 or stronger
- [ ] All ECDSA keys replaced with Dilithium3 or stronger
- [ ] Key management procedures updated
- [ ] Staff trained on PQC
- [ ] Documentation updated
- [ ] Security audit completed
- [ ] Performance testing completed
- [ ] Rollback procedures tested
- [ ] Monitoring in place

## Timeline Example

| Week | Phase | Activities |
|------|-------|------------|
| 1-4 | Assessment | Inventory, risk assessment |
| 5-8 | Planning | Algorithm selection, design |
| 9-12 | Pilot | Test deployment, integration |
| 13-20 | Deployment | Staged rollout, monitoring |
| 21-24 | Stabilization | Full production, optimization |

## Resources

- [NIST PQC Standardization](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [FIPS 203: ML-KEM](https://csrc.nist.gov/pubs/fips/203/final)
- [FIPS 204: ML-DSA](https://csrc.nist.gov/pubs/fips/204/final)
- [CNSA 2.0 Timeline](https://www.nsa.gov/Cybersecurity/Guidance/Commercial-National-Security-Algorithm-Suite/)