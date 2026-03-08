# Installation Guide

## System Requirements

### Minimum Requirements
- **OS**: Windows 10+, macOS 10.15+, or Linux (glibc 2.31+)
- **RAM**: 4 GB
- **Storage**: 500 MB
- **CPU**: x86_64 or ARM64

### Recommended Requirements
- **OS**: Windows 11, macOS 12+, or Linux (Ubuntu 22.04+)
- **RAM**: 8 GB
- **Storage**: 1 GB
- **CPU**: x86_64 (4+ cores)

## Installation Methods

### Linux

#### Debian/Ubuntu
```bash
# Download the latest .deb package
wget https://github.com/vantisCorp/VantisOffice/releases/latest/download/vantisoffice_amd64.deb

# Install
sudo dpkg -i vantisoffice_amd64.deb
sudo apt-get install -f  # Install dependencies
```

#### Arch Linux
```bash
# From AUR
yay -S vantisoffice
```

#### From Source
```bash
# Install dependencies
sudo apt-get install build-essential cmake libssl-dev pkg-config

# Clone and build
git clone https://github.com/vantisCorp/VantisOffice.git
cd VantisOffice
cargo build --release
```

### macOS

#### Homebrew
```bash
brew install vantisoffice
```

#### Manual Installation
```bash
# Install dependencies
brew install cmake openssl

# Clone and build
git clone https://github.com/vantisCorp/VantisOffice.git
cd VantisOffice
cargo build --release
```

### Windows

#### Installer
```powershell
# Download the installer
Invoke-WebRequest -Uri "https://github.com/vantisCorp/VantisOffice/releases/latest/download/vantisoffice-setup.exe" -OutFile "installer.exe"

# Run the installer
./installer.exe
```

#### From Source
```powershell
# Install Rust from https://rustup.rs
# Install Visual Studio Build Tools

# Clone and build
git clone https://github.com/vantisCorp/VantisOffice.git
cd VantisOffice
cargo build --release
```

## Post-Installation

### Verify Installation
```bash
vantisoffice --version
```

### Initial Configuration
```bash
# Create configuration directory
vantisoffice config init

# Set your preferences
vantisoffice config set theme dark
vantisoffice config set language en
```

## Troubleshooting

### Common Issues

1. **OpenSSL not found**
   ```bash
   # Linux
   sudo apt-get install libssl-dev
   
   # macOS
   brew install openssl
   export OPENSSL_DIR=$(brew --prefix openssl)
   ```

2. **Permission denied**
   ```bash
   chmod +x /usr/local/bin/vantisoffice
   ```

3. **Missing dependencies**
   ```bash
   # Linux
   sudo apt-get install -f
   
   # macOS
   brew install cmake
   ```

## Uninstallation

```bash
# Linux
sudo apt-get remove vantisoffice

# macOS
brew uninstall vantisoffice

# Windows
# Use Add/Remove Programs
```