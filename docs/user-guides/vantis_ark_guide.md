# Vantis Ark User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Creating Backups](#creating-backups)
4. [Shamir Secret Sharing](#shamir-secret-sharing)
5. [Backup Stations](#backup-stations)
6. [Recovery](#recovery)
7. [Scheduling](#scheduling)
8. [Settings](#settings)
9. [Security](#security)
10. [Troubleshooting](#troubleshooting)

## Introduction

Vantis Ark is a distributed backup system using Shamir Secret Sharing for secure, distributed storage across multiple backup stations.

**Key Features:**
- Shamir Secret Sharing (split data into multiple parts)
- Distributed backup stations
- Automatic backup scheduling
- Health monitoring of backup stations
- Recovery from partial backups
- End-to-end encryption

## Getting Started

### Launching Vantis Ark

```bash
cargo run --release -p vantis-ark
```

### Initial Setup

1. Launch Vantis Ark
2. Configure backup stations
3. Set up encryption keys
4. Create your first backup

### Interface Overview

```
┌─────────────────────────────────────────────────────┐
│  File  Backup  Recovery  Stations  Settings  Help │
├─────────────────────────────────────────────────────┤
│  [New Backup] [Recover] [Add Station]             │
├─────────────────────────────────────────────────────┤
│  Dashboard:                                        │
│  ├─ Total Backups: 24                              │
│  ├─ Storage Used: 4.2 GB / 50 GB                   │
│  ├─ Active Stations: 5 / 10                        │
│  └─ Last Backup: 2 hours ago                      │
├─────────────────────────────────────────────────────┤
│  Backup Stations:                                  │
│  ├─ Station 1 (Home)     ● Healthy   2.1 GB       │
│  ├─ Station 2 (Office)   ● Healthy   1.8 GB       │
│  ├─ Station 3 (Cloud)    ● Healthy   0.3 GB       │
│  └─ Station 4 (Friend)   ⚠ Offline   ---          │
├─────────────────────────────────────────────────────┤
│  Recent Backups:                                   │
│  ├─ Documents     Mar 3, 10:00 AM    256 MB      │
│  ├─ Photos        Mar 3, 9:30 AM     1.2 GB      │
│  └─ Projects      Mar 2, 6:00 PM     512 MB      │
└─────────────────────────────────────────────────────┘
```

## Creating Backups

### New Backup

**Create Backup:**
1. Click **New Backup** or press `Ctrl+N` / `Cmd+N`
2. Select source files/folders
3. Configure backup settings
4. Choose target stations
5. Start backup

### Backup Configuration

**Basic Settings:**
- Name: Backup name
- Source: Files/folders to backup
- Destination: Backup stations
- Encryption: Enable/disable

**Advanced Settings:**
- Parts: Number of Shamir parts (N)
- Threshold: Minimum parts for recovery (K)
- Compression: Enable/disable
- Incremental: Only changed files
- Retention: How long to keep

### Shamir Configuration

**Parts and Threshold:**
- Total Parts (N): How many pieces to split into
- Threshold (K): How many parts needed to recover
- Recommendation: Use 10 parts, threshold of 3

**Example Configurations:**
- **High Security**: 10 parts, threshold 7
- **Balanced**: 10 parts, threshold 3
- **Quick Recovery**: 5 parts, threshold 2

### Backup Process

**During Backup:**
1. Files encrypted locally
2. Data split using Shamir
3. Parts distributed to stations
4. Checksums verified
5. Backup confirmed

**Backup Report:**
- Total size
- Files backed up
- Time elapsed
- Stations used
- Errors/warnings

## Shamir Secret Sharing

### Understanding Shamir

Shamir Secret Sharing splits your data into multiple parts:

**How it Works:**
1. Data encrypted with random key
2. Key split into N parts using polynomial
3. Any K parts can reconstruct the key
4. Less than K parts reveals nothing

**Benefits:**
- No single point of failure
- Survives station failures
- Quantum-resistant
- Trustless recovery

### Part Management

**View Parts:**
1. Select backup
2. **Backup** → **View Parts**
3. See part distribution:
   - Which station has which part
   - Part verification status
   - Part checksums

**Verify Parts:**
1. Select backup
2. **Backup** → **Verify Parts**
3. System checks all parts
4. Report shows:
   - Valid parts
   - Invalid parts
   - Missing parts

### Part Redundancy

**Geographic Distribution:**
- Distribute parts across locations
- Different networks
- Different trust domains

**Trust Levels:**
- **High Trust**: Home, personal servers
- **Medium Trust**: Friends, family
- **Low Trust**: Cloud providers, public servers

## Backup Stations

### Adding Stations

**Add New Station:**
1. **Stations** → **Add Station**
2. Enter station details:
   - Name
   - Type (local, remote, cloud)
   - Address/URL
   - Authentication
3. Test connection
4. Save station

**Station Types:**
- **Local**: External drive, NAS
- **Remote**: Another computer on network
- **Cloud**: S3, Dropbox, custom cloud
- **Friend**: Shared station with trusted contact

### Station Properties

**Basic Properties:**
- Name: Station identifier
- Type: Storage type
- Address: Network address
- Status: Online/Offline

**Advanced Properties:**
- Capacity: Maximum storage
- Used: Current usage
- Trust Level: High/Medium/Low
- Health Score: 0-100%

**Security Properties:**
- Encryption key
- Authentication method
- Trust certificate

### Station Health

**Monitor Health:**
1. **Stations** → **Health Dashboard**
2. View all stations:
   - Status (● Healthy, ⚠ Warning, ✗ Offline)
   - Response time
   - Storage available
   - Last check

**Health Alerts:**
- Station offline
- Low storage
- High response time
- Corrupted parts

### Removing Stations

**Remove Station:**
1. Select station
2. **Stations** → **Remove**
3. Choose:
   - Transfer parts to another station
   - Delete parts
4. Confirm removal

## Recovery

### Starting Recovery

**Recover Backup:**
1. Click **Recover**
2. Select backup to recover
3. Choose destination
4. Select parts to use
5. Start recovery

### Recovery Process

**Steps:**
1. Gather available parts
2. Verify part checksums
3. Reconstruct encryption key
4. Decrypt data
5. Restore to destination

**Recovery Report:**
- Parts used
- Verification status
- Data integrity
- Time elapsed
- Errors/warnings

### Partial Recovery

If not enough parts available:

**Insufficient Parts:**
- System shows missing part count
- Request parts from other stations
- Wait for offline stations

**Forced Partial Recovery:**
- Recover what's available
- Some files may be corrupted
- Check integrity after recovery

### Recovery Scenarios

**Full Recovery:**
- All parts available
- Complete data restoration
- Guaranteed integrity

**Partial Recovery:**
- Some stations offline
- Delay recovery until stations online
- Or proceed with risk

**Emergency Recovery:**
- Minimum threshold parts
- Critical data recovery
- Verify after restoration

## Scheduling

### Automatic Backups

**Create Schedule:**
1. **Settings** → **Scheduling**
2. Create new schedule:
   - Backup name
   - Source files
   - Frequency
   - Time
   - Retention

**Schedule Types:**
- **Daily**: Every day at specified time
- **Weekly**: On specified days
- **Monthly**: On specified day of month
- **Custom**: Custom cron expression

### Retention Policies

**Set Retention:**
1. Create or edit schedule
2. Set retention policy:
   - Keep all versions
   - Keep last N versions
   - Keep for N days
   - Custom rules

**Retention Examples:**
- Keep daily backups for 30 days
- Keep weekly backups for 12 weeks
- Keep monthly backups for 12 months
- Keep yearly backups forever

### Schedule Management

**View Schedules:**
1. **Settings** → **Schedules**
2. See all schedules:
   - Name
   - Frequency
   - Last run
   - Next run
   - Status

**Pause/Resume:**
1. Select schedule
2. Click **Pause** or **Resume**
3. Schedule stops/starts running

## Settings

### General Settings

**Application:**
- Default backup location
- Default parts/threshold
- Auto-start on boot
- Minimize to tray
- Notifications

**Network:**
- Connection timeout
- Retry attempts
- Bandwidth limit
- Proxy settings

### Encryption Settings

**Encryption Options:**
- Algorithm (AES-256-GCM, ChaCha20-Poly1305)
- Key derivation (Argon2, PBKDF2)
- Key rotation frequency

**Key Management:**
- View encryption keys
- Export keys (secure backup)
- Rotate keys
- Import keys

### Storage Settings

**Station Defaults:**
- Default station for new backups
- Minimum free space
- Auto-balance distribution

**Cleanup:**
- Remove orphaned parts
- Compact storage
- Verify integrity

## Security

### Encryption

**Encryption Details:**
- All data encrypted before splitting
- Keys never stored with data
- End-to-end encryption

**Key Protection:**
- Master password
- Hardware key (YubiKey)
- TPM 2.0 integration

### Trust Model

**Station Trust:**
- **High Trust**: Full encryption key
- **Medium Trust**: Part-level encryption
- **Low Trust**: Double encryption

**Verification:**
- Verify station identity
- Check part integrity
- Monitor for tampering

### Security Audit

**Run Audit:**
1. **Settings** → **Security** → **Audit**
2. System checks:
   - Encryption status
   - Part distribution
   - Station health
   - Key rotation
   - Access logs

## Troubleshooting

### Common Issues

**Backup Fails:**
- Check source files exist
- Verify station connectivity
- Check storage space
- Review error logs

**Recovery Fails:**
- Ensure sufficient parts
- Verify station online
- Check part integrity
- Try alternative stations

**Station Offline:**
- Check network connectivity
- Verify station address
- Check authentication
- Review station logs

**Parts Corrupted:**
- Verify checksums
- Re-verify parts
- Use alternative parts
- Restore from different station

### Error Messages

**"Insufficient parts for recovery":**
- Need more parts (K threshold not met)
- Wait for offline stations
- Check station availability

**"Part verification failed":**
- Part checksum mismatch
- Re-backup from source
- Use different part

**"Station not responding":**
- Network issue
- Station offline
- Authentication failed

### Getting Help

- Check documentation
- Review error logs
- Run diagnostics
- Report issues at: https://github.com/vantisCorp/VantisOffice/issues

---

**Last Updated**: 2024-03-03  
**VantisArk Version**: 0.2.0