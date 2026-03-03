# Vantis Canvas User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Canvas Navigation](#canvas-navigation)
4. [Working with Slides](#working-with-slides)
5. [Shapes and Objects](#shapes-and-objects)
6. [Text Elements](#text-elements)
7. [Images](#images)
8. [Animations and Transitions](#animations-and-transitions)
9. [Layers and Grouping](#layers-and-grouping)
10. [Effects and Styles](#effects-and-styles)
11. [Export Options](#export-options)
12. [Keyboard Shortcuts](#keyboard-shortcuts)
13. [Tips and Tricks](#tips-and-tricks)

## Introduction

Vantis Canvas is a 3D-accelerated presentation application featuring an infinite canvas for non-linear navigation, GPU-accelerated rendering at 120Hz, and advanced animations.

**Key Features:**
- Infinite Canvas for non-linear navigation
- GPU-accelerated rendering (120Hz)
- 30+ easing functions for animations
- Export to SVG, PDF, PNG, PowerPoint
- Real-time collaboration with Vantis Link

## Getting Started

### Launching Vantis Canvas

```bash
cargo run --release -p vantis-canvas
```

### Creating Your First Presentation

1. Launch Vantis Canvas
2. Click "New Canvas" or press `Ctrl+N` / `Cmd+N`
3. Add slides and content
4. Apply animations and transitions
5. Export your presentation

### Interface Overview

```
┌─────────────────────────────────────────────────────┐
│  File  Edit  View  Insert  Format  Slideshow  Help │
├─────────────────────────────────────────────────────┤
│  New  Open  Save  Export  Slideshow  Present       │
├─────────────────────────────────────────────────────┤
│  │ [Slide 1] │ [Slide 2] │ [Slide 3] │ ...       │
├─────────────────────────────────────────────────────┤
│                                             [Zoom]  │
│                                             │ ▲   │
│  ┌──────────────────────────────────┐       ◄──┼──►│
│  │                                  │       │ ▼   │
│  │        Infinite Canvas            │              │
│  │                                  │              │
│  │                                  │              │
│  │                                  │              │
│  │                                  │              │
│  └──────────────────────────────────┘              │
│                                             [Fit]  │
│                                                   │
├─────────────────────────────────────────────────────┤
│  [Shape] [Text] [Image] [Line] [Effects] [Layers]│
└─────────────────────────────────────────────────────┘
```

## Canvas Navigation

### Infinite Canvas Navigation

Vantis Canvas features an infinite canvas allowing non-linear presentation flow.

**Navigation Methods:**

**Mouse Navigation:**
- Middle-click and drag to pan
- Scroll wheel to zoom in/out
- Ctrl+scroll to zoom precisely
- Shift+scroll to pan horizontally

**Keyboard Navigation:**
- Arrow keys: Pan the canvas
- `+` / `-`: Zoom in/out
- `Ctrl+0`: Fit canvas to screen
- `Ctrl+1`: Zoom to 100%
- `Ctrl+2`: Zoom to 200%
- `Ctrl+5`: Zoom to 50%

**Navigator Panel:**
- Open from **View** → **Navigator**
- Shows mini map of entire canvas
- Click and drag to navigate
- Zoom slider for quick zoom control

### Canvas Views

**Fit to Screen:**
1. **View** → **Fit to Screen**
2. Or press `Ctrl+0` / `Cmd+0`

**Zoom to 100%:**
1. **View** → **Zoom 100%**
2. Or press `Ctrl+1` / `Cmd+1`

**Custom Zoom:**
1. Use zoom dropdown in toolbar
2. Or press `Ctrl++` / `Cmd++` to zoom in
3. Or press `Ctrl+-` / `Cmd+-` to zoom out

### Slide Navigation

**Next/Previous Slide:**
- Next: `Space`, `Enter`, `→`, or click Next button
- Previous: `Backspace`, `←`, or click Previous button
- Go to specific slide: `Ctrl+G` / `Cmd+G`

**Slide Sorter:**
- **View** → **Slide Sorter**
- Thumbnail view of all slides
- Drag and drop to reorder
- Double-click to edit slide

## Working with Slides

### Creating Slides

**New Slide:**
1. **Insert** → **New Slide**
2. Or press `Ctrl+M` / `Cmd+M`
3. Choose slide layout (blank, title, content, etc.)

**Duplicate Slide:**
1. Select slide in slide sorter
2. **Edit** → **Duplicate**
3. Or press `Ctrl+D` / `Cmd+D`

**Delete Slide:**
1. Select slide
2. **Edit** → **Delete**
3. Or press `Del`

**Reorder Slides:**
1. Switch to slide sorter view
2. Drag slide to new position
3. Drop to reorder

### Slide Properties

**Slide Settings:**
1. Select slide
2. **Format** → **Slide Properties**
3. Configure:
   - Slide dimensions (1920x1080, 1280x720, etc.)
   - Background color or image
   - Orientation (landscape, portrait)
   - Transition effect

**Slide Layouts:**
- Blank
- Title Slide
- Title and Content
- Two Content
- Comparison
- Title Only
- Section Header

## Shapes and Objects

### Adding Shapes

1. **Insert** → **Shape**
2. Choose shape type:
   - Basic: Rectangle, Circle, Ellipse, Triangle
   - Lines: Line, Arrow, Double Arrow
   - Polygons: Polygon, Star, Hexagon
   - Paths: Freehand, Bezier Curve
3. Click and drag on canvas to draw

### Shape Types

**Basic Shapes:**
- Rectangle
- Circle / Ellipse
- Triangle
- Polygon (custom sides)
- Star (custom points)
- Diamond
- Trapezoid

**Lines and Arrows:**
- Line
- Arrow (single, double)
- Elbow connector
- Curved connector
- Freehand line

**Polygons:**
- Pentagon
- Hexagon
- Octagon
- Custom polygon

**Paths:**
- Freehand drawing
- Bezier curves
- Custom path

### Shape Properties

**Position and Size:**
- Drag to move
- Drag corner handles to resize
- **Format** → **Position and Size**
- Enter exact dimensions in pixels

**Colors:**
- Fill color
- Stroke (border) color
- **Format** → **Shape Style** → **Colors**

**Borders:**
- Border width
- Border style (solid, dashed, dotted)
- **Format** → **Shape Style** → **Border**

**Rotation:**
- Drag rotation handle (green circle)
- **Format** → **Rotate**
- Enter angle in degrees

**Alignment:**
- **Format** → **Align**
- Align left, center, right
- Align top, middle, bottom
- Distribute horizontally/vertically

### Shape Effects

**Shadow:**
1. Select shape
2. **Format** → **Effects** → **Shadow**
3. Configure:
   - Color
   - Blur radius
   - Offset (X, Y)
   - Opacity

**Glow:**
1. Select shape
2. **Format** → **Effects** → **Glow**
3. Configure:
   - Color
   - Size
   - Opacity

**Blur:**
1. Select shape
2. **Format** → **Effects** → **Blur**
3. Set blur radius

**Blend Modes:**
- Normal
- Multiply
- Screen
- Overlay
- Darken
- Lighten

## Text Elements

### Adding Text

**Text Box:**
1. **Insert** → **Text**
2. Click and drag on canvas
3. Type your text

**Title/Headings:**
1. Choose heading style from toolbar
2. H1, H2, H3, H4, H5, H6

### Text Formatting

**Font Settings:**
1. Select text
2. **Format** → **Font**
3. Configure:
   - Font family (Roboto, Arial, Georgia, etc.)
   - Font size
   - Color
   - Bold, Italic, Underline

**Paragraph Formatting:**
1. Select paragraph
2. **Format** → **Paragraph**
3. Configure:
   - Alignment (left, center, right, justify)
   - Line height
   - Letter spacing
   - Paragraph spacing

**Text Styles:**
- Title
- Subtitle
- Body text
- Caption
- Quote

### Text Effects

**Shadow:**
1. Select text
2. **Format** → **Text Effects** → **Shadow**
3. Configure shadow settings

**Glow:**
1. Select text
2. **Format** → **Text Effects** → **Glow**
3. Configure glow settings

**Outline:**
1. Select text
2. **Format** → **Text Effects** → **Outline**
3. Configure outline color and width

## Images

### Adding Images

**Insert from File:**
1. **Insert** → **Image** → **From File**
2. Select image file
3. Position on canvas

**Insert from Clipboard:**
1. Copy image to clipboard
2. **Insert** → **Image** → **From Clipboard**
3. Or press `Ctrl+V` / `Cmd+V`

### Image Properties

**Position and Size:**
- Drag to move
- Drag handles to resize
- Hold `Shift` to maintain aspect ratio

**Cropping:**
1. Select image
2. **Format** → **Crop**
3. Drag crop handles
4. Press Enter to apply

**Adjustments:**
1. Select image
2. **Format** → **Image Adjustments**
3. Configure:
   - Brightness
   - Contrast
   - Saturation
   - Exposure
   - Gamma

### Image Effects

**Filters:**
1. Select image
2. **Format** → **Image Filters**
3. Choose filter:
   - None
   - Grayscale
   - Sepia
   - Invert
   - Blur
   - Sharpen

**Effects:**
- Shadow
- Glow
- Reflection
- Border frame

## Animations and Transitions

### Adding Animations

1. Select object
2. **Format** → **Animations**
3. Choose animation type:
   - Entrance (Fade, Slide, Zoom, etc.)
   - Emphasis (Spin, Pulse, etc.)
   - Exit (Fade, Slide, Zoom, etc.)
4. Set timing and options

### Animation Types

**Entrance Animations:**
- Fade
- Slide (Left, Right, Up, Down)
- Zoom (In, Out)
- Rotate
- Scale
- Move (to position)
- Wipe

**Emphasis Animations:**
- Pulse
- Spin
- Bounce
- Shake
- Color Change
- Size Change

**Exit Animations:**
- Fade
- Slide
- Zoom
- Rotate
- Shrink

**Motion Paths:**
- Line
- Arc
- Custom path

### Easing Functions

Vantis Canvas supports 30+ easing functions:

**Linear:**
- Linear

**Ease In:**
- EaseInQuad
- EaseInCubic
- EaseInQuart
- EaseInQuint
- EaseInExpo
- EaseInSine

**Ease Out:**
- EaseOutQuad
- EaseOutCubic
- EaseOutQuart
- EaseOutQuint
- EaseOutExpo
- EaseOutSine

**Ease In Out:**
- EaseInOutQuad
- EaseInOutCubic
- EaseInOutQuart
- EaseInOutQuint
- EaseInOutExpo
- EaseInOutSine

**Special:**
- EaseInElastic
- EaseOutElastic
- EaseInOutElastic
- EaseInBounce
- EaseOutBounce
- EaseInOutBounce

**How to Use:**
1. Select animation
2. **Animation Options** → **Easing**
3. Choose easing function from dropdown
4. Adjust duration (0.1s to 10s)

### Transitions

**Slide Transitions:**
1. Select slide
2. **Format** → **Slide Transition**
3. Choose transition:
   - None
   - Fade
   - Slide (Left, Right, Up, Down)
   - Zoom (In, Out)
   - Wipe
   - Dissolve
4. Set duration (0.1s to 2s)

**Transition Options:**
- Duration
- Direction
- Smooth timing

### Animation Timeline

1. **View** → **Animation Timeline**
2. See all animations on slide
3. Drag to reorder animations
4. Adjust timing by dragging
5. Click play to preview

## Layers and Grouping

### Using Layers

**Layers Panel:**
1. **View** → **Layers**
2. See all objects on slide
3. Click to select
4. Drag to reorder layer

**Layer Operations:**
- Bring to front
- Send to back
- Bring forward
- Send backward

**Locking Layers:**
1. Select layer
2. Right-click → **Lock**
3. Or click lock icon in Layers panel
4. Lock icon appears on layer

**Hiding Layers:**
1. Select layer
2. Right-click → **Hide**
3. Or click eye icon in Layers panel
4. Layer is hidden from view

### Grouping Objects

**Group Objects:**
1. Select multiple objects (Ctrl+click / Cmd+click)
2. **Format** → **Group**
3. Or press `Ctrl+G` / `Cmd+G`
4. Objects move together

**Ungroup:**
1. Select group
2. **Format** → **Ungroup**
3. Or press `Ctrl+Shift+G` / `Cmd+Shift+G`

### Aligning and Distributing

**Align Objects:**
1. Select multiple objects
2. **Format** → **Align**
3. Choose alignment:
   - Align Left
   - Align Center
   - Align Right
   - Align Top
   - Align Middle
   - Align Bottom

**Distribute Objects:**
1. Select 3+ objects
2. **Format** → **Distribute**
3. Choose:
   - Distribute Horizontally
   - Distribute Vertically

## Effects and Styles

### Shape Styles

**Quick Styles:**
1. Select shape
2. **Format** → **Quick Styles**
3. Choose pre-defined style:
   - Simple
   - Gradient
   - Pattern
   - 3D effects

**Custom Styles:**
1. Select shape
2. **Format** → **Shape Style**
3. Configure:
   - Fill color/gradient
   - Border color/width
   - Effects (shadow, glow, etc.)

### Color Themes

**Applying Themes:**
1. **Design** → **Themes**
2. Choose theme
3. Theme applies colors and fonts

**Custom Theme:**
1. **Design** → **Themes** → **Customize**
2. Set accent colors
3. Choose fonts
4. Save theme

## Export Options

### Supported Formats

1. **Vantis Format** (.vantis) - Native format with full features
2. **SVG** (.svg) - Vector graphics format
3. **JSON** (.json) - Structured data format
4. **PDF** (.pdf) - Portable document format (placeholder)
5. **PNG** (.png) - Raster image format (placeholder)
6. **PowerPoint** (.pptx) - Microsoft PowerPoint format (placeholder)

### Exporting Presentations

1. **File** → **Export**
2. Choose format
3. Select options:
   - All slides or selected range
   - Include animations
   - Include transitions
   - High resolution (for images)
4. Click **Export**
5. Choose location and filename

### Export Settings by Format

**SVG Export:**
- Vector graphics only
- Include fonts (embedded or converted to paths)
- Compress SVG

**JSON Export:**
- Include all properties
- Include metadata
- Pretty print or compact

**PDF Export:**
- Page size (A4, Letter, Custom)
- Include notes
- High quality or optimized size
- Include background images

**PNG Export:**
- Resolution (72, 150, 300 DPI)
- Transparent background
- Each slide as separate file

## Keyboard Shortcuts

### File Operations

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Canvas | `Ctrl+N` | `Cmd+N` |
| Open | `Ctrl+O` | `Cmd+O` |
| Save | `Ctrl+S` | `Cmd+S` |
| Save As | `Ctrl+Shift+S` | `Cmd+Shift+S` |
| Export | `Ctrl+E` | `Cmd+E` |
| Print | `Ctrl+P` | `Cmd+P` |

### Canvas Navigation

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Pan | Middle-click + drag | Middle-click + drag |
| Zoom In | `Ctrl++` | `Cmd++` |
| Zoom Out | `Ctrl+-` | `Cmd+-` |
| Fit to Screen | `Ctrl+0` | `Cmd+0` |
| Zoom 100% | `Ctrl+1` | `Cmd+1` |
| Zoom 200% | `Ctrl+2` | `Cmd+2` |
| Zoom 50% | `Ctrl+5` | `Cmd+5` |

### Slide Operations

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Slide | `Ctrl+M` | `Cmd+M` |
| Duplicate Slide | `Ctrl+D` | `Cmd+D` |
| Delete Slide | `Del` | `Del` |
| Next Slide | `Space`, `Enter`, `→` | `Space`, `Enter`, `→` |
| Previous Slide | `Backspace`, `←` | `Backspace`, `←` |

### Editing

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Undo | `Ctrl+Z` | `Cmd+Z` |
| Redo | `Ctrl+Y` | `Cmd+Y` |
| Copy | `Ctrl+C` | `Cmd+C` |
| Cut | `Ctrl+X` | `Cmd+X` |
| Paste | `Ctrl+V` | `Cmd+V` |
| Select All | `Ctrl+A` | `Cmd+A` |
| Delete | `Del` | `Del` |

### View

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Slideshow Mode | `F5` | `F5` |
| Exit Slideshow | `Esc` | `Esc` |
| Full Screen | `F11` | `Ctrl+Cmd+F` |
| Toggle Rulers | `Ctrl+R` | `Cmd+R` |
| Toggle Grid | `Ctrl+G` | `Cmd+G` |
| Toggle Guides | `Ctrl+;` | `Cmd+;` |

## Tips and Tricks

### Design Tips

1. **Keep it Simple**: Less is more in presentations
2. **Use Hierarchy**: Guide audience through information
3. **Consistent Style**: Use consistent fonts and colors
4. **Visual Balance**: Balance elements on each slide
5. **High Contrast**: Ensure text is readable

### Animation Tips

1. **Subtle Animations**: Use gentle transitions
2. **Purposeful Motion**: Only animate for emphasis
3. **Consistent Easing**: Use similar easing functions
4. **Test Timing**: Ensure animations don't drag
5. **Preview Often**: Check how animations look

### Performance Tips

1. **Optimize Images**: Use appropriately sized images
2. **Limit Effects**: Too many effects slow performance
3. **Use Shapes**: Prefer shapes over images when possible
4. **Test on Hardware**: Test presentation target device

### Collaboration Tips

1. **Share Early**: Get feedback during development
2. **Version Control**: Keep backup of important versions
3. **Clear Communication**: Discuss changes with collaborators
4. **Review Together**: Present together to discuss content

### Presentation Tips

1. **Practice**: Rehearse your presentation
2. **Know Your Content**: Be familiar with all slides
3. **Engage Audience**: Make eye contact, ask questions
4. **Use Notes**: Keep notes for each slide
5. **Have Backup**: Bring backup copy of presentation

## Troubleshooting

### Common Issues

**Canvas Performance:**
- Reduce number of objects
- Simplify complex shapes
- Reduce effects
- Check system resources

**Animation Issues:**
- Check easing function settings
- Verify animation timing
- Test animation order
- Check for conflicts

**Export Issues:**
- Verify format compatibility
- Check export options
- Ensure sufficient disk space
- Try different format if needed

**Display Issues:**
- Check display scaling
- Update graphics drivers
- Adjust zoom level
- Check color profile

### Getting Help

- Check documentation for specific features
- Review keyboard shortcuts for navigation
- Report issues at: https://github.com/vantisCorp/VantisOffice/issues

---

**Last Updated**: 2024-03-03  
**VantisCanvas Version**: 0.2.0