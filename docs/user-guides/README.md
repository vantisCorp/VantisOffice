# VantisOffice User Guides

Welcome to the VantisOffice User Guides documentation. This directory contains comprehensive guides for all VantisOffice applications.

## 📚 Available Guides

### Getting Started
- **[USER_GUIDES_OVERVIEW.md](./USER_GUIDES_OVERVIEW.md)** - Start here! Complete overview of VantisOffice, installation instructions, and common features.

### Application Guides

#### Productivity Suite
- **[Vantis Writer Guide](./vantis_writer_guide.md)** - Advanced word processor with Markdown support, Babel Typography, and Deep Focus Mode
- **[Vantis Grid Guide](./vantis_grid_guide.md)** - AI-powered spreadsheet with Excel-compatible formulas, charts, and data analysis
- **[Vantis Canvas Guide](./vantis_canvas_guide.md)** - 3D-accelerated presentations with infinite canvas and GPU rendering
- **[Vantis Lens Guide](./vantis_lens_guide.md)** - Secure PDF viewer with automatic sterilization and e-signatures

#### Collaboration Suite
- **[Vantis Link Guide](./vantis_link_guide.md)** - Peer-to-peer collaboration with real-time editing and end-to-end encryption
- **[Vantis Flow Guide](./vantis_flow_guide.md)** - Planning and diagramming with mind maps, flowcharts, Gantt charts, and Kanban boards
- **[Vantis Chronos Guide](./vantis_chronos_guide.md)** - Privacy-first calendar with PGP encryption and time zone support

#### Continuity Suite
- **[Vantis Ark Guide](./vantis_ark_guide.md)** - Distributed backup with Shamir Secret Sharing and automatic scheduling
- **[Vantis Bridge Guide](./vantis_bridge_guide.md)** - Legacy format converter for importing Office documents

## 🚀 Quick Start

### Installation

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

### Choosing Your First Application

**For Writing:**
Start with [Vantis Writer](./vantis_writer_guide.md) - Perfect for documents, notes, and Markdown writing.

**For Data:**
Start with [Vantis Grid](./vantis_grid_guide.md) - Ideal for spreadsheets, calculations, and data analysis.

**For Presentations:**
Start with [Vantis Canvas](./vantis_canvas_guide.md) - Create stunning presentations with infinite canvas.

**For Viewing PDFs:**
Start with [Vantis Lens](./vantis_lens_guide.md) - Secure PDF viewer with sterilization features.

**For Collaboration:**
Start with [Vantis Link](./vantis_link_guide.md) - Real-time collaboration with end-to-end encryption.

**For Planning:**
Start with [Vantis Flow](./vantis_flow_guide.md) - Mind maps, flowcharts, Gantt charts, and Kanban boards.

**For Scheduling:**
Start with [Vantis Chronos](./vantis_chronos_guide.md) - Privacy-first calendar with PGP encryption.

**For Backup:**
Start with [Vantis Ark](./vantis_ark_guide.md) - Distributed backup with Shamir Secret Sharing.

**For Conversion:**
Start with [Vantis Bridge](./vantis_bridge_guide.md) - Import and convert Office documents.

## 🔑 Key Features by Application

| Application | Key Features |
|-------------|--------------|
| Vantis Writer | Markdown support, Babel Typography, Deep Focus Mode |
| Vantis Grid | AI-powered, 10GB+ datasets, Excel-compatible formulas |
| Vantis Canvas | Infinite canvas, 120Hz GPU rendering, 30+ easing functions |
| Vantis Lens | PDF sterilization, E-signatures, Secure sandbox |
| Vantis Link | P2P architecture, End-to-end encryption, CRDT sync |
| Vantis Flow | Mind maps, Flowcharts, Gantt charts, Kanban boards |
| Vantis Chronos | PGP encryption, Time zones, Recurring events |
| Vantis Ark | Shamir Secret Sharing, Distributed storage, Auto-scheduling |
| Vantis Bridge | Import DOCX/XLSX/PPTX, Sanitization, Batch processing |

## 📖 Guide Structure

Each guide follows a consistent structure:

1. **Introduction** - Overview and key features
2. **Getting Started** - Installation and first steps
3. **Features** - Detailed feature documentation
4. **Usage** - How to use specific features
5. **Keyboard Shortcuts** - Quick reference
6. **Tips and Tricks** - Best practices
7. **Troubleshooting** - Common issues and solutions

## 🎯 Recommended Reading Path

### New Users
1. Start with [USER_GUIDES_OVERVIEW.md](./USER_GUIDES_OVERVIEW.md)
2. Choose your first application from the list above
3. Read the corresponding application guide
4. Explore keyboard shortcuts
5. Try tips and tricks

### Advanced Users
1. Review all application guides
2. Focus on collaboration features (Vantis Link)
3. Explore advanced features in each application
4. Implement security best practices

### Administrators
1. Review security features in each guide
2. Configure backup with Vantis Ark
3. Set up collaboration with Vantis Link
4. Implement encryption policies

## 🔒 Security & Privacy

VantisOffice is designed with privacy at its core:

- **End-to-End Encryption**: All documents encrypted with ChaCha20-Poly1305
- **Zero-Trust Architecture**: No central servers for collaboration
- **Local-First**: Your data stays on your device
- **Hardware Security**: TPM 2.0 support for encryption keys
- **Open Source**: Code available for audit

Learn more about security in individual application guides.

## ⌨️ Common Keyboard Shortcuts

These shortcuts work across most applications:

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Document | `Ctrl+N` | `Cmd+N` |
| Open | `Ctrl+O` | `Cmd+O` |
| Save | `Ctrl+S` | `Cmd+S` |
| Undo | `Ctrl+Z` | `Cmd+Z` |
| Redo | `Ctrl+Y` | `Cmd+Y` |
| Copy | `Ctrl+C` | `Cmd+C` |
| Cut | `Ctrl+X` | `Cmd+X` |
| Paste | `Ctrl+V` | `Cmd+V` |
| Find | `Ctrl+F` | `Cmd+F` |
| Print | `Ctrl+P` | `Cmd+P` |

## 🆘 Getting Help

### Documentation
- Check the specific application guide
- Review troubleshooting sections
- Check keyboard shortcuts

### Community
- GitHub Issues: https://github.com/vantisCorp/VantisOffice/issues
- Community Forums (coming soon)

### Security
- Report security issues: security@vantis.ai

## 📦 System Requirements

### Minimum Requirements
- **Operating System**: Linux, macOS, Windows (experimental)
- **Rust**: 1.93.1 or later
- **RAM**: 4GB minimum
- **Storage**: 500MB for installation

### Recommended Requirements
- **Operating System**: Linux, macOS
- **Rust**: 1.93.1 or later
- **RAM**: 8GB or more
- **Storage**: 2GB or more
- **Graphics**: Vulkan-compatible GPU (for Vantis Canvas)

## 🔗 Resources

- **GitHub Repository**: https://github.com/vantisCorp/VantisOffice
- **Latest Release**: https://github.com/vantisCorp/VantisOffice/releases/latest
- **API Documentation**: https://github.com/vantisCorp/VantisOffice/blob/main/docs/API_DOCUMENTATION.md
- **Developer Guide**: https://github.com/vantisCorp/VantisOffice/blob/main/docs/DEVELOPER_GUIDE.md

## 📝 License

VantisOffice is proprietary software. See [LICENSE](../../LICENSE) for terms.

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING_GUIDE.md](../../CONTRIBUTING_GUIDE.md) for details.

---

**Last Updated**: 2024-03-03  
**Documentation Version**: 1.0  
**VantisOffice Version**: 0.3.0