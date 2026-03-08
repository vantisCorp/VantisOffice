# Theming Guide

## Overview

VantisOffice supports custom themes for personalizing the user interface. This guide covers how to create, install, and customize themes.

## Built-in Themes

### Available Themes
- **Light** - Default light theme
- **Dark** - Default dark theme
- **High Contrast** - Accessibility-focused theme
- **Nord** - Arctic, bluish color palette
- **Dracula** - Dark theme with purple accents
- **Solarized** - Precision color scheme

### Switching Themes

```bash
# Via CLI
vantisoffice config set theme dark

# Via GUI
Settings > Appearance > Theme
```

## Creating Custom Themes

### Theme Structure

```json
{
  "name": "my-theme",
  "version": "1.0.0",
  "author": "Your Name",
  "colors": {
    "background": "#1e1e1e",
    "foreground": "#d4d4d4",
    "accent": "#569cd6",
    "error": "#f44747",
    "success": "#4ec9b0",
    "warning": "#dcdcaa",
    "sidebar": "#252526",
    "border": "#3c3c3c"
  },
  "fonts": {
    "ui": "Inter, sans-serif",
    "mono": "JetBrains Mono, monospace",
    "document": "Libre Office Serif"
  }
}
```

### Theme File Location

```
~/.config/vantisoffice/themes/my-theme.json
```

### Creating a Theme

1. Create theme directory:
   ```bash
   mkdir -p ~/.config/vantisoffice/themes
   ```

2. Create theme file:
   ```bash
   touch ~/.config/vantisoffice/themes/my-theme.json
   ```

3. Edit theme file with your colors

4. Apply theme:
   ```bash
   vantisoffice config set theme my-theme
   ```

## Color Reference

### UI Elements

| Element | Description |
|---------|-------------|
| `background` | Main background color |
| `foreground` | Main text color |
| `accent` | Primary accent color |
| `sidebar` | Sidebar background |
| `border` | Border color |
| `selection` | Text selection color |
| `hover` | Hover state color |
| `active` | Active element color |
| `disabled` | Disabled element color |

### Status Colors

| Element | Description |
|---------|-------------|
| `error` | Error messages |
| `warning` | Warning messages |
| `success` | Success messages |
| `info` | Information messages |

## Advanced Theming

### CSS Custom Properties

For advanced customization, you can use CSS:

```css
/* ~/.config/vantisoffice/themes/custom.css */

:root {
  --font-size-base: 14px;
  --border-radius: 8px;
  --transition-speed: 200ms;
}

.toolbar {
  background: var(--accent);
  backdrop-filter: blur(10px);
}
```

### Icon Themes

```bash
# Install icon theme
vantisoffice icons install my-icons

# Use icon theme
vantisoffice config set icons my-icons
```

## Sharing Themes

### Package Theme

```bash
# Create theme package
vantisoffice theme pack my-theme

# Output: my-theme.vot (VantisOffice Theme)
```

### Install Shared Theme

```bash
# From file
vantisoffice theme install my-theme.vot

# From URL
vantisoffice theme install https://example.com/themes/my-theme.vot
```

## Troubleshooting

### Theme Not Loading
1. Check JSON syntax
2. Verify file location
3. Check file permissions

### Colors Not Applied
1. Restart application
2. Clear cache: `vantisoffice cache clear`
3. Check for conflicts with system theme