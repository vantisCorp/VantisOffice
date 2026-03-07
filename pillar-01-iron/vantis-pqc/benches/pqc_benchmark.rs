//! Performance benchmarks for Vantis PQC
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import the library
use vantis_pqc::{
    KyberKeyPair, KyberSecurityLevel, encapsulate, decapsulate,
    DilithiumKeyPair, DilithiumSecurityLevel, sign, verify,
    hybrid_key_exchange, HybridAlgorithm,
    derive_keys_from_shared_secret,
};

fn kyber_keygen_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("kyber_keygen");

    for (level, name) in [
        (KyberSecurityLevel::Level1, "Kyber512"),
        (KyberSecurityLevel::Level2, "Kyber768"),
        (KyberSecurityLevel::Level3, "Kyber1024"),
    ] {
        group.bench_function(name, |b| {
            b.iter(|| KyberKeyPair::generate(black_box(level.clone())).unwrap())
        });
    }

    group.finish();
}

fn kyber_encapsulate_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("kyber_encapsulate");

    for (level, name) in [
        (KyberSecurityLevel::Level1, "Kyber512"),
        (KyberSecurityLevel::Level2, "Kyber768"),
        (KyberSecurityLevel::Level3, "Kyber1024"),
    ] {
        let kp = KyberKeyPair::generate(level.clone()).unwrap();
        let pk = kp.public_key.clone();

        group.bench_function(name, |b| {
            b.iter(|| encapsulate(black_box(&pk)).unwrap())
        });
    }

    group.finish();
}

fn kyber_decapsulate_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("kyber_decapsulate");

    for (level, name) in [
        (KyberSecurityLevel::Level1, "Kyber512"),
        (KyberSecurityLevel::Level2, "Kyber768"),
        (KyberSecurityLevel::Level3, "Kyber1024"),
    ] {
        let kp = KyberKeyPair::generate(level.clone()).unwrap();
        let (_, ct) = encapsulate(&kp.public_key).unwrap();

        group.bench_function(name, |b| {
            b.iter(|| decapsulate(black_box(&kp.private_key), black_box(&ct.data)).unwrap())
        });
    }

    group.finish();
}

fn dilithium_keygen_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dilithium_keygen");

    for (level, name) in [
        (DilithiumSecurityLevel::Level2, "Dilithium2"),
        (DilithiumSecurityLevel::Level3, "Dilithium3"),
        (DilithiumSecurityLevel::Level5, "Dilithium5"),
    ] {
        group.bench_function(name, |b| {
            b.iter(|| DilithiumKeyPair::generate(black_box(level.clone())).unwrap())
        });
    }

    group.finish();
}

fn dilithium_sign_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dilithium_sign");

    let message = b"Test message for benchmarking Dilithium signature";

    for (level, name) in [
        (DilithiumSecurityLevel::Level2, "Dilithium2"),
        (DilithiumSecurityLevel::Level3, "Dilithium3"),
        (DilithiumSecurityLevel::Level5, "Dilithium5"),
    ] {
        let kp = DilithiumKeyPair::generate(level.clone()).unwrap();

        group.bench_function(name, |b| {
            b.iter(|| sign(black_box(&kp.private_key), black_box(message)).unwrap())
        });
    }

    group.finish();
}

fn dilithium_verify_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dilithium_verify");

    let message = b"Test message for benchmarking Dilithium signature";

    for (level, name) in [
        (DilithiumSecurityLevel::Level2, "Dilithium2"),
        (DilithiumSecurityLevel::Level3, "Dilithium3"),
        (DilithiumSecurityLevel::Level5, "Dilithium5"),
    ] {
        let kp = DilithiumKeyPair::generate(level.clone()).unwrap();
        let sig = sign(&kp.private_key, message).unwrap();

        group.bench_function(name, |b| {
            b.iter(|| verify(black_box(&kp.public_key), black_box(message), black_box(&sig.data)).unwrap())
        });
    }

    group.finish();
}

fn hybrid_key_exchange_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("hybrid_key_exchange");

    // Generate keys for hybrid
    let kyber_kp = KyberKeyPair::generate(KyberSecurityLevel::Level2).unwrap();

    // For X25519, we'd need a real key - using placeholder
    let classical_pk = vec![0u8; 32];

    group.bench_function("X25519_Kyber768", |b| {
        b.iter(|| {
            hybrid_key_exchange(
                black_box(&classical_pk),
                black_box(&kyber_kp.public_key),
                black_box(HybridAlgorithm::X25519Kyber768),
            )
        })
    });

    group.finish();
}

fn kdf_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("kdf");

    let shared_secret = vec![0u8; 32];

    group.bench_function("derive_keys_from_shared_secret", |b| {
        b.iter(|| {
            derive_keys_from_shared_secret(
                black_box(&shared_secret),
                black_box("benchmark_context"),
                black_box(2),
                black_box(32),
            )
        })
    });

    group.finish();
}

fn full_kem_roundtrip_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_kem_roundtrip");

    // Pre-generate keys
    let kp512 = KyberKeyPair::generate(KyberSecurityLevel::Level1).unwrap();
    let kp768 = KyberKeyPair::generate(KyberSecurityLevel::Level2).unwrap();
    let kp1024 = KyberKeyPair::generate(KyberSecurityLevel::Level3).unwrap();

    group.bench_function("Kyber512_full", |b| {
        b.iter(|| {
            let (_, ct) = encapsulate(&kp512.public_key).unwrap();
            decapsulate(&kp512.private_key, &ct.data).unwrap()
        })
    });

    group.bench_function("Kyber768_full", |b| {
        b.iter(|| {
            let (_, ct) = encapsulate(&kp768.public_key).unwrap();
            decapsulate(&kp768.private_key, &ct.data).unwrap()
        })
    });

    group.bench_function("Kyber1024_full", |b| {
        b.iter(|| {
            let (_, ct) = encapsulate(&kp1024.public_key).unwrap();
            decapsulate(&kp1024.private_key, &ct.data).unwrap()
        })
    });

    group.finish();
}

fn full_signature_roundtrip_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_signature_roundtrip");

    let message = b"Benchmark message";

    // Pre-generate keys
    let kp2 = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level2).unwrap();
    let kp3 = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level3).unwrap();
    let kp5 = DilithiumKeyPair::generate(DilithiumSecurityLevel::Level5).unwrap();

    group.bench_function("Dilithium2_full", |b| {
        b.iter(|| {
            let sig = sign(&kp2.private_key, message).unwrap();
            verify(&kp2.public_key, message, &sig.data).unwrap()
        })
    });

    group.bench_function("Dilithium3_full", |b| {
        b.iter(|| {
            let sig = sign(&kp3.private_key, message).unwrap();
            verify(&kp3.public_key, message, &sig.data).unwrap()
        })
    });

    group.bench_function("Dilithium5_full", |b| {
        b.iter(|| {
            let sig = sign(&kp5.private_key, message).unwrap();
            verify(&kp5.public_key, message, &sig.data).unwrap()
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    kyber_keygen_benchmark,
    kyber_encapsulate_benchmark,
    kyber_decapsulate_benchmark,
    dilithium_keygen_benchmark,
    dilithium_sign_benchmark,
    dilithium_verify_benchmark,
    hybrid_key_exchange_benchmark,
    kdf_benchmark,
    full_kem_roundtrip_benchmark,
    full_signature_roundtrip_benchmark,
);

criterion_main!(benches);