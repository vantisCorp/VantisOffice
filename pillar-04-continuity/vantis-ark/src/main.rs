//! Demo application for Vantis Ark

use chrono::Utc;
use vantis_ark::*;

fn main() -> Result<(), ArkError> {
    println!("=== Vantis Ark Demo ===\n");

    // Initialize Ark
    init()?;
    println!("✓ Vantis Ark initialized\n");

    // Create storage
    let storage = Box::new(InMemoryStorage::new());
    let backup_manager = BackupManager::new(storage);

    // Create backup
    let data = b"Important data to backup".to_vec();
    let backup = backup_manager.create_backup("My Backup".to_string(), data)?;
    println!("✓ Created backup: {}\n", backup.name);
    println!("  Size: {} bytes\n", backup.metadata.size);
    println!("  Checksum: {}\n", backup.metadata.checksum);

    // Split into parts using Shamir Secret Sharing
    let split_config = SplitConfig::new(10, 3);
    let parts = SecretSharing::split(&backup.data, &split_config);
    println!(
        "✓ Split backup into {} parts (threshold: {})\n",
        parts.len(),
        split_config.threshold
    );

    // Verify parts
    let mut verified_count = 0;
    for part in &parts {
        if PartVerifier::verify(part) {
            verified_count += 1;
        }
    }
    println!("✓ Verified {} parts\n", verified_count);

    // Create stations
    let mut station_manager = StationManager::new();

    let station1 = Station {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Station 1".to_string(),
        url: "https://station1.example.com".to_string(),
        trust_level: TrustLevel::High,
        health_status: HealthStatus::Healthy,
        last_check: Utc::now(),
        capacity: 1024 * 1024 * 1024, // 1GB
        used_capacity: 0,
    };

    let station2 = Station {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Station 2".to_string(),
        url: "https://station2.example.com".to_string(),
        trust_level: TrustLevel::High,
        health_status: HealthStatus::Healthy,
        last_check: Utc::now(),
        capacity: 1024 * 1024 * 1024, // 1GB
        used_capacity: 0,
    };

    let station3 = Station {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Station 3".to_string(),
        url: "https://station3.example.com".to_string(),
        trust_level: TrustLevel::Medium,
        health_status: HealthStatus::Healthy,
        last_check: Utc::now(),
        capacity: 1024 * 1024 * 1024, // 1GB
        used_capacity: 0,
    };

    station_manager.add_station(station1.clone());
    station_manager.add_station(station2.clone());
    station_manager.add_station(station3.clone());
    println!("✓ Added 3 stations\n");

    // Distribute parts
    let healthy_stations = station_manager.get_healthy_stations();
    let distribution_config = DistributionConfig::new(3);
    let distributor = Distributor::new(distribution_config);
    let result = distributor.distribute(&parts, &healthy_stations);
    println!("✓ Distributed parts to stations\n");
    println!("  Total parts: {}\n", result.total_parts);
    println!("  Successful: {}\n", result.successful);
    println!("  Failed: {}\n", result.failed);

    // Test encryption
    let transport_encryption = TransportEncryption::new(TransportAlgorithm::AES256GCM);
    let key = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let encrypted = transport_encryption.encrypt(b"Secret data", &key)?;
    println!("✓ Encrypted data with transport encryption\n");

    let decrypted = transport_encryption.decrypt(&encrypted, &key)?;
    println!(
        "✓ Decrypted data: {}\n",
        String::from_utf8(decrypted).unwrap()
    );

    // Test storage encryption
    let storage_encryption = StorageEncryption::new(StorageAlgorithm::AES256);
    let encrypted_storage = storage_encryption.encrypt(b"Storage data", "password")?;
    println!("✓ Encrypted data with storage encryption\n");

    let decrypted_storage = storage_encryption.decrypt(&encrypted_storage, "password")?;
    println!(
        "✓ Decrypted storage data: {}\n",
        String::from_utf8(decrypted_storage).unwrap()
    );

    // Test key management
    let mut key_manager = KeyManager::new();
    let generated_key = key_manager.generate_key(32);
    println!("✓ Generated 32-byte key\n");

    let derived_key = key_manager.derive_key("my_password", b"salt");
    println!("✓ Derived key from password\n");

    // Test scheduler
    let mut scheduler = BackupScheduler::new();

    let daily_schedule = Schedule {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Daily Backup".to_string(),
        frequency: ScheduleFrequency::Daily,
        config: scheduler::ScheduleConfig {
            hour: 2,
            minute: 0,
            day_of_month: 1,
            weekdays: vec![Weekday::Monday],
        },
        retention: RetentionPolicy::default(),
        enabled: true,
        created_at: Utc::now(),
    };

    let weekly_schedule = Schedule {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Weekly Backup".to_string(),
        frequency: ScheduleFrequency::Weekly,
        config: scheduler::ScheduleConfig {
            hour: 3,
            minute: 0,
            day_of_month: 1,
            weekdays: vec![Weekday::Sunday],
        },
        retention: RetentionPolicy::default(),
        enabled: true,
        created_at: Utc::now(),
    };

    scheduler.add_schedule(daily_schedule);
    scheduler.add_schedule(weekly_schedule);
    println!("✓ Added 2 backup schedules\n");

    // Test UI
    let dashboard = Dashboard::new();
    println!("=== Dashboard ===");
    println!("{}", dashboard.render());

    let status_display = StatusDisplay::new(BackupStatus::Completed);
    println!("\n=== Status ===");
    println!("{}", status_display.render());

    let recovery_ui = RecoveryUI::new();
    println!("\n=== Recovery UI ===");
    println!("{}", recovery_ui.render());

    let settings_ui = SettingsUI::new();
    println!("\n=== Settings ===");
    println!("{}", settings_ui.render());

    // Test recovery
    let recover_config = RecoverConfig::new(3);
    let recovered_data = SecretSharing::recover(&parts[..3], &recover_config)?;
    println!("\n✓ Recovered data from 3 parts\n");
    println!(
        "  Recovered: {}\n",
        String::from_utf8(recovered_data).unwrap()
    );

    println!("=== Demo Complete ===");
    Ok(())
}
