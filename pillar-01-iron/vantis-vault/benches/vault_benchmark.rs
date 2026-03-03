// Performance benchmarks for Vantis Vault
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use vantis_vault::api::{EncryptionProfile, KeySlot, Vault};

fn benchmark_encryption(c: &mut Criterion) {
    let vault = Vault::new().unwrap();
    let mut group = c.benchmark_group("encryption");
    
    let sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024]; // 1KB, 10KB, 100KB, 1MB
    
    for size in sizes {
        let data = vec![b'x'; size];
        
        group.bench_with_input(BenchmarkId::new("none", size), &size, |b, &_size| {
            b.iter(|| {
                black_box(vault.encrypt_document(black_box(&data), EncryptionProfile::None, KeySlot::Primary).unwrap())
            })
        });
        
        group.bench_with_input(BenchmarkId::new("software", size), &size, |b, &_size| {
            b.iter(|| {
                black_box(vault.encrypt_document(black_box(&data), EncryptionProfile::Software, KeySlot::Primary).unwrap())
            })
        });
        
        group.bench_with_input(BenchmarkId::new("tpm20", size), &size, |b, &_size| {
            b.iter(|| {
                black_box(vault.encrypt_document(black_box(&data), EncryptionProfile::TPM2_0, KeySlot::Primary).unwrap())
            })
        });
    }
    
    group.finish();
}

fn benchmark_decryption(c: &mut Criterion) {
    let vault = Vault::new().unwrap();
    let mut group = c.benchmark_group("decryption");
    
    let sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024]; // 1KB, 10KB, 100KB, 1MB
    
    for size in sizes {
        let data = vec![b'x'; size];
        let encrypted = vault.encrypt_document(&data, EncryptionProfile::Software, KeySlot::Primary).unwrap();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| {
                black_box(vault.decrypt_document(black_box(&encrypted), KeySlot::Primary).unwrap())
            })
        });
    }
    
    group.finish();
}

fn benchmark_encryption_decryption_roundtrip(c: &mut Criterion) {
    let vault = Vault::new().unwrap();
    let mut group = c.benchmark_group("roundtrip");
    
    let sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024]; // 1KB, 10KB, 100KB, 1MB
    
    for size in sizes {
        let data = vec![b'x'; size];
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| {
                let encrypted = black_box(vault.encrypt_document(black_box(&data), EncryptionProfile::Software, KeySlot::Primary).unwrap());
                black_box(vault.decrypt_document(&encrypted, KeySlot::Primary).unwrap())
            })
        });
    }
    
    group.finish();
}

fn benchmark_key_slots(c: &mut Criterion) {
    let vault = Vault::new().unwrap();
    let mut group = c.benchmark_group("key_slots");
    
    let data = vec![b'x'; 10 * 1024]; // 10KB
    
    group.bench_function("primary", |b| {
        b.iter(|| {
            let encrypted = vault.encrypt_document(&data, EncryptionProfile::Software, KeySlot::Primary).unwrap();
            black_box(vault.decrypt_document(&encrypted, KeySlot::Primary).unwrap())
        })
    });
    
    group.bench_function("backup", |b| {
        b.iter(|| {
            let encrypted = vault.encrypt_document(&data, EncryptionProfile::Software, KeySlot::Backup).unwrap();
            black_box(vault.decrypt_document(&encrypted, KeySlot::Backup).unwrap())
        })
    });
    
    group.bench_function("custom", |b| {
        b.iter(|| {
            let encrypted = vault.encrypt_document(&data, EncryptionProfile::Software, KeySlot::Custom(1)).unwrap();
            black_box(vault.decrypt_document(&encrypted, KeySlot::Custom(1)).unwrap())
        })
    });
    
    group.finish();
}

fn benchmark_multiple_documents(c: &mut Criterion) {
    let vault = Vault::new().unwrap();
    let mut group = c.benchmark_group("multiple_documents");
    
    let doc_size = 10 * 1024; // 10KB
    let data = vec![b'x'; doc_size];
    
    group.bench_function("encrypt_10_docs", |b| {
        b.iter(|| {
            for _ in 0..10 {
                black_box(vault.encrypt_document(black_box(&data), EncryptionProfile::Software, KeySlot::Primary).unwrap());
            }
        })
    });
    
    group.bench_function("encrypt_100_docs", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(vault.encrypt_document(black_box(&data), EncryptionProfile::Software, KeySlot::Primary).unwrap());
            }
        })
    });
    
    group.bench_function("encrypt_decrypt_10_docs", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let encrypted = vault.encrypt_document(&data, EncryptionProfile::Software, KeySlot::Primary).unwrap();
                black_box(vault.decrypt_document(&encrypted, KeySlot::Primary).unwrap());
            }
        })
    });
    
    group.finish();
}

fn benchmark_vault_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("vault_operations");
    
    group.bench_function("create_vault", |b| {
        b.iter(|| {
            black_box(Vault::new().unwrap())
        })
    });
    
    group.bench_function("default_vault", |b| {
        b.iter(|| {
            black_box(Vault::default())
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_encryption,
    benchmark_decryption,
    benchmark_encryption_decryption_roundtrip,
    benchmark_key_slots,
    benchmark_multiple_documents,
    benchmark_vault_operations
);
criterion_main!(benches);