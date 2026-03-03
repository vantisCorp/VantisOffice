# Vantis Bridge User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Importing Documents](#importing-documents)
4. [Format Conversion](#format-conversion)
5. [Sanitization](#sanitization)
6. [Batch Processing](#batch-processing)
7. [Export Options](#export-options)
8. [Settings](#settings)
9. [Troubleshooting](#troubleshooting)

## Introduction

Vantis Bridge is a legacy format converter that imports Office documents and converts them to Vantis formats with sanitization options.

**Key Features:**
- Import DOCX, XLSX, PPTX, PDF
- Convert to Vantis formats
- Remove macros and scripts
- Sanitize embedded objects
- Batch processing
- Preserve formatting

## Getting Started

### Launching Vantis Bridge

```bash
cargo run --release -p vantis-bridge
```

### Creating Your First Conversion

1. Launch Vantis Bridge
2. Click "Import Document" or press `Ctrl+O` / `Cmd+O`
3. Select file to convert
4. Choose conversion options
5. Start conversion

### Interface Overview

```
┌─────────────────────────────────────────────────────┐
│  File  Import  Convert  Batch  Settings  Help      │
├─────────────────────────────────────────────────────┤
│  [Import] [Convert] [Batch Process] [Export]      │
├─────────────────────────────────────────────────────┤
│  Source: report.docx                               │
│  Status: Ready                                     │
├─────────────────────────────────────────────────────┤
│  Conversion Options:                               │
│  Target Format: ▼ Vantis Writer (Markdown)        │
│  ☑ Remove Macros                                   │
│  ☑ Remove Scripts                                  │
│  ☑ Sanitize Embedded Objects                      │
│  ☑ Remove Metadata                                │
├─────────────────────────────────────────────────────┤
│  Preview:                                          │
│  ┌──────────────────────────────────┐              │
│  │  Report Title                     │              │
│  │  ─────────────────────────────── │              │
│  │                                  │              │
│  │  Document content preview...     │              │
│  │                                  │              │
│  └──────────────────────────────────┘              │
├─────────────────────────────────────────────────────┤
│  [Convert] [Preview Full] [Cancel]                │
└─────────────────────────────────────────────────────┘
```

## Importing Documents

### Supported Formats

**Word Processing:**
- DOCX - Microsoft Word
- DOC - Legacy Word (via conversion)
- ODT - OpenDocument Text
- RTF - Rich Text Format

**Spreadsheets:**
- XLSX - Microsoft Excel
- XLS - Legacy Excel (via conversion)
- ODS - OpenDocument Spreadsheet
- CSV - Comma Separated Values

**Presentations:**
- PPTX - Microsoft PowerPoint
- PPT - Legacy PowerPoint (via conversion)
- ODP - OpenDocument Presentation

**Other:**
- PDF - Portable Document Format
- TXT - Plain Text
- MD - Markdown

### Import Process

**Import Single File:**
1. Click **Import** or press `Ctrl+O` / `Cmd+O`
2. Select file
3. File loads and parses
4. Preview displayed

**Import Multiple Files:**
1. **File** → **Import Multiple**
2. Select multiple files
3. Files added to queue
4. Process as batch

### Import Options

**General Options:**
- Preserve formatting
- Preserve styles
- Preserve images
- Convert formulas

**Advanced Options:**
- Custom encoding
- Page range selection
- Extract embedded images
- Extract metadata

## Format Conversion

### Target Formats

**Word Processing:**
- Vantis Writer (Markdown)
- Vantis Writer (Vantis Format)
- Plain Text
- HTML

**Spreadsheets:**
- Vantis Grid (JSON)
- Vantis Grid (CSV)
- Plain Text

**Presentations:**
- Vantis Canvas (JSON)
- Vantis Canvas (SVG)
- PDF

**Common:**
- JSON - Structured data
- XML - Markup format
- Plain Text

### Conversion Settings

**Vantis Writer (Markdown):**
- Standard Markdown
- GitHub Flavored Markdown
- MultiMarkdown
- Custom dialect

**Vantis Grid (JSON):**
- Preserve formulas
- Convert to values only
- Include formatting
- Include charts

**Vantis Canvas (JSON):**
- Preserve animations
- Preserve transitions
- Include embedded media
- Optimize images

### Conversion Quality

**High Quality:**
- Preserve all formatting
- Keep all features
- Larger file size
- Longer conversion time

**Balanced:**
- Preserve important formatting
- Remove unnecessary elements
- Moderate file size

**Optimized:**
- Minimal formatting
- Plain text focus
- Smallest file size
- Fastest conversion

## Sanitization

### Why Sanitize?

Sanitization removes potentially harmful or unwanted elements:

**Security Benefits:**
- Removes macros (potential malware)
- Removes scripts (JavaScript, VBA)
- Sanitizes embedded objects
- Removes external references

**Privacy Benefits:**
- Removes metadata (author, dates)
- Removes hidden text
- Removes tracked changes
- Removes comments

### Sanitization Options

**Remove Macros:**
- VBA macros in Office documents
- Automatically detected and removed
- Warning displayed

**Remove Scripts:**
- JavaScript in documents
- Excel scripts
- PowerPoint macros

**Sanitize Embedded Objects:**
- OLE objects
- Embedded files
- ActiveX controls
- Flash objects

**Remove Metadata:**
- Author information
- Creation/modification dates
- Document statistics
- Hidden properties

**Remove Comments:**
- Inline comments
- Review comments
- Track changes
- Annotations

### Sanitization Levels

**Basic:**
- Remove macros and scripts
- Remove external references
- Keep formatting intact

**Standard:**
- Basic level plus
- Remove metadata
- Remove embedded objects
- Keep comments

**Strict:**
- Standard level plus
- Remove comments
- Remove hidden content
- Plain text focus

### Sanitization Report

After sanitization:

**Report Contents:**
- Total items removed
- Type of items removed
- Security warnings
- Privacy issues found

**Review Report:**
1. Check what was removed
2. Verify no important content lost
3. Save report for reference

## Batch Processing

### Batch Import

**Import Multiple Files:**
1. **Batch** → **Import Folder**
2. Select folder
3. Files auto-detected
4. Add to queue

**Add to Queue:**
1. Select files
2. Drag and drop to queue
3. Files appear in list
4. Process sequentially

### Batch Configuration

**Global Settings:**
- Apply same settings to all files
- Or configure individually
- Save preset for reuse

**Queue Management:**
- Reorder files
- Remove files
- Pause/resume processing

### Batch Processing

**Start Processing:**
1. Configure settings
2. Click **Process Batch**
3. Progress bar shows:
   - Current file
   - Total progress
   - Time remaining
4. Process completes

**Process Options:**
- Stop on error or continue
- Log all operations
- Generate summary report

### Batch Results

**Summary Report:**
- Files processed
- Files failed
- Total time
- Errors and warnings

**Individual Reports:**
- Per-file conversion details
- Sanitization report
- Export location

## Export Options

### Export Settings

**Export Location:**
- Same directory as source
- Custom directory
- Subfolder (converted/)

**File Naming:**
- Keep original name
- Add suffix (_converted)
- Custom naming pattern

**Export Formats:**
- Same as target format
- Additional formats
- Multiple formats

### Export After Conversion

**Auto Export:**
- Export immediately after conversion
- Or review before export
- Preview option

**Export Formats:**
- Vantis formats
- Legacy formats
- Web formats
- Text formats

## Settings

### General Settings

**Application:**
- Default source directory
- Default export directory
- Auto-open after conversion
- Remember last used settings

**Performance:**
- Max concurrent processes
- Memory limit
- Cache size

### Conversion Defaults

**Default Target Format:**
- Vantis Writer (Markdown)
- Vantis Grid (JSON)
- Vantis Canvas (JSON)

**Default Sanitization:**
- Standard level
- Remove macros: Yes
- Remove scripts: Yes
- Remove metadata: Yes

### Quality Settings

**Image Quality:**
- Original quality
- Compressed (medium)
- Optimized (small)

**Formatting Preservation:**
- Full preservation
- Basic preservation
- Minimal preservation

### Notification Settings

**Completion:**
- Show notification when done
- Play sound
- Open export folder

**Errors:**
- Show error dialog
- Log errors only
- Stop on error

## Troubleshooting

### Common Issues

**Import Fails:**
- Check file format supported
- Verify file not corrupted
- Try opening in original application
- Check file permissions

**Conversion Errors:**
- Check target format options
- Reduce conversion quality
- Try different target format
- Review error message

**Formatting Lost:**
- Increase preservation level
- Check formatting compatibility
- Use alternative format
- Manual adjustments may be needed

**Batch Processing Issues:**
- Reduce concurrent processes
- Increase memory limit
- Check disk space
- Process in smaller batches

### Error Messages

**"Unsupported format":**
- File format not supported
- Convert to supported format first
- Check supported formats list

**"Conversion failed":**
- Check error details
- Try different conversion options
- Report issue with file

**"Sanitization removed critical content":**
- Review sanitization settings
- Adjust sanitization level
- Check sanitization report

### Getting Help

- Check documentation for specific features
- Review error logs
- Test with simple files
- Report issues at: https://github.com/vantisCorp/VantisOffice/issues

---

**Last Updated**: 2024-03-03  
**VantisBridge Version**: 0.2.0