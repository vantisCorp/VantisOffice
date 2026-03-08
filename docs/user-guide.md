# VantisOffice User Guide

## Introduction

VantisOffice is a comprehensive office suite designed for security, privacy, and performance. This guide covers all major features and workflows.

## Core Applications

### VantisWriter (Word Processing)

VantisWriter is a powerful word processor with real-time collaboration features.

#### Key Features
- Rich text formatting
- Styles and templates
- Tables and images
- Export to PDF, DOCX, ODT
- End-to-end encryption

#### Basic Usage
```bash
# Create a new document
vantis-writer new document.vw

# Open existing document
vantis-writer open document.vw

# Export to PDF
vantis-writer export document.vw output.pdf
```

### VantisGrid (Spreadsheets)

VantisGrid provides powerful spreadsheet capabilities with advanced formula support.

#### Key Features
- 1M+ rows support
- 200+ built-in functions
- Pivot tables
- Charts and graphs
- Real-time collaboration

#### Basic Usage
```bash
# Create a new spreadsheet
vantis-grid new spreadsheet.vg

# Import from CSV
vantis-grid import data.csv spreadsheet.vg

# Export to Excel
vantis-grid export spreadsheet.vg output.xlsx
```

### VantisCanvas (Presentations)

VantisCanvas is a modern presentation tool with GPU-accelerated rendering.

#### Key Features
- Smooth animations
- 3D transitions
- Export to PPTX, PDF
- Presenter mode
- Recording support

#### Basic Usage
```bash
# Create a new presentation
vantis-canvas new presentation.vc

# Present
vantis-canvas present presentation.vc
```

### VantisLens (Image Editing)

VantisLens provides professional image editing capabilities.

#### Key Features
- Non-destructive editing
- Layer support
- AI-powered tools
- Raw image support
- Export to PNG, JPEG, WebP

### VantisVault (Secure Storage)

VantisVault provides encrypted document storage with post-quantum cryptography.

#### Key Features
- AES-256-GCM encryption
- Post-quantum cryptography (Kyber/Dilithium)
- Secure sharing
- Version history
- Cloud sync

## Collaboration Features

### Real-time Collaboration

All VantisOffice applications support real-time collaboration:

1. **Share Document**: Click the share button or use `Ctrl+Shift+S`
2. **Invite Collaborators**: Enter email addresses
3. **Set Permissions**: View, edit, or admin access
4. **Collaborate**: See changes in real-time

### Version Control

```bash
# View history
vantisoffice history document.vw

# Restore previous version
vantisoffice restore document.vw --version 5
```

## Security Features

### End-to-End Encryption

All documents are encrypted by default:
- **At Rest**: AES-256-GCM
- **In Transit**: TLS 1.3
- **Post-Quantum**: Kyber/Dilithium support

### Password Protection

```bash
# Encrypt document
vantisoffice encrypt document.vw --password

# Decrypt document
vantisoffice decrypt document.vw.encrypted --password
```

## Keyboard Shortcuts

### Global Shortcuts
| Shortcut | Action |
|----------|--------|
| Ctrl+N | New document |
| Ctrl+O | Open document |
| Ctrl+S | Save document |
| Ctrl+Shift+S | Save as |
| Ctrl+Q | Quit |
| F11 | Full screen |

### VantisWriter Shortcuts
| Shortcut | Action |
|----------|--------|
| Ctrl+B | Bold |
| Ctrl+I | Italic |
| Ctrl+U | Underline |
| Ctrl+K | Insert link |
| Ctrl+Shift+C | Copy format |
| Ctrl+Shift+V | Paste format |

## Troubleshooting

### Application Won't Start
1. Check system requirements
2. Update graphics drivers
3. Clear cache: `vantisoffice cache clear`

### Slow Performance
1. Close unnecessary applications
2. Reduce undo history size
3. Disable GPU acceleration (if issues)

### File Won't Open
1. Check file permissions
2. Verify file format
3. Try recovery mode: `vantisoffice recover file.vw`

## Getting Help

- **Documentation**: https://github.com/vantisCorp/VantisOffice/wiki
- **Issues**: https://github.com/vantisCorp/VantisOffice/issues
- **Discord**: https://discord.gg/vantisoffice