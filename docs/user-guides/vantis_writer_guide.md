# Vantis Writer User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Creating Documents](#creating-documents)
4. [Editing Documents](#editing-documents)
5. [Markdown Support](#markdown-support)
6. [Typography Features](#typography-features)
7. [Deep Focus Mode](#deep-focus-mode)
8. [Export Options](#export-options)
9. [Keyboard Shortcuts](#keyboard-shortcuts)
10. [Tips and Tricks](#tips-and-tricks)

## Introduction

Vantis Writer is an advanced word processor designed for writers and content creators. It features native Markdown support, beautiful typography, and a distraction-free Deep Focus Mode.

**Key Features:**
- Native Markdown support with live preview
- Babel Typography for beautiful text rendering
- Deep Focus Mode for distraction-free writing
- Export to multiple formats (Vantis, PDF, HTML, Markdown)
- Rich text editing with styles and formatting

## Getting Started

### Launching Vantis Writer

```bash
cargo run --release -p vantis-writer
```

### Creating Your First Document

1. Launch Vantis Writer
2. Click "New Document" or press `Ctrl+N` / `Cmd+N`
3. Start typing your content

### Interface Overview

```
┌─────────────────────────────────────────────┐
│  File  Edit  View  Insert  Format  Help   │
├─────────────────────────────────────────────┤
│  New  Open  Save  Save As  Export  Print   │
├─────────────────────────────────────────────┤
│                                             │
│  Document Title                              │
│  ─────────────────────────────────────      │
│                                             │
│  Start typing your document here...         │
│                                             │
│                                             │
└─────────────────────────────────────────────┘
```

## Creating Documents

### New Document

Create a new document from scratch:

1. Click **File** → **New** or press `Ctrl+N` / `Cmd+N`
2. Start typing your content
3. Document is automatically saved to your workspace

### Document Structure

Vantis Writer documents use a simple structure:

```rust
Document {
    id: String,
    title: String,
    author: String,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
    paragraphs: Vec<Paragraph>,
    metadata: DocumentMetadata,
}
```

### Setting Document Properties

1. Click **File** → **Document Properties**
2. Set:
   - Title
   - Author
   - Language
   - Default font
   - Page size and margins

## Editing Documents

### Basic Text Editing

**Formatting Text:**
- Bold: Select text and press `Ctrl+B` / `Cmd+B`
- Italic: Select text and press `Ctrl+I` / `Cmd+I`
- Underline: Select text and press `Ctrl+U` / `Cmd+U`
- Strikethrough: Select text and press `Ctrl+Shift+S` / `Cmd+Shift+S`

**Aligning Text:**
- Left align: `Ctrl+L` / `Cmd+L`
- Center align: `Ctrl+E` / `Cmd+E`
- Right align: `Ctrl+R` / `Cmd+R`
- Justify: `Ctrl+J` / `Cmd+J`

**Paragraph Styles:**
- Heading 1: `Ctrl+Alt+1` / `Cmd+Opt+1`
- Heading 2: `Ctrl+Alt+2` / `Cmd+Opt+2`
- Heading 3: `Ctrl+Alt+3` / `Cmd+Opt+3`
- Normal: `Ctrl+Alt+0` / `Cmd+Opt+0`

### Lists

**Bulleted Lists:**
- Type `- ` followed by your text
- Or press `Ctrl+Shift+L` / `Cmd+Shift+L`

**Numbered Lists:**
- Type `1. ` followed by your text
- Or press `Ctrl+Shift+N` / `Cmd+Shift+N`

### Links and Images

**Insert Link:**
1. Select text or place cursor where link should go
2. Press `Ctrl+K` / `Cmd+K`
3. Enter URL and optional title

**Insert Image:**
1. Place cursor where image should go
2. Click **Insert** → **Image** or press `Ctrl+Shift+I` / `Cmd+Shift+I`
3. Select image file

## Markdown Support

Vantis Writer has native Markdown support with live preview.

### Basic Markdown Syntax

**Headers:**
```markdown
# Heading 1
## Heading 2
### Heading 3
```

**Emphasis:**
```markdown
*italic* or _italic_
**bold** or __bold__
***bold italic***
~~strikethrough~~
```

**Links:**
```markdown
[Link text](https://example.com)
```

**Images:**
```markdown
![Alt text](image.jpg)
```

**Code:**
```markdown
`inline code`
```
code block
```
```

**Tables:**
```markdown
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
```

### Using Markdown

1. Enable Markdown mode from the **View** menu
2. Type Markdown syntax directly
3. Preview updates automatically
4. Switch between edit and preview modes

## Typography Features

Vantis Writer uses Babel Typography for beautiful text rendering.

### Font Selection

1. Select text or place cursor
2. Click **Format** → **Font**
3. Choose from available fonts:
   - Serif: Georgia, Times New Roman, Garamond
   - Sans-serif: Arial, Helvetica, Roboto
   - Monospace: Courier New, Consolas, Fira Code

### Font Size

1. Select text
2. Use dropdown in toolbar or press:
   - Increase: `Ctrl+]` / `Cmd+]`
   - Decrease: `Ctrl+[` / `Cmd+[`

### Line Height

1. Select paragraphs
2. Choose line spacing from **Format** → **Paragraph**
3. Options: Single, 1.15, 1.5, Double

### Kerning and Tracking

Fine-tune character spacing:
1. Select text
2. Format → Typography → **Character Spacing**
3. Adjust kerning and tracking

## Deep Focus Mode

Deep Focus Mode helps you write without distractions.

### Activating Deep Focus Mode

1. Click **View** → **Deep Focus Mode**
2. Or press `Ctrl+Shift+F` / `Cmd+Shift+F`
3. Interface simplifies to show only your document

### Deep Focus Features

- **Dimmed UI**: Toolbar and menus fade away
- **Centered Text**: Document centered on screen
- **Focus Line**: Highlight current paragraph
- **Minimal Distractions**: Hide all UI elements

### Customizing Deep Focus Mode

1. Click **View** → **Deep Focus Settings**
2. Configure:
   - Background color
   - Text color
   - Focus line highlight color
   - Font size in focus mode
   - Automatic activation after period of inactivity

### Exiting Deep Focus Mode

Press `Esc` or `Ctrl+Shift+F` / `Cmd+Shift+F` to exit.

## Export Options

### Supported Formats

Vantis Writer supports multiple export formats:

1. **Vantis Format** (.vantis) - Native format with full features
2. **PDF** (.pdf) - Universal document format
3. **HTML** (.html) - Web-ready format
4. **Markdown** (.md) - Plain text format
5. **Plain Text** (.txt) - Basic text format

### Exporting Documents

1. Click **File** → **Export**
2. Choose format from dropdown
3. Select options:
   - Include metadata
   - Embed images
   - Custom styling
4. Click **Export** and choose location

### Export Options by Format

**PDF Export:**
- Page size (A4, Letter, Legal)
- Margins
- Include table of contents
- Embed fonts

**HTML Export:**
- Inline CSS or external stylesheet
- Include document metadata
- Responsive design
- Custom template

**Markdown Export:**
- Include frontmatter
- Preserve formatting
- Include image references

## Keyboard Shortcuts

### File Operations

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Document | `Ctrl+N` | `Cmd+N` |
| Open | `Ctrl+O` | `Cmd+O` |
| Save | `Ctrl+S` | `Cmd+S` |
| Save As | `Ctrl+Shift+S` | `Cmd+Shift+S` |
| Export | `Ctrl+E` | `Cmd+E` |
| Print | `Ctrl+P` | `Cmd+P` |

### Editing

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Undo | `Ctrl+Z` | `Cmd+Z` |
| Redo | `Ctrl+Y` | `Cmd+Y` |
| Cut | `Ctrl+X` | `Cmd+X` |
| Copy | `Ctrl+C` | `Cmd+C` |
| Paste | `Ctrl+V` | `Cmd+V` |
| Select All | `Ctrl+A` | `Cmd+A` |

### Formatting

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Bold | `Ctrl+B` | `Cmd+B` |
| Italic | `Ctrl+I` | `Cmd+I` |
| Underline | `Ctrl+U` | `Cmd+U` |
| Strikethrough | `Ctrl+Shift+S` | `Cmd+Shift+S` |

### Navigation

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Find | `Ctrl+F` | `Cmd+F` |
| Find Next | `Ctrl+G` | `Cmd+G` |
| Find Previous | `Ctrl+Shift+G` | `Cmd+Shift+G` |
| Go to Line | `Ctrl+G` then type line | `Cmd+G` then type line |

### View

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Deep Focus Mode | `Ctrl+Shift+F` | `Cmd+Shift+F` |
| Toggle Sidebar | `Ctrl+\` | `Cmd+\` |
| Zoom In | `Ctrl++` | `Cmd++` |
| Zoom Out | `Ctrl+-` | `Cmd+-` |
| Reset Zoom | `Ctrl+0` | `Cmd+0` |

## Tips and Tricks

### Writing Productivity

1. **Use Templates**: Save frequently used document structures as templates
2. **Quick Insert**: Type `::` to quickly insert:
   - Tables
   - Code blocks
   - Images
   - Quotes
3. **Autocomplete**: Start typing and use `Tab` to accept suggestions
4. **Word Count**: Check word count from status bar or `Ctrl+Shift+C` / `Cmd+Shift+C`

### Organizing Documents

1. **Use Headings**: Create structure with heading levels 1-3
2. **Outline View**: Toggle outline view from **View** menu
3. **Table of Contents**: Auto-generate from headings
4. **Document Map**: Navigate large documents with document map

### Formatting Tips

1. **Styles vs Direct Formatting**: Use paragraph styles for consistency
2. **Custom Styles**: Save frequently used formatting as custom styles
3. **Style Inspector**: Check and clean up formatting with style inspector
4. **Format Painter**: Copy formatting with `Ctrl+Shift+C` / `Cmd+Shift+C`, paste with `Ctrl+Shift+V` / `Cmd+Shift+V`

### Collaboration (Future)

1. **Vantis Link Integration**: Share documents with real-time collaboration
2. **Version History**: Track document changes over time
3. **Comments and Suggestions**: Add inline comments and suggested edits
4. **Track Changes**: Review and accept/reject changes

### Advanced Features

1. **Macros**: Record and replay repetitive tasks
2. **Plugins**: Extend functionality with plugins
3. **Custom Themes**: Create custom color schemes and themes
4. **Auto-save**: Configure auto-save intervals

## Troubleshooting

### Common Issues

**Text Formatting Issues**
- Clear formatting: `Ctrl+Space` / `Cmd+Space`
- Reset paragraph styles: **Format** → **Clear Styles**

**Export Problems**
- Verify export format supports needed features
- Check file permissions for save location
- Try a different export location

**Performance Issues**
- Close other applications
- Disable Deep Focus Mode animations
- Reduce font preview caching

### Getting Help

- Check individual sections for specific features
- View keyboard shortcuts for common tasks
- Report issues at: https://github.com/vantisCorp/VantisOffice/issues

---

**Last Updated**: 2024-03-03  
**VantisWriter Version**: 0.2.0