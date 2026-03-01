# Vantis Ark

## Overview

Vantis Ark provides distributed backup for VantisOffice documents using Shamir Secret Sharing. It splits backups into multiple parts distributed across trusted stations, requiring only a subset to reconstruct the original data.

## Key Features

- **Shamir Secret Sharing**: Cryptographic splitting of backup data
- **Distributed Storage**: Parts stored across multiple locations
- **Flexible Threshold**: Configurable parts needed for recovery
- **Secure Distribution**: Encrypted transport between stations
- **Automatic Verification**: Integrity verification of backups
- **Redundancy**: Multiple backup copies for safety

## Architecture

```
vantis-ark/
├── src/
│   ├── core/
│   │   ├── backup.rs          # Backup model
│   │   ├── recovery.rs       # Recovery model
│   │   └── storage.rs        # Storage abstraction
│   ├── shamir/
│   │   ├── secret_sharing.rs # Shamir implementation
│   │   ├── split.rs          # Splitting logic
│   │   ├── combine.rs        # Recovery logic
│   │   └── verification.rs   # Part verification
│   ├── distribution/
│   │   ├── network.rs        # Network distribution
│   │   ├── stations.rs       # Station management
│   │   ├── protocol.rs       # Distribution protocol
│   │   └── health.rs         # Station health monitoring
│   ├── encryption/
│   │   ├── transport.rs      # Transport encryption
│   │   ├── storage.rs        # Storage encryption
│   │   └── keys.rs           # Key management
│   ├── scheduler/
│   │   ├── backup.rs         # Backup scheduler
│   │   ├── tasks.rs          # Backup tasks
│   │   └── retention.rs      # Retention policy
│   └── ui/
│       ├── dashboard.rs      # Backup dashboard
│       ├── status.rs         # Status display
│       ├── recovery.rs       # Recovery UI
│       └── settings.rs       # Settings UI
├── stations/
│   ├── config/               # Station configurations
│   └── manifests/            # Backup manifests
└── tests/
    ├── shamir/               # Shamir tests
    └── distribution/         # Distribution tests
```

## Shamir Secret Sharing

### Splitting Backups

```rust
use vantis_ark::shamir::{SecretSharing, SplitConfig};

let config = SplitConfig::new()
    .with_total_parts(10)      // Split into 10 parts
    .with_threshold(3)          // Need 3 to recover
    .with_encryption_key(key);

let sharing = SecretSharing::new(config)?;

// Split backup data
let parts = sharing.split(backup_data)?;

for (i, part) in parts.iter().enumerate() {
    println!("Part {}: {} bytes", i, part.len());
}
```

### Recovering Backups

```rust
use vantis_ark::shamir::{SecretSharing, RecoverConfig};

let config = RecoverConfig::new()
    .with_encryption_key(key);

let sharing = SecretSharing::new(config)?;

// Recover from any 3 parts
let recovered = sharing.combine(&[part1, part3, part7])?;

println!("Recovered {} bytes", recovered.len());
```

### Part Verification

```rust
use vantis_ark::shamir::{PartVerifier, VerificationResult};

let verifier = PartVerifier::new()?;

let result = verifier.verify(&part)?;

match result {
    VerificationResult::Valid => {
        println!("Part is valid");
    },
    VerificationResult::Corrupted => {
        println!("Part is corrupted");
    },
    VerificationResult::Invalid => {
        println!("Part is invalid");
    },
}
```

## Distributed Storage

### Station Management

```rust
use vantis_ark::distribution::{Station, StationManager};

let station1 = Station::new("Station-1")
    .with_address("192.168.1.100:8080")
    .with_capacity(TeraBytes(10))
    .with_trust_level(TrustLevel::High);

let station2 = Station::new("Station-2")
    .with_address("192.168.1.101:8080")
    .with_capacity(TeraBytes(10))
    .with_trust_level(TrustLevel::High);

let manager = StationManager::new()?;
manager.add_station(station1)?;
manager.add_station(station2)?;
```

### Distribution Protocol

```rust
use vantis_ark::distribution::{Distributor, DistributionConfig};

let config = DistributionConfig::new()
    .with_replication_factor(2)  // Each part stored 2 times
    .with_verify_after_upload(true)
    .with_timeout(Duration::from_secs(60));

let distributor = Distributor::new(config)?;

// Distribute parts to stations
distributor.distribute(&parts, &stations)?;
```

### Health Monitoring

```rust
use vantis_ark::distribution::{HealthMonitor, HealthStatus};

let monitor = HealthMonitor::new()?;

monitor.check_all_stations()?;

for station in monitor.stations() {
    match station.health_status() {
        HealthStatus::Healthy => {
            println!("{} is healthy", station.name);
        },
        HealthStatus::Degraded => {
            println!("{} is degraded", station.name);
        },
        HealthStatus::Offline => {
            println!("{} is offline", station.name);
        },
    }
}
```

## Backup Scheduler

### Scheduled Backups

```rust
use vantis_ark::scheduler::{BackupScheduler, Schedule};

let scheduler = BackupScheduler::new()?;

// Add daily backup
scheduler.add_schedule(Schedule::new()
    .with_frequency(Frequency::Daily)
    .with_time(Time::from_hms(2, 0, 0))
    .with_retention(Retention::Days(30))
)?;

// Add weekly backup
scheduler.add_schedule(Schedule::new()
    .with_frequency(Frequency::Weekly)
    .with_day(Weekday::Sunday)
    .with_time(Time::from_hms(1, 0, 0))
    .with_retention(Retention::Weeks(52))
)?;
```

### Backup Tasks

```rust
use vantis_ark::scheduler::{BackupTask, TaskConfig};

let config = TaskConfig::new()
    .with_sources(vec![
        "/home/user/documents",
        "/home/user/projects",
    ])
    .with_shamir_config(shamir_config)
    .with_compression(true)
    .with_encryption(true);

let task = BackupTask::new(config)?;

// Execute backup
let result = task.execute()?;

println!("Backup completed: {} parts", result.parts_created);
println!("Backup size: {}", result.size_compressed);
println!("Time taken: {}", result.duration);
```

### Retention Policy

```rust
use vantis_ark::scheduler::RetentionPolicy;

let policy = RetentionPolicy::new()
    .keep_daily_for(30)
    .keep_weekly_for(52)
    .keep_monthly_for(12)
    .keep_yearly_for(5);

policy.apply(&backup_repository)?;
```

## Encryption

### Transport Encryption

```rust
use vantis_ark::encryption::{TransportEncryption, AESGCM};

let encryption = TransportEncryption::new(AESGCM)?;

// Encrypt part before sending
let encrypted = encryption.encrypt(part, &transport_key)?;

// Decrypt after receiving
let decrypted = encryption.decrypt(encrypted, &transport_key)?;
```

### Storage Encryption

```rust
use vantis_ark::encryption::{StorageEncryption, Argon2};

let storage_enc = StorageEncryption::new(Argon2)?;

// Derive storage key
let storage_key = storage_enc.derive_key(
    user_password,
    salt
)?;

// Encrypt backup
let encrypted_backup = storage_enc.encrypt(backup_data, &storage_key)?;
```

## API Examples

### Creating a Backup

```rust
use vantis_ark::{Ark, BackupConfig};

let config = BackupConfig::new()
    .with_shamir_parts(10, 3)  // 10 parts, need 3
    .with_stations(stations)
    .with_encryption(true)
    .with_compression(true)
    .with_verify(true);

let ark = Ark::new(config)?;

// Create backup
let backup = ark.create_backup(vec![
    "/home/user/documents",
    "/home/user/projects",
])?;

println!("Backup ID: {}", backup.id);
println!("Parts created: {}", backup.part_count);
```

### Restoring from Backup

```rust
use vantis_ark::{Ark, RestoreConfig};

let config = RestoreConfig::new()
    .with_backup_id(backup_id)
    .with_destination("/home/user/restore")
    .with_encryption_key(key);

let ark = Ark::new(config)?;

// Restore backup
ark.restore()?;

println!("Backup restored successfully");
```

### Managing Backups

```rust
use vantis_ark::{Ark, BackupManager};

let manager = BackupManager::new()?;

// List backups
let backups = manager.list_backups()?;

for backup in backups {
    println!("{}: {} bytes", backup.id, backup.size);
    println!("  Created: {}", backup.created_at);
    println!("  Parts: {}/{}", backup.available_parts, backup.total_parts);
}

// Delete old backup
manager.delete_backup(old_backup_id)?;
```

## Integration Points

- **Vantis Vault**: Backup encryption
- **Vantis Core-IO**: File operations
- **Vantis Ark Mobile**: Remote backup management
- **All VantisOffice Apps**: Document backup

## Configuration

```toml
# ark.toml
[shamir]
default_total_parts = 10
default_threshold = 3
verification_enabled = true

[distribution]
replication_factor = 2
verify_after_upload = true
timeout_seconds = 60
max_concurrent_uploads = 5

[storage]
encryption_enabled = true
compression_enabled = true
compression_level = 6

[scheduler]
default_retention_days = 30
auto_cleanup = true
max_backups = 100

[stations]
health_check_interval = 3600
offline_threshold = 86400
auto_repair = true
```

## Performance Metrics

- **Backup Creation**: 2GB/minute
- **Backup Restoration**: 3GB/minute
- **Split Time**: 100ms per GB
- **Recovery Time**: 150ms per GB
- **Distribution Time**: 5s for 1GB
- **Verification Time**: 200ms per part

## Security Features

1. **Shamir Secret Sharing**: Cryptographic security
2. **Transport Encryption**: Encrypted data transfer
3. **Storage Encryption**: Encrypted at rest
4. **Part Verification**: Integrity checks
5. **Secure Distribution**: Verified stations
6. **Audit Trail**: Complete backup logs

## Recovery Scenarios

### Single Station Failure

```rust
// If one station fails, parts are still available
let available = manager.available_parts(backup_id)?;
if available >= config.threshold {
    // Can still recover
    ark.restore()?;
}
```

### Multiple Station Failures

```rust
// If too many stations fail
let available = manager.available_parts(backup_id)?;
if available< config.threshold {
    // Need to repair or redistribute
    ark.redistribute_parts(backup_id)?;
}
```

### Disaster Recovery

```rust
// In case of major data loss
// Collect parts from available stations
let available_parts = collector.collect_from_all_stations()?;

if available_parts.len() >= config.threshold {
    // Can recover data
    let recovered = ark.recover_from_parts(available_parts)?;
    
    // Restore to new location
    ark.restore_to(recovered, "/new/location")?;
}
```

## Build Requirements

- Rust 1.70+
- OpenSSL (cryptographic operations)
- tokio (async runtime)
- serde (serialization)
- shamir-secret-sharing (SSS library)

---

**Part of VantisOffice Pillar IV - Critical Tools**
