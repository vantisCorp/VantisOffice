# VantisOffice User Guides

## Welcome to VantisOffice

VantisOffice is a secure, private, and performant office suite designed as an alternative to traditional office software. This collection of user guides will help you get started with each application in the VantisOffice ecosystem.

## VantisOffice Applications

### Productivity Suite (Logic Layer)

1. **Vantis Writer** - Advanced word processor with Markdown support
2. **Vantis Grid** - AI-powered spreadsheet with formula engine
3. **Vantis Canvas** - 3D-accelerated presentations with infinite canvas
4. **Vantis Lens** - Secure PDF viewer with sterilization features

### Collaboration Suite (Sync Layer)

5. **Vantis Link** - Peer-to-peer collaboration with real-time editing
6. **Vantis Flow** - Planning and diagramming tool with mind maps
7. **Vantis Chronos** - Privacy-first calendar with PGP encryption

### Continuity Suite (Continuity Layer)

8. **Vantis Ark** - Distributed backup with Shamir Secret Sharing
9. **Vantis Bridge** - Legacy format converter for Office documents

## Getting Started

### Installation

VantisOffice applications are currently available as Rust libraries with demo applications. To build and run:

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisOffice.git
cd VantisOffice

# Build all modules
cargo build --release

# Run a specific application
cargo run --release -p vantis-writer
cargo run --release -p vantis-grid
cargo run --release -p vantis-canvas
```

### System Requirements

- **Operating System**: Linux, macOS, Windows (experimental)
- **Rust**: 1.93.1 or later
- **Dependencies**:
  - Vulkan SDK (for GPU-accelerated rendering in Vantis Canvas)
  - OpenSSL (for encryption features)
  - cmake (for building certain dependencies)

### Privacy and Security

VantisOffice is designed with privacy at its core:

- **End-to-End Encryption**: All documents encrypted with ChaCha20-Poly1305
- **Zero-Trust Architecture**: No central servers for collaboration
- **Local-First**: Your data stays on your device
- **Hardware Security**: TPM 2.0 support for encryption keys
- **Open Source**: Code available for audit at https://github.com/vantisCorp/VantisOffice

## User Guides by Application

### [Vantis Writer Guide](./vantis_writer_guide.md)
- Creating and editing documents
- Markdown support
- Typography features
- Deep Focus Mode
- Export options

### [Vantis Grid Guide](./vantis_grid_guide.md)
- Creating spreadsheets
- Formula engine
- AI-powered features
- Charts and visualization
- Data import/export

### [Vantis Canvas Guide](./vantis_canvas_guide.md)
- Creating presentations
- Infinite canvas navigation
- Shapes and text
- Animations and transitions
- Export formats

### [Vantis Lens Guide](./vantis_lens_guide.md)
- Viewing PDFs
- PDF sterilization
- Annotations
- E-signatures
- Security features

### [Vantis Link Guide](./vantis_link_guide.md)
- Real-time collaboration
- Peer-to-peer sync
- Conflict resolution
- Session management
- Offline support

### [Vantis Flow Guide](./vantis_flow_guide.md)
- Mind maps
- Flowcharts
- Gantt charts
- Task management
- Export options

### [Vantis Chronos Guide](./vantis_chronos_guide.md)
- Calendar views
- Event creation
- PGP encryption
- Meeting suggestions
- Import/export (ICS)

### [Vantis Ark Guide](./vantis_ark_guide.md)
- Creating backups
- Shamir Secret Sharing
- Backup scheduling
- Recovery procedures
- Storage management

### [Vantis Bridge Guide](./vantis_bridge_guide.md)
- Importing Office documents
- Format conversion
- Sanitization options
- Export to Vantis formats
- Batch processing

## Keyboard Shortcuts

### Common Shortcuts (All Applications)

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Document | Ctrl+N | Cmd+N |
| Open | Ctrl+O | Cmd+O |
| Save | Ctrl+S | Cmd+S |
| Save As | Ctrl+Shift+S | Cmd+Shift+S |
| Undo | Ctrl+Z | Cmd+Z |
| Redo | Ctrl+Y / Ctrl+Shift+Z | Cmd+Y / Cmd+Shift+Z |
| Copy | Ctrl+C | Cmd+C |
| Cut | Ctrl+X | Cmd+X |
| Paste | Ctrl+V | Cmd+V |
| Find | Ctrl+F | Cmd+F |
| Print | Ctrl+P | Cmd+P |

### Application-Specific Shortcuts

See individual application guides for detailed keyboard shortcuts.

## Troubleshooting

### Common Issues

**Build Errors**
- Ensure Rust is installed: `rustc --version`
- Check for missing dependencies (cmake, Vulkan SDK)
- Try running `cargo clean && cargo build`

**Performance Issues**
- Check system resources (CPU, RAM)
- Disable GPU acceleration if needed
- Close other applications

**Encryption Errors**
- Verify TPM 2.0 is available (for Vantis Vault)
- Check OpenSSL installation
- Ensure sufficient entropy for key generation

### Getting Help

- **Documentation**: Check individual application guides
- **Issues**: Report bugs at https://github.com/vantisCorp/VantisOffice/issues
- **Community**: Join our community forums (coming soon)
- **Security**: Report security issues to security@vantis.ai

## Contributing

We welcome contributions! See [CONTRIBUTING_GUIDE.md](../CONTRIBUTING_GUIDE.md) for details.

## License

VantisOffice is proprietary software. See [LICENSE](../LICENSE) for terms.

## Version Information

Current version: **v0.2.0**

Release notes: https://github.com/vantisCorp/VantisOffice/releases

---

**Last Updated**: 2024-03-03  
**Documentation Version**: 1.0  
**VantisOffice Version**: 0.2.0