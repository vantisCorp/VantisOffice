# Vantis Grid User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Creating Spreadsheets](#creating-spreadsheets)
4. [Data Entry](#data-entry)
5. [Formulas and Functions](#formulas-and-functions)
6. [AI-Powered Features](#ai-powered-features)
7. [Charts and Visualization](#charts-and-visualization)
8. [Data Analysis](#data-analysis)
9. [Collaboration](#collaboration)
10. [Export Options](#export-options)
11. [Keyboard Shortcuts](#keyboard-shortcuts)
12. [Tips and Tricks](#tips-and-tricks)

## Introduction

Vantis Grid is an AI-powered spreadsheet application with Excel-compatible formulas, advanced data analysis, and real-time collaboration.

**Key Features:**
- Excel-compatible formula engine
- AI-powered trend analysis and anomaly detection
- Neural Engine for data predictions
- Support for 10GB+ datasets
- Real-time collaboration with Vantis Link
- Export to multiple formats (CSV, JSON, Excel, PDF)

## Getting Started

### Launching Vantis Grid

```bash
cargo run --release -p vantis-grid
```

### Creating Your First Spreadsheet

1. Launch Vantis Grid
2. Click "New Workbook" or press `Ctrl+N` / `Cmd+N`
3. Enter data in cells
4. Use formulas to perform calculations

### Interface Overview

```
┌──────────────────────────────────────────────────────────┐
│  File  Edit  Insert  Format  Data  View  Help         │
├──────────────────────────────────────────────────────────┤
│  │A│B│C│D│E│F│G│H│I│J│K│L│M│N│O│P│Q│R│S│T│U│V│W│X│Y│Z│  │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ 1│   │   │   │   │   │   │   │   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ 2│   │   │   │   │   │   │   │   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ 3│   │   │   │   │   │   │   │   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ 4│   │   │   │   │   │   │   │   │   │   │   │   │   │   │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
```

## Creating Spreadsheets

### New Workbook

Create a new workbook:

1. Click **File** → **New** or press `Ctrl+N` / `Cmd+N`
2. Workbook starts with 3 worksheets (default)
3. Name sheets by double-clicking the tab

### Managing Worksheets

**Add Worksheet:**
- Click **+** button next to sheet tabs
- Or press `Shift+F11`

**Rename Worksheet:**
- Double-click sheet tab
- Enter new name
- Press Enter

**Delete Worksheet:**
- Right-click sheet tab
- Select **Delete**
- Confirm deletion

**Move Worksheet:**
- Drag sheet tab to new position
- Or right-click → **Move or Copy**

## Data Entry

### Basic Cell Entry

1. Click on a cell to select it
2. Type your data
3. Press Enter to move down or Tab to move right
4. Press Esc to cancel entry

### Data Types

Vantis Grid supports these data types:

**Numbers:**
- Integers: `123`, `-456`
- Decimals: `3.14`, `0.005`
- Scientific notation: `1.23E5`, `4.56E-3`
- Currency: `$1,234.56`, `€789.00`
- Percentages: `25%`, `0.15%`

**Text:**
- Regular text: `Hello World`
- Text with numbers: `Product 123`
- Multi-line text: Press `Alt+Enter` for line breaks

**Dates and Times:**
- Dates: `2024-03-03`, `03/03/2024`
- Times: `14:30`, `2:30 PM`
- Date and Time: `2024-03-03 14:30`

**Boolean:**
- `TRUE` or `FALSE`
- Treated as 1 or 0 in formulas

**Formulas:**
- Start with `=` sign
- Example: `=SUM(A1:A10)`

### Cell References

**Relative References:**
- `A1`, `B2`, `C10`
- Change when copied

**Absolute References:**
- `$A$1`, `$B$2`
- Don't change when copied

**Mixed References:**
- `$A1` (column locked, row relative)
- `A$1` (column relative, row locked)

### Data Validation

Restrict data entered in cells:

1. Select cells
2. **Data** → **Data Validation**
3. Choose validation type:
   - Whole numbers
   - Decimal
   - List
   - Date
   - Time
   - Text length
   - Custom formula
4. Set constraints and error message

## Formulas and Functions

### Basic Formula Syntax

Formulas always start with `=`:

```
=SUM(A1:A10)
=A1*B2+C3
=IF(A1>100,"High","Low")
```

### Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `=A1+B1` |
| `-` | Subtraction | `=A1-B1` |
| `*` | Multiplication | `=A1*B1` |
| `/` | Division | `=A1/B1` |
| `^` | Exponentiation | `=A1^2` |
| `%` | Percentage | `=A1%` |

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `=` | Equal | `=A1=B1` |
| `>` | Greater than | `=A1>B1` |
| `<` | Less than | `=A1<B1` |
| `>=` | Greater or equal | `=A1>=B1` |
| `<=` | Less or equal | `=A1<=B1` |
| `<>` | Not equal | `=A1<>B1` |

### Common Functions

**Mathematical Functions:**
```
=SUM(A1:A10)        Sum of range
=AVERAGE(A1:A10)    Average of range
=MIN(A1:A10)        Minimum value
=MAX(A1:A10)        Maximum value
=COUNT(A1:A10)      Count of numbers
=ROUND(A1, 2)       Round to 2 decimals
=ABS(A1)            Absolute value
```

**Statistical Functions:**
```
=STDEV(A1:A10)      Standard deviation
=VAR(A1:A10)        Variance
=MEDIAN(A1:A10)     Median
=MODE(A1:A10)       Most frequent value
```

**Text Functions:**
```
=CONCATENATE(A1, B1)    Join text
=LEN(A1)                Text length
=UPPER(A1)              Uppercase
=LOWER(A1)              Lowercase
=TRIM(A1)               Remove extra spaces
=LEFT(A1, 5)            First 5 characters
=RIGHT(A1, 3)           Last 3 characters
=MID(A1, 2, 4)          4 characters from position 2
```

**Date and Time Functions:**
```
=TODAY()            Current date
=NOW()              Current date and time
=YEAR(A1)           Year from date
=MONTH(A1)          Month from date (1-12)
=DAY(A1)            Day from date
=DATE(2024, 3, 3)   Create date from numbers
```

**Logical Functions:**
```
=IF(A1>100,"Yes","No")      Conditional
=AND(A1>10, B1<20)          All conditions true
=OR(A1>10, B1<20)           Any condition true
=NOT(A1>10)                 Negation
```

**Lookup Functions:**
```
=VLOOKUP(A1, B1:D10, 2, FALSE)    Vertical lookup
=HLOOKUP(A1, B1:D10, 2, FALSE)    Horizontal lookup
=INDEX(A1:B10, 2, 1)              Value at position
=MATCH(A1, A1:A10, 0)             Position of value
```

### Array Formulas

Perform calculations on arrays:

```
{=SUM(A1:A10 * B1:B10)}     Element-wise multiplication
{=TRANSPOSE(A1:B5)}         Transpose array
```

Press `Ctrl+Shift+Enter` to enter array formulas.

## AI-Powered Features

Vantis Grid includes AI features powered by the Neural Engine.

### Trend Analysis

Analyze data trends automatically:

1. Select data range
2. **Data** → **Analyze** → **Trend Analysis**
3. Choose trend type:
   - Linear
   - Polynomial
   - Exponential
   - Moving Average
4. View trend line and equation
5. Get future predictions

**Example:**
```
Data: Sales data for 12 months
Trend: Linear regression shows 5% monthly growth
Prediction: Next month sales = Current × 1.05
```

### Anomaly Detection

Find outliers in your data:

1. Select data range
2. **Data** → **Analyze** → **Detect Anomalies**
3. Set sensitivity (Low, Medium, High)
4. Review detected anomalies
5. Choose to:
   - Keep (valid outlier)
   - Remove (error)
   - Investigate (requires manual review)

### Data Predictions

Predict future values:

1. Select historical data
2. **Data** → **Predict** → **Next Values**
3. Choose prediction model
4. Set number of predictions
5. View confidence intervals

### AI Suggestions

Get intelligent suggestions:

1. Start typing in a cell
2. Vantis Grid suggests:
   - Auto-complete values
   - Formula suggestions
   - Chart recommendations
   - Data cleaning suggestions
3. Press `Tab` to accept suggestion

## Charts and Visualization

### Creating Charts

1. Select data range
2. **Insert** → **Chart**
3. Choose chart type:
   - Column/Bar chart
   - Line chart
   - Pie chart
   - Scatter plot
   - Area chart
   - Combo chart
4. Customize chart

### Chart Types

**Column/Bar Charts:**
- Vertical columns for comparing categories
- Horizontal bars for long labels
- Clustered, stacked, or 100% stacked

**Line Charts:**
- Trends over time
- Multiple data series
- With or without markers

**Pie Charts:**
- Parts of a whole
- Maximum 7-8 categories
- Donut or pie style

**Scatter Plots:**
- Correlation between variables
- X-Y data points
- Trend lines

**Area Charts:**
- Cumulative totals over time
- Stacked or overlapping

### Customizing Charts

**Chart Elements:**
- Add title: **Chart Design** → **Add Chart Element** → **Chart Title**
- Add axis labels: **Chart Design** → **Add Chart Element** → **Axis Titles**
- Add legend: **Chart Design** → **Add Chart Element** → **Legend**
- Add data labels: **Chart Design** → **Add Chart Element** → **Data Labels**

**Chart Styles:**
- Apply style: **Chart Design** → **Chart Styles**
- Change colors: **Chart Design** → **Change Colors**
- Switch row/column: **Chart Design** → **Switch Row/Column**

**Chart Filters:**
- Filter data: **Chart Design** → **Select Data**
- Edit data series
- Add or remove data

## Data Analysis

### Sorting and Filtering

**Sort Data:**
1. Select range
2. **Data** → **Sort**
3. Sort by column, direction (ascending/descending)
4. Add multiple sort levels

**Filter Data:**
1. Select range
2. **Data** → **Filter**
3. Click filter arrows in header row
4. Check/uncheck items to filter
5. Use text, number, or date filters

**Advanced Filters:**
1. **Data** → **Advanced**
2. Set criteria range
3. Choose filter action
4. Copy to another location or filter in place

### Pivot Tables

Create summary tables from data:

1. Select data range
2. **Insert** → **Pivot Table**
3. Choose destination (new worksheet or existing)
4. Drag fields to:
   - Rows
   - Columns
   - Values
   - Filters
5. Customize calculations (Sum, Count, Average, etc.)

### Conditional Formatting

Highlight cells based on conditions:

1. Select cells
2. **Home** → **Conditional Formatting**
3. Choose rule type:
   - Highlight cells rules (greater than, less than, etc.)
   - Top/Bottom rules
   - Data bars
   - Color scales
   - Icon sets
4. Set conditions and formatting
5. Click OK

### Data Consolidation

Combine data from multiple ranges:

1. **Data** → **Consolidate**
2. Select function (Sum, Count, Average, etc.)
3. Add ranges to consolidate
4. Check "Create links to source data" if needed
5. Click OK

## Collaboration

Vantis Grid integrates with Vantis Link for real-time collaboration.

### Start Collaborating

1. **File** → **Share**
2. Invite collaborators via email or link
3. Set permissions:
   - View only
   - Edit
   - Admin
4. Share link or send invitations

### Real-Time Editing

- See collaborators' cursors
- View changes in real-time
- Chat with collaborators
- Track who made which changes

### Conflict Resolution

When multiple users edit the same cell:

1. Automatic merge when possible
2. Manual resolution for conflicts
3. View all versions of a cell
4. Accept/reject changes

### Version History

Track changes over time:

1. **File** → **Version History**
2. View all saved versions
3. Compare versions
4. Restore previous version

## Export Options

### Supported Formats

1. **Vantis Format** (.vantis) - Native format
2. **CSV** (.csv) - Comma-separated values
3. **JSON** (.json) - Structured data format
4. **Excel** (.xlsx) - Microsoft Excel format (placeholder)
5. **PDF** (.pdf) - Portable document format (placeholder)

### Exporting Data

1. **File** → **Export**
2. Choose format
3. Select export options:
   - Include formulas or values only
   - Include formatting
   - Include charts
   - Specific sheets or all sheets
4. Choose location
5. Click **Export**

### Importing Data

**Import from CSV:**
1. **File** → **Import** → **CSV**
2. Select file
3. Set delimiter (comma, tab, semicolon)
4. Preview data
5. Set column types
6. Click **Import**

**Import from JSON:**
1. **File** → **Import** → **JSON**
2. Select file
3. Preview structure
4. Map JSON fields to columns
5. Click **Import**

## Keyboard Shortcuts

### Cell Navigation

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Move right | `→` | `→` |
| Move left | `←` | `←` |
| Move down | `↓` | `↓` |
| Move up | `↑` | `↑` |
| Move to start of row | `Home` | `Home` |
| Move to end of row | `End` | `End` |
| Move to start of sheet | `Ctrl+Home` | `Cmd+Home` |
| Move to end of sheet | `Ctrl+End` | `Cmd+End` |
| Go to cell | `Ctrl+G` | `Cmd+G` |

### Cell Selection

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Select cell | Click cell | Click cell |
| Select range | Click and drag | Click and drag |
| Select row | Click row number | Click row number |
| Select column | Click column letter | Click column letter |
| Select all | `Ctrl+A` | `Cmd+A` |
| Extend selection | `Shift+Arrows` | `Shift+Arrows` |

### Editing

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Edit cell | `F2` | `F2` |
| Enter formula | `=` then formula | `=` then formula |
| Cancel edit | `Esc` | `Esc` |
| Copy | `Ctrl+C` | `Cmd+C` |
| Cut | `Ctrl+X` | `Cmd+X` |
| Paste | `Ctrl+V` | `Cmd+V` |
| Paste special | `Ctrl+Alt+V` | `Cmd+Ctrl+V` |
| Fill down | `Ctrl+D` | `Cmd+D` |
| Fill right | `Ctrl+R` | `Cmd+R` |

### Formatting

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Bold | `Ctrl+B` | `Cmd+B` |
| Italic | `Ctrl+I` | `Cmd+I` |
| Underline | `Ctrl+U` | `Cmd+U` |
| Number format | `Ctrl+Shift+1` | `Cmd+Shift+1` |
| Currency format | `Ctrl+Shift+4` | `Cmd+Shift+4` |
| Percentage format | `Ctrl+Shift+5` | `Cmd+Shift+5` |
| Date format | `Ctrl+Shift+3` | `Cmd+Shift+3` |

## Tips and Tricks

### Data Entry Tips

1. **Auto-complete**: Start typing to auto-complete from column
2. **Flash Fill**: Extract data patterns automatically
3. **Quick Fill**: Drag fill handle to extend series
4. **Data Validation**: Use validation to prevent errors

### Formula Tips

1. **Formula Auto-complete**: Start typing function name to see suggestions
2. **Evaluate Formula**: Step through formula calculation
3. **Trace Precedents**: See cells referenced by formula
4. **Trace Dependents**: See cells that reference selected cell
5. **Watch Window**: Monitor values of specific cells

### Performance Tips

1. **Large Datasets**: Vantis Grid supports 10GB+ datasets
2. **Formula Optimization**: Use efficient formulas for large ranges
3. **Conditional Formatting**: Use sparingly for large ranges
4. **Array Formulas**: Avoid unnecessary array formulas

### Visualization Tips

1. **Chart Selection**: Choose right chart type for your data
2. **Chart Simplicity**: Keep charts simple and clear
3. **Color Usage**: Use colors meaningfully
4. **Data Labels**: Use labels only when necessary
5. **Sparklines**: Add mini charts to cells

### Analysis Tips

1. **Clean Data First**: Ensure data is clean before analysis
2. **Use Named Ranges**: Give meaningful names to ranges
3. **Document Your Work**: Add comments and notes
4. **Test Assumptions**: Validate your analysis results

### AI Features Tips

1. **Trend Analysis**: Use for time-series data with clear patterns
2. **Anomaly Detection**: Adjust sensitivity based on data variability
3. **Predictions**: Use sufficient historical data for accuracy
4. **Suggestions**: Review AI suggestions before accepting

## Troubleshooting

### Common Issues

**Formula Errors:**
- `#DIV/0!`: Division by zero
- `#N/A`: Value not available
- `#NAME?`: Invalid function or name
- `#NULL!`: Intersection of two areas doesn't exist
- `#NUM!`: Invalid number
- `#REF!`: Invalid cell reference
- `#VALUE!`: Wrong type of argument

**Fix Formula Errors:**
1. Click cell with error
2. Review formula
3. Check cell references
4. Verify function syntax
5. Use **Evaluate Formula** to debug

**Import/Export Issues:**
- Check file format compatibility
- Verify delimiter settings for CSV
- Ensure proper encoding for text files
- Check file permissions

**Performance Issues:**
- Reduce conditional formatting
- Simplify complex formulas
- Remove unnecessary calculations
- Close other applications

### Getting Help

- Use formula auto-complete for function syntax
- Check function reference in help
- Review error messages carefully
- Report issues at: https://github.com/vantisCorp/VantisOffice/issues

---

**Last Updated**: 2024-03-03  
**VantisGrid Version**: 0.2.0