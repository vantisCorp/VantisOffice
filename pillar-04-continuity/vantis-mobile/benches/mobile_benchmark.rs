//! Performance benchmarks for Vantis Mobile
//!
//! Benchmarks cover:
//! - Cryptographic operations (encryption, decryption, key generation, hashing)
//! - Model serialization/deserialization
//! - Protocol message construction and encoding

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use vantis_mobile::crypto::*;
use vantis_mobile::models::*;
use vantis_mobile::protocol::*;
use uuid::Uuid;

// ============================================================================
// Cryptographic Benchmarks
// ============================================================================

fn bench_key_generation(c: &mut Criterion) {
    c.bench_function("EncryptionKey::generate", |b| {
        b.iter(|| {
            let key = EncryptionKey::generate().unwrap();
            black_box(key);
        })
    });
}

fn bench_keypair_generation(c: &mut Criterion) {
    c.bench_function("KeyPair::generate", |b| {
        b.iter(|| {
            let kp = KeyPair::generate().unwrap();
            black_box(kp);
        })
    });
}

fn bench_encryption(c: &mut Criterion) {
    let key = EncryptionKey::generate().unwrap();
    let encryptor = Encryptor::new(&key).unwrap();

    let mut group = c.benchmark_group("encryption");

    for size in [64, 256, 1024, 4096, 16384, 65536].iter() {
        let data = vec![0xABu8; *size];
        group.bench_with_input(
            BenchmarkId::new("encrypt", size),
            size,
            |b, _| {
                b.iter(|| {
                    let encrypted = encryptor.encrypt(black_box(&data)).unwrap();
                    black_box(encrypted);
                })
            },
        );
    }

    group.finish();
}

fn bench_decryption(c: &mut Criterion) {
    let key = EncryptionKey::generate().unwrap();
    let encryptor = Encryptor::new(&key).unwrap();

    let mut group = c.benchmark_group("decryption");

    for size in [64, 256, 1024, 4096, 16384, 65536].iter() {
        let data = vec![0xABu8; *size];
        let encrypted = encryptor.encrypt(&data).unwrap();

        group.bench_with_input(
            BenchmarkId::new("decrypt", size),
            size,
            |b, _| {
                b.iter(|| {
                    let decrypted = encryptor.decrypt(black_box(&encrypted)).unwrap();
                    black_box(decrypted);
                })
            },
        );
    }

    group.finish();
}

fn bench_encrypt_decrypt_roundtrip(c: &mut Criterion) {
    let key = EncryptionKey::generate().unwrap();
    let encryptor = Encryptor::new(&key).unwrap();

    let mut group = c.benchmark_group("encrypt_decrypt_roundtrip");

    for size in [256, 1024, 4096].iter() {
        let data = vec![0xCDu8; *size];
        group.bench_with_input(
            BenchmarkId::new("roundtrip", size),
            size,
            |b, _| {
                b.iter(|| {
                    let encrypted = encryptor.encrypt(black_box(&data)).unwrap();
                    let decrypted = encryptor.decrypt(&encrypted).unwrap();
                    black_box(decrypted);
                })
            },
        );
    }

    group.finish();
}

fn bench_sha256(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256");

    for size in [64, 256, 1024, 4096, 65536].iter() {
        let data = vec![0xEFu8; *size];
        group.bench_with_input(
            BenchmarkId::new("hash", size),
            size,
            |b, _| {
                b.iter(|| {
                    let hash = sha256(black_box(&data));
                    black_box(hash);
                })
            },
        );
    }

    group.finish();
}

fn bench_sha512(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha512");

    for size in [64, 256, 1024, 4096, 65536].iter() {
        let data = vec![0xEFu8; *size];
        group.bench_with_input(
            BenchmarkId::new("hash", size),
            size,
            |b, _| {
                b.iter(|| {
                    let hash = sha512(black_box(&data));
                    black_box(hash);
                })
            },
        );
    }

    group.finish();
}

fn bench_random_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_bytes");

    for size in [16, 32, 64, 256, 1024].iter() {
        group.bench_with_input(
            BenchmarkId::new("generate", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let bytes = random_bytes(black_box(size)).unwrap();
                    black_box(bytes);
                })
            },
        );
    }

    group.finish();
}

fn bench_key_base64_roundtrip(c: &mut Criterion) {
    let key = EncryptionKey::generate().unwrap();

    c.bench_function("key_base64_roundtrip", |b| {
        b.iter(|| {
            let encoded = key.to_base64();
            let decoded = EncryptionKey::from_base64(black_box(&encoded)).unwrap();
            black_box(decoded);
        })
    });
}

// ============================================================================
// Model Serialization Benchmarks
// ============================================================================

fn bench_device_info_serialization(c: &mut Criterion) {
    let device = DeviceInfo::new(
        DeviceType::Ios,
        "iPhone 15 Pro".to_string(),
        "17.2".to_string(),
        "1.0.0".to_string(),
    );

    c.bench_function("DeviceInfo::serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&device)).unwrap();
            black_box(json);
        })
    });
}

fn bench_device_info_deserialization(c: &mut Criterion) {
    let device = DeviceInfo::new(
        DeviceType::Ios,
        "iPhone 15 Pro".to_string(),
        "17.2".to_string(),
        "1.0.0".to_string(),
    );
    let json = serde_json::to_string(&device).unwrap();

    c.bench_function("DeviceInfo::deserialize", |b| {
        b.iter(|| {
            let parsed: DeviceInfo = serde_json::from_str(black_box(&json)).unwrap();
            black_box(parsed);
        })
    });
}

fn bench_document_metadata_serialization(c: &mut Criterion) {
    let doc = DocumentMetadata::new(
        DocumentType::Writer,
        "Quarterly Report Q4 2026".to_string(),
        Uuid::new_v4(),
    );

    c.bench_function("DocumentMetadata::serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&doc)).unwrap();
            black_box(json);
        })
    });
}

fn bench_document_metadata_deserialization(c: &mut Criterion) {
    let doc = DocumentMetadata::new(
        DocumentType::Writer,
        "Quarterly Report Q4 2026".to_string(),
        Uuid::new_v4(),
    );
    let json = serde_json::to_string(&doc).unwrap();

    c.bench_function("DocumentMetadata::deserialize", |b| {
        b.iter(|| {
            let parsed: DocumentMetadata = serde_json::from_str(black_box(&json)).unwrap();
            black_box(parsed);
        })
    });
}

fn bench_notification_serialization(c: &mut Criterion) {
    let notification = Notification::new(
        NotificationType::DocumentShared,
        "New Document Shared".to_string(),
        "Alice shared 'Budget 2026' with you".to_string(),
        NotificationPriority::High,
    );

    c.bench_function("Notification::serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&notification)).unwrap();
            black_box(json);
        })
    });
}

fn bench_sync_progress_update(c: &mut Criterion) {
    c.bench_function("SyncProgress::update", |b| {
        b.iter(|| {
            let mut progress = SyncProgress::new(1000);
            for i in 0..100 {
                progress.update(black_box(i * 10), black_box(i as u64 * 1024));
            }
            black_box(progress);
        })
    });
}

// ============================================================================
// Protocol Message Benchmarks
// ============================================================================

fn bench_protocol_message_handshake(c: &mut Criterion) {
    c.bench_function("ProtocolMessage::handshake", |b| {
        b.iter(|| {
            let msg = ProtocolMessage::handshake(
                black_box(Uuid::new_v4()),
                black_box(DeviceType::Android),
                black_box("Pixel 8".to_string()),
            );
            black_box(msg);
        })
    });
}

fn bench_protocol_message_ping_pong(c: &mut Criterion) {
    c.bench_function("ProtocolMessage::ping", |b| {
        b.iter(|| {
            let msg = ProtocolMessage::ping();
            black_box(msg);
        })
    });

    c.bench_function("ProtocolMessage::pong", |b| {
        b.iter(|| {
            let msg = ProtocolMessage::pong();
            black_box(msg);
        })
    });
}

fn bench_protocol_message_serialization(c: &mut Criterion) {
    let msg = ProtocolMessage::handshake(
        Uuid::new_v4(),
        DeviceType::Ios,
        "iPhone 15".to_string(),
    );

    c.bench_function("ProtocolMessage::serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&msg)).unwrap();
            black_box(json);
        })
    });
}

fn bench_protocol_message_deserialization(c: &mut Criterion) {
    let msg = ProtocolMessage::handshake(
        Uuid::new_v4(),
        DeviceType::Ios,
        "iPhone 15".to_string(),
    );
    let json = serde_json::to_string(&msg).unwrap();

    c.bench_function("ProtocolMessage::deserialize", |b| {
        b.iter(|| {
            let parsed: ProtocolMessage = serde_json::from_str(black_box(&json)).unwrap();
            black_box(parsed);
        })
    });
}

fn bench_tunnel_config_creation(c: &mut Criterion) {
    c.bench_function("TunnelConfig::new", |b| {
        b.iter(|| {
            let config = TunnelConfig::new(
                black_box("wss://tunnel.vantis.ai".to_string()),
                black_box(Uuid::new_v4()),
                black_box(&[0u8; 32]),
            )
            .with_device_type(DeviceType::Android)
            .with_device_name("Pixel 8 Pro".to_string())
            .with_os_version("14.0".to_string())
            .with_app_version("1.0.0".to_string());
            black_box(config);
        })
    });
}

fn bench_remote_command_creation(c: &mut Criterion) {
    c.bench_function("RemoteCommand::new", |b| {
        b.iter(|| {
            let cmd = RemoteCommand::new(
                black_box(AppType::Writer),
                black_box("open_document".to_string()),
                black_box(serde_json::json!({
                    "document_id": Uuid::new_v4().to_string(),
                    "read_only": false
                })),
            );
            black_box(cmd);
        })
    });
}

fn bench_command_response_creation(c: &mut Criterion) {
    let cmd_id = Uuid::new_v4();

    c.bench_function("CommandResponse::success", |b| {
        b.iter(|| {
            let resp = CommandResponse::success(
                black_box(cmd_id),
                black_box(Some(serde_json::json!({"status": "opened"}))),
            );
            black_box(resp);
        })
    });

    c.bench_function("CommandResponse::error", |b| {
        b.iter(|| {
            let resp = CommandResponse::error(
                black_box(cmd_id),
                black_box("Document not found".to_string()),
            );
            black_box(resp);
        })
    });
}

// ============================================================================
// Criterion Groups
// ============================================================================

criterion_group!(
    crypto_benches,
    bench_key_generation,
    bench_keypair_generation,
    bench_encryption,
    bench_decryption,
    bench_encrypt_decrypt_roundtrip,
    bench_sha256,
    bench_sha512,
    bench_random_bytes,
    bench_key_base64_roundtrip,
);

criterion_group!(
    model_benches,
    bench_device_info_serialization,
    bench_device_info_deserialization,
    bench_document_metadata_serialization,
    bench_document_metadata_deserialization,
    bench_notification_serialization,
    bench_sync_progress_update,
);

criterion_group!(
    protocol_benches,
    bench_protocol_message_handshake,
    bench_protocol_message_ping_pong,
    bench_protocol_message_serialization,
    bench_protocol_message_deserialization,
    bench_tunnel_config_creation,
    bench_remote_command_creation,
    bench_command_response_creation,
);

criterion_main!(crypto_benches, model_benches, protocol_benches);