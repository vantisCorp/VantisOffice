// Performance benchmarks for Vantis Ark
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use vantis_ark::core::{Backup, Recovery, InMemoryStorage, BackupManager, RecoveryStatus};
use vantis_ark::shamir::{SecretSharing, SplitConfig, RecoverConfig, PartVerifier};
use vantis_ark::StorageBackend;
use chrono::Utc;

fn benchmark_backup_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("backup_creation");
    
    let data_sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024]; // 1KB, 10KB, 100KB, 1MB
    
    for size in data_sizes {
        let size_for_closure = size;
        group.bench_with_input(BenchmarkId::from_parameter(format!("{}KB", size / 1024)), &size, |b, &_size| {
            b.iter(|| {
                let data = vec![0u8; size_for_closure];
                black_box(Backup::new("Test Backup".to_string(), data))
            })
        });
    }
    
    group.finish();
}

fn benchmark_recovery_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("recovery_creation");
    
    let data = vec![0u8; 10 * 1024];
    
    group.bench_function("create_recovery", |b| {
        b.iter(|| {
            let backup = Backup::new("Test Backup".to_string(), data.clone());
            let recovery = Recovery {
                id: "recovery-1".to_string(),
                backup_id: backup.id.clone(),
                parts: vec![],
                started_at: Utc::now(),
                completed_at: None,
                status: RecoveryStatus::InProgress,
            };
            black_box(recovery)
        })
    });
    
    group.finish();
}

fn benchmark_shamir_splitting(c: &mut Criterion) {
    let mut group = c.benchmark_group("shamir_splitting");
    
    let data_sizes = vec![1024, 10 * 1024, 100 * 1024]; // 1KB, 10KB, 100KB
    
    for size in data_sizes {
        let data = vec![b'x'; size];
        let split_config = SplitConfig::new(10, 3);
        
        group.bench_with_input(BenchmarkId::from_parameter(format!("{}KB", size / 1024)), &size, |b, &_size| {
            b.iter(|| {
                black_box(SecretSharing::split(black_box(&data), &split_config))
            })
        });
    }
    
    group.finish();
}

fn benchmark_shamir_recovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("shamir_recovery");
    
    let data = vec![b'x'; 10 * 1024]; // 10KB
    let split_config = SplitConfig::new(10, 3);
    let recover_config = RecoverConfig::new(3);
    
    let parts = SecretSharing::split(&data, &split_config);
    
    group.bench_function("recover_from_parts", |b| {
        b.iter(|| {
            black_box(SecretSharing::recover(black_box(&parts), &recover_config).unwrap())
        })
    });
    
    group.finish();
}

fn benchmark_part_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("part_verification");
    
    let part_counts = vec![1, 5, 10, 20];
    
    for count in part_counts {
        let data = vec![b'x'; 10 * 1024];
        let split_config = SplitConfig::new(count, 3);
        let parts = SecretSharing::split(&data, &split_config);
        
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &_count| {
            b.iter(|| {
                for part in &parts {
                    black_box(PartVerifier::verify(part));
                }
            })
        });
    }
    
    group.finish();
}

fn benchmark_storage_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage_operations");
    
    group.bench_function("store_data", |b| {
        b.iter(|| {
            let storage = InMemoryStorage::new();
            let data = vec![b'x'; 10 * 1024];
            black_box(storage.store("test-key", &data).unwrap())
        })
    });
    
    group.bench_function("exists_check", |b| {
        b.iter(|| {
            let storage = InMemoryStorage::new();
            black_box(storage.exists("test-key"))
        })
    });
    
    group.finish();
}

fn benchmark_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    let backup = Backup::new("Test Backup".to_string(), vec![0u8; 1024]);
    
    group.bench_function("serialize_backup", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&backup)).unwrap())
        })
    });
    
    group.bench_function("deserialize_backup", |b| {
        let json = serde_json::to_string(&backup).unwrap();
        b.iter(|| {
            black_box(serde_json::from_str::<Backup>(black_box(&json)).unwrap())
        })
    });
    
    group.finish();
}

fn benchmark_backup_manager(c: &mut Criterion) {
    let mut group = c.benchmark_group("backup_manager");
    
    group.bench_function("create_backup", |b| {
        b.iter(|| {
            let storage = Box::new(InMemoryStorage::new());
            let manager = BackupManager::new(storage);
            let data = vec![b'x'; 10 * 1024];
            black_box(manager.create_backup("Test Backup".to_string(), data).unwrap())
        })
    });
    
    group.finish();
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    
    group.bench_function("multiple_backups", |b| {
        b.iter(|| {
            let mut backups = Vec::new();
            for i in 0..10 {
                let data = vec![b'x'; 10 * 1024];
                backups.push(Backup::new(format!("Backup {}", i), data));
            }
            black_box(backups)
        })
    });
    
    group.bench_function("multiple_parts", |b| {
        b.iter(|| {
            let data = vec![b'x'; 10 * 1024];
            let config = SplitConfig::new(20, 5);
            let parts = SecretSharing::split(&data, &config);
            black_box(parts)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_backup_creation,
    benchmark_recovery_creation,
    benchmark_shamir_splitting,
    benchmark_shamir_recovery,
    benchmark_part_verification,
    benchmark_storage_operations,
    benchmark_serialization,
    benchmark_backup_manager,
    benchmark_concurrent_operations
);
criterion_main!(benches);