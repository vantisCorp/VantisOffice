# VantisOffice Developer Guide

## Architecture Overview

VantisOffice is built on a modular architecture with four pillars:

### Pillar 01 - Iron (System Foundations)
- **flux-vector-engine**: GPU-accelerated UI framework
- **vantis-core-io**: Core I/O operations
- **vantis-pqc**: Post-quantum cryptography
- **vantis-vault**: Secure storage
- **wasm-sandbox**: Plugin sandboxing

### Pillar 02 - Logic (Productivity Applications)
- **vantis-canvas**: Presentations
- **vantis-grid**: Spreadsheets
- **vantis-lens**: Image editing
- **vantis-writer**: Word processing

### Pillar 03 - Sync (Ecosystem & Collaboration)
- **vantis-chronos**: Calendar and scheduling
- **vantis-flow**: Workflow automation
- **vantis-link**: Collaboration and CRDT

### Pillar 04 - Continuity (Critical Tools)
- **vantis-ark**: Backup and recovery
- **vantis-bridge**: Cross-platform compatibility
- **vantis-mobile**: Mobile applications

## Development Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies (Linux)
sudo apt-get install build-essential cmake libssl-dev pkg-config

# Install dependencies (macOS)
brew install cmake openssl
```

### Building

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisOffice.git
cd VantisOffice

# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --workspace

# Format code
cargo fmt
```

### Project Structure

```
VantisOffice/
├── pillar-01-iron/       # System foundations
├── pillar-02-logic/      # Productivity apps
├── pillar-03-sync/       # Collaboration
├── pillar-04-continuity/ # Critical tools
├── docs/                 # Documentation
├── tests/                # Integration tests
├── examples/             # Example code
└── scripts/              # Build scripts
```

## Creating a Plugin

### Plugin Template

```rust
// my_plugin/src/lib.rs
use vantis_office::plugin::{Plugin, PluginContext, PluginResult};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    fn initialize(&mut self, context: &PluginContext) -> PluginResult<()> {
        // Initialize plugin
        Ok(())
    }

    fn execute(&self, command: &str, args: &[String]) -> PluginResult<String> {
        match command {
            "hello" => Ok("Hello from my plugin!".to_string()),
            _ => Err(format!("Unknown command: {}", command)),
        }
    }
}

vantis_office::export_plugin!(MyPlugin);
```

### Building Plugins

```bash
# Build for WASM target
cargo build --target wasm32-unknown-unknown --release

# The plugin will be in target/wasm32-unknown-unknown/release/
```

## API Reference

### Document API

```rust
use vantis_writer::{Document, Style, Paragraph};

// Create a new document
let mut doc = Document::new();

// Add content
doc.add_paragraph(Paragraph::new("Hello, World!"));

// Apply style
doc.apply_style(Style::Heading1);

// Save
doc.save("document.vw")?;
```

### Spreadsheet API

```rust
use vantis_grid::{Spreadsheet, Cell, Formula};

// Create spreadsheet
let mut sheet = Spreadsheet::new();

// Set cell value
sheet.set_cell(0, 0, Cell::Number(42.0));

// Add formula
sheet.set_cell(1, 0, Cell::Formula(Formula::parse("=A1*2")?));

// Calculate
sheet.calculate();
```

### Cryptography API

```rust
use vantis_pqc::{KyberKeyPair, KyberSecurityLevel, DilithiumKeyPair};

// Generate Kyber keypair
let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768)?;

// Encapsulate
let result = vantis_pqc::encapsulate(keypair.public_key(), KyberSecurityLevel::Kyber768)?;

// Sign with Dilithium
let sign_keypair = DilithiumKeyPair::generate(DilithiumSecurityLevel::Dilithium3)?;
let signature = sign_keypair.sign(message)?;
```

## Testing

### Unit Tests

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test -p vantis-writer --test document_tests

# Run with coverage
cargo tarpaulin --workspace --out Xml
```

### Integration Tests

```bash
# Run integration tests
cargo test --workspace --test '*'
```

### Benchmarks

```bash
# Run benchmarks
cargo bench --workspace
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create tag: `git tag v0.x.0`
4. Push tag: `git push --tags`
5. CI/CD will create release automatically