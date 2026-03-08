# Getting Started with VantisOffice

## Quick Start Guide

Welcome to VantisOffice - a secure, private, and performant alternative to Microsoft Office.

### Prerequisites

- **Rust** 1.80.0 or later
- **CMake** 3.20 or later
- **OpenSSL** 3.0 or later
- **Git**

### Installation

#### From Source

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisOffice.git
cd VantisOffice

# Build the project
cargo build --release

# Run tests
cargo test --workspace

# Install binaries
cargo install --path .
```

#### From Crates.io (Coming Soon)

```bash
cargo install vantisoffice
```

### First Steps

1. **Create your first document** - Use `vantis-writer` for word processing
2. **Create a spreadsheet** - Use `vantis-grid` for spreadsheets
3. **Create a presentation** - Use `vantis-canvas` for presentations

### Module Overview

| Module | Purpose |
|--------|---------|
| vantis-writer | Word processing |
| vantis-grid | Spreadsheets |
| vantis-canvas | Presentations |
| vantis-lens | Image editing |
| vantis-vault | Secure storage |
| vantis-pqc | Post-quantum cryptography |

### Next Steps

- Read the [Installation Guide](installation.md)
- Explore the [User Guide](user-guide.md)
- Learn about [Security](security.md)