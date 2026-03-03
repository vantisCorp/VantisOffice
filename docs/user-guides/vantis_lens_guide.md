# Vantis Lens User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Opening PDFs](#opening-pdfs)
4. [PDF Sterilization](#pdf-sterilization)
5. [Annotations](#annotations)
6. [E-Signatures](#e-signatures)
7. [Viewing Options](#viewing-options)
8. [Search and Navigation](#search-and-navigation)
9. [Export Options](#export-options)
10. [Keyboard Shortcuts](#keyboard-shortcuts)
11. [Security Features](#security-features)
12. [Tips and Tricks](#tips-and-tricks)

## Introduction

Vantis Lens is a secure PDF viewer with automatic sterilization, e-signature support, and comprehensive annotation tools.

**Key Features:**
- Automatic PDF sterilization (removes metadata, JavaScript, embedded files)
- Secure sandbox rendering
- E-signature support (eIDAS compliant)
- Multiple annotation types (text, highlight, shapes, stamps)
- Export to PDF, PNG, JPEG, Text, HTML
- Read-only and annotation modes

## Getting Started

### Launching Vantis Lens

```bash
cargo run --release -p vantis-lens
```

### Opening Your First PDF

1. Launch Vantis Lens
2. Click "Open PDF" or press `Ctrl+O` / `Cmd+O`
3. Select PDF file
4. Document opens with automatic sterilization

### Interface Overview

```
┌─────────────────────────────────────────────────────┐
│  File  View  Tools  Annotate  Sign  Export  Help │
├─────────────────────────────────────────────────────┤
│  Open  Print  Zoom  Full Screen  Preferences       │
├─────────────────────────────────────────────────────┤
│  │ ◀ │ 1 / 24 │ ▶ │  Search: [____________]      │
├─────────────────────────────────────────────────────┤
│                                             [Zoom]  │
│                                             │ ▲   │
│  ┌──────────────────────────────────┐       ◄──┼──►│
│  │                                  │       │ ▼   │
│  │          PDF Content             │              │
│  │                                  │              │
│  │                                  │              │
│  │                                  │              │
│  │                                  │              │
│  └──────────────────────────────────┘              │
│                                             [Fit]  │
│                                                   │
├─────────────────────────────────────────────────────┤
│  [Text] [Highlight] [Shape] [Stamp] [Signature]  │
└─────────────────────────────────────────────────────┘
```

## Opening PDFs

### Opening Files

**Open PDF File:**
1. **File** → **Open** or press `Ctrl+O` / `Cmd+O`
2. Navigate to PDF file
3. Select and open
4. PDF is automatically sterilized

**Open Recent:**
1. **File** → **Open Recent**
2. Choose from list of recently opened files

**Open from URL:**
1. **File** → **Open from URL**
2. Enter PDF URL
3. Click **Open**
4. PDF downloads and opens

### File Information

View PDF metadata:

1. **File** → **Properties**
2. View information:
   - Title
   - Author
   - Subject
   - Keywords
   - Creation date
   - Modified date
   - Page count
   - File size
   - PDF version

### Page Navigation

**Page Controls:**
- Next page: `→`, `Space`, `Page Down`
- Previous page: `←`, `Backspace`, `Page Up`
- First page: `Home`
- Last page: `End`
- Go to page: `Ctrl+G` / `Cmd+G` then enter page number

**Scrolling:**
- Mouse wheel: Scroll vertically
- Shift + mouse wheel: Scroll horizontally
- `Ctrl+↑` / `Ctrl+↓`: Scroll up/down
- `Ctrl+←` / `Ctrl+→`: Scroll left/right

## PDF Sterilization

Vantis Lens automatically sterilizes PDFs for security.

### What is PDF Sterilization?

PDF sterilization removes potentially harmful or tracking elements from PDFs:

**Removed Elements:**
- Metadata (author, creation date, etc.)
- JavaScript code
- Embedded files
- External references
- Annotations with sensitive data
- Document actions
- Form fields
- Bookmarks/Outlines

### Sterilization Options

**Automatic Sterilization:**
- Enabled by default
- Applied when opening PDF
- Can be disabled in preferences

**Manual Sterilization:**
1. **Tools** → **Sterilize PDF**
2. Review sterilization report
3. Configure options:
   - Remove metadata
   - Remove JavaScript
   - Remove embedded files
   - Remove annotations
   - Remove forms
   - Remove bookmarks
4. Click **Sterilize**

### Sterilization Report

View what was removed:

1. **Tools** → **Sterilization Report**
2. Report shows:
   - Original PDF size
   - Sterilized PDF size
   - Elements removed
   - Actions taken
   - Warnings

### Saving Sterilized PDF

1. **File** → **Save As**
2. Choose location
3. Sterilized PDF is saved

## Annotations

Vantis Lens provides comprehensive annotation tools.

### Annotation Tools

**Text Annotations:**
1. Select **Text** tool from toolbar
2. Click on PDF to add text box
3. Type your text
4. Click outside to finish

**Highlighting:**
1. Select **Highlight** tool from toolbar
2. Drag over text to highlight
3. Choose highlight color
4. Adjust opacity

**Sticky Notes:**
1. Select **Sticky Note** tool from toolbar
2. Click on PDF to add note
3. Type your note
4. Click outside to finish

**Shapes:**
1. Select **Shape** tool from toolbar
2. Choose shape (rectangle, circle, line, arrow)
3. Draw on PDF
4. Adjust size and position

**Freehand Drawings:**
1. Select **Freehand** tool from toolbar
2. Draw on PDF
3. Adjust line width and color
4. Click outside to finish

**Stamps:**
1. Select **Stamp** tool from toolbar
2. Choose stamp type:
   - Approved
   - Rejected
   - Confidential
   - Draft
   - Final
3. Click on PDF to place stamp

**Eraser:**
1. Select **Eraser** tool from toolbar
2. Click on annotation to remove
3. Or use **Annotate** → **Delete All** to clear all

### Annotation Properties

**Color:**
1. Select annotation
2. **Annotate** → **Properties** → **Color**
3. Choose color from palette

**Opacity:**
1. Select annotation
2. **Annotate** → **Properties** → **Opacity**
3. Adjust opacity slider (0-100%)

**Line Width:**
1. Select annotation or shape
2. **Annotate** → **Properties** → **Line Width**
3. Adjust width in pixels

**Font:**
1. Select text annotation
2. **Annotate** → **Properties** → **Font**
3. Choose font family and size

**Position:**
- Drag annotation to move
- Drag corners to resize
- Rotate by dragging rotation handle

### Managing Annotations

**List View:**
1. **Annotate** → **Annotations**
2. See list of all annotations
3. Click to jump to annotation
4. Right-click for options:
   - Delete
   - Edit properties
   - Copy
   - Hide/Show

**Export Annotations:**
1. **Annotate** → **Export Annotations**
2. Choose format:
   - Include in PDF
   - Export as XFDF
   - Export as JSON

**Import Annotations:**
1. **Annotate** → **Import Annotations**
2. Select file (XFDF, JSON)
3. Annotations merge with existing

## E-Signatures

Vantis Lens supports eIDAS-compliant digital signatures.

### Creating Signatures

**Draw Signature:**
1. **Sign** → **Draw Signature**
2. Draw signature with mouse/touch
3. Click **Accept** or **Redraw**
4. Place signature on PDF

**Type Signature:**
1. **Sign** → **Type Signature**
2. Type name
3. Choose font style
4. Click **Accept**
5. Place signature on PDF

**Import Signature:**
1. **Sign** → **Import Signature**
2. Select signature image file
3. Place signature on PDF

### Digital Certificates

**Import Certificate:**
1. **Sign** → **Manage Certificates**
2. **Import Certificate**
3. Select certificate file (.p12, .pfx)
4. Enter password if required

**Select Certificate:**
1. **Sign** → **Digital Sign**
2. Choose certificate
3. Enter password if required
4. Select signature area
5. Configure signature properties:
   - Reason
   - Location
   - Contact info
6. Click **Sign**

**Verify Signature:**
1. Click on signature
2. **Sign** → **Verify**
3. View signature details:
   - Signer name
   - Certificate issuer
   - Validity dates
   - Signature status (valid, invalid, unknown)

### Signature Properties

**Appearance:**
1. Select signature
2. **Sign** → **Properties**
3. Configure:
   - Show name
   - Show date
   - Show reason
   - Show logo
   - Custom text

**Timestamp:**
1. Select signature
2. **Sign** → **Add Timestamp**
3. Timestamp server adds trusted timestamp

## Viewing Options

### Zoom

**Zoom Controls:**
- Zoom In: `Ctrl++` / `Cmd++`
- Zoom Out: `Ctrl+-` / `Cmd+-`
- Zoom 100%: `Ctrl+1` / `Cmd+1`
- Fit to Width: `Ctrl+2` / `Cmd+2`
- Fit to Page: `Ctrl+0` / `Cmd+0`

**Zoom Slider:**
- Drag zoom slider in toolbar
- Shows current zoom level (25% - 400%)

### Page Layout

**Single Page:**
- **View** → **Single Page**
- Show one page at a time

**Single Page Continuous:**
- **View** → **Single Page Continuous**
- Show one column of pages

**Two Pages:**
- **View** → **Two Pages**
- Show two pages side by side

**Two Pages Continuous:**
- **View** → **Two Pages Continuous**
- Show two columns of pages

### Rotation

**Rotate Pages:**
1. **View** → **Rotate**
2. Choose rotation:
   - Rotate Clockwise: `Ctrl+Shift++` / `Cmd+Shift++`
   - Rotate Counter-Clockwise: `Ctrl+Shift+-` / `Cmd+Shift+-`
   - Rotate 180°

**Auto-Rotate:**
1. **View** → **Auto-Rotate**
2. Pages automatically rotate to fit orientation

### Display Mode

**Full Screen:**
- Press `F11` (Windows/Linux) or `Ctrl+Cmd+F` (macOS)
- Hides all UI elements
- Press `Esc` to exit

**Presentation Mode:**
- Press `F5`
- Optimized for presentations
- Full screen with minimal controls
- Press `Esc` to exit

**Reading Mode:**
- **View** → **Reading Mode**
- Minimal UI
- Best for reading

**Night Mode:**
- **View** → **Night Mode**
- Inverts colors for comfortable reading in low light

## Search and Navigation

### Searching PDFs

**Basic Search:**
1. Press `Ctrl+F` / `Cmd+F`
2. Enter search term
3. Press Enter to search
4. Results highlighted in document
5. Use `F3` / `Shift+F3` for next/previous

**Advanced Search:**
1. **View** → **Advanced Search**
2. Configure options:
   - Case sensitive
   - Whole words only
   - Regular expressions
   - Search in annotations
   - Search in metadata
3. Click **Search**

**Search Results:**
- Number of results shown
- Click result to jump to location
- Highlights show in document

### Bookmarks

**Add Bookmark:**
1. Navigate to page
2. **View** → **Bookmarks** → **Add Bookmark**
3. Enter bookmark name
4. Click **OK**

**Navigate Bookmarks:**
1. **View** → **Bookmarks** → **Bookmarks Panel**
2. Click bookmark to jump to page
3. Right-click for options:
   - Rename
   - Delete
   - Move up/down

### Thumbnails

**Thumbnail View:**
1. **View** → **Thumbnails**
2. See all pages as thumbnails
3. Click thumbnail to jump to page
4. Drag to reorder pages

## Export Options

### Supported Formats

1. **PDF** (.pdf) - Portable document format
2. **PNG** (.png) - Raster image format
3. **JPEG** (.jpg) - Compressed image format
4. **Text** (.txt) - Plain text format
5. **HTML** (.html) - Web-ready format

### Exporting PDFs

**Export to PDF:**
1. **File** → **Export** → **PDF**
2. Choose export options:
   - Include annotations
   - Include bookmarks
   - Optimize for file size or quality
   - Encrypt with password
3. Click **Export**
4. Choose location and filename

**Export to Images:**
1. **File** → **Export** → **Images**
2. Choose format (PNG or JPEG)
3. Select pages:
   - Current page
   - All pages
   - Page range
4. Configure image settings:
   - Resolution (72, 150, 300 DPI)
   - Quality (for JPEG)
5. Click **Export**

**Export to Text:**
1. **File** → **Export** → **Text**
2. Choose options:
   - Include page numbers
   - Preserve formatting
   - Include annotations
3. Click **Export**

**Export to HTML:**
1. **File** → **Export** → **HTML**
2. Configure options:
   - Include images
   - Include annotations
   - Single file or with separate images
   - CSS style (inline or external)
3. Click **Export**

## Keyboard Shortcuts

### File Operations

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Open | `Ctrl+O` | `Cmd+O` |
| Save | `Ctrl+S` | `Cmd+S` |
| Save As | `Ctrl+Shift+S` | `Cmd+Shift+S` |
| Print | `Ctrl+P` | `Cmd+P` |
| Close | `Ctrl+W` | `Cmd+W` |

### Navigation

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Next Page | `→`, `Space`, `Page Down` | `→`, `Space`, `Page Down` |
| Previous Page | `←`, `Backspace`, `Page Up` | `←`, `Backspace`, `Page Up` |
| First Page | `Home` | `Home` |
| Last Page | `End` | `End` |
| Go to Page | `Ctrl+G` | `Cmd+G` |

### Zoom

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Zoom In | `Ctrl++` | `Cmd++` |
| Zoom Out | `Ctrl+-` | `Cmd+-` |
| Zoom 100% | `Ctrl+1` | `Cmd+1` |
| Fit to Width | `Ctrl+2` | `Cmd+2` |
| Fit to Page | `Ctrl+0` | `Cmd+0` |

### Search

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Find | `Ctrl+F` | `Cmd+F` |
| Find Next | `F3` | `Cmd+G` |
| Find Previous | `Shift+F3` | `Cmd+Shift+G` |
| Advanced Search | `Ctrl+Shift+F` | `Cmd+Shift+F` |

### View

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Full Screen | `F11` | `Ctrl+Cmd+F` |
| Presentation Mode | `F5` | `F5` |
| Exit Full Screen | `Esc` | `Esc` |
| Rotate Clockwise | `Ctrl+Shift++` | `Cmd+Shift++` |
| Rotate Counter-Clockwise | `Ctrl+Shift+-` | `Cmd+Shift+-` |

### Annotation

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Text Tool | `T` | `T` |
| Highlight Tool | `H` | `H` |
| Freehand Tool | `P` | `P` |
| Eraser Tool | `E` | `E` |
| Undo Annotation | `Ctrl+Z` | `Cmd+Z` |
| Redo Annotation | `Ctrl+Y` | `Cmd+Y` |

## Security Features

### Sandbox Rendering

Vantis Lens uses secure sandbox rendering to protect against malicious PDFs.

**Sandbox Benefits:**
- Isolates PDF rendering from system
- Prevents code execution
- Blocks network access
- Limits file system access

**Sandbox Status:**
- Check sandbox status from **Help** → **Sandbox Status**
- Shows sandbox is active and secure

### Encryption

**Password Protection:**
1. **File** → **Encrypt with Password**
2. Set open password
3. Set edit password (optional)
4. Set permissions:
   - Print
   - Copy text
   - Modify annotations
   - Fill forms
5. Click **Encrypt**

**Opening Encrypted PDF:**
1. Open PDF
2. Enter password
3. PDF opens with applied permissions

### Certificate Management

**Import Certificate:**
1. **Sign** → **Manage Certificates** → **Import**
2. Select certificate file
3. Enter password if required
4. Certificate added to keystore

**Remove Certificate:**
1. **Sign** → **Manage Certificates**
2. Select certificate
3. Click **Remove**
4. Confirm removal

### Security Audit

**Run Security Audit:**
1. **Tools** → **Security Audit**
2. Audit checks:
   - JavaScript presence
   - Embedded files
   - External references
   - Encryption status
   - Certificates
   - Metadata
3. Review audit report
4. Fix identified issues

## Tips and Tricks

### Reading Productivity

1. **Reading Mode**: Use reading mode for comfortable viewing
2. **Night Mode**: Reduce eye strain in low light
3. **Custom Zoom**: Set your preferred default zoom
4. **Auto-Scroll**: Enable auto-scroll for hands-free reading
5. **Bookmark Important Pages**: Create bookmarks for quick navigation

### Annotation Tips

1. **Color Coding**: Use consistent colors for different annotation types
2. **Shortcuts**: Learn keyboard shortcuts for common tools
3. **Templates**: Save frequently used annotations as templates
4. **Layers**: Use layers to organize annotations
5. **Export Annotations**: Export annotations for backup or sharing

### Signature Tips

1. **Practice Signature**: Practice drawing your signature
2. **Digital Certificates**: Use digital certificates for legal documents
3. **Timestamps**: Add timestamps for non-repudiation
4. **Multiple Signatures**: Use different signatures for different purposes
5. **Verify Always**: Verify signatures on received documents

### Security Best Practices

1. **Sterilize PDFs**: Always sterilize PDFs from unknown sources
2. **Check Sandbox**: Verify sandbox is active
3. **Review Metadata**: Check for sensitive metadata before sharing
4. **Encrypt Sensitive Docs**: Password protect confidential documents
5. **Update Certificates**: Keep digital certificates up to date

### Performance Tips

1. **Optimize Images**: Reduce PDF size by optimizing images
2. **Disable Animations**: Improve performance by disabling animations
3. **Clear Cache**: Clear cache periodically
4. **Close Unused Tabs**: Close unused PDF documents
5. **Check System Resources**: Ensure sufficient RAM and CPU

## Troubleshooting

### Common Issues

**PDF Won't Open:**
- Check file is not corrupted
- Verify it's a valid PDF file
- Try opening with a different viewer
- Check file permissions

**Annotations Not Saving:**
- Ensure PDF is not read-only
- Check disk space
- Verify you have write permissions
- Try saving to a different location

**Signature Not Valid:**
- Check certificate validity
- Verify certificate issuer
- Check system date/time
- Ensure certificate is trusted

**Performance Issues:**
- Close other applications
- Clear cache
- Reduce zoom level
- Disable hardware acceleration

### Getting Help

- Check documentation for specific features
- Review keyboard shortcuts for navigation
- Report issues at: https://github.com/vantisCorp/VantisOffice/issues

---

**Last Updated**: 2024-03-03  
**VantisLens Version**: 0.2.0