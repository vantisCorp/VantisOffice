# Vantis Flow User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Mind Maps](#mind-maps)
4. [Flowcharts](#flowcharts)
5. [Gantt Charts](#gantt-charts)
6. [Kanban Boards](#kanban-boards)
7. [Task Management](#task-management)
8. [Collaboration](#collaboration)
9. [Export Options](#export-options)
10. [Keyboard Shortcuts](#keyboard-shortcuts)
11. [Tips and Tricks](#tips-and-tricks)

## Introduction

Vantis Flow is a planning and diagramming tool with mind maps, flowcharts, Gantt charts, and Kanban boards.

**Key Features:**
- Mind maps with automatic layout (Radial, Tree, Left-Right)
- Flowcharts with intelligent routing
- Gantt charts for project timelines
- Kanban boards for task management
- Real-time collaboration with Vantis Link
- Export to SVG, JSON, and more

## Getting Started

### Launching Vantis Flow

```bash
cargo run --release -p vantis-flow
```

### Creating Your First Canvas

1. Launch Vantis Flow
2. Click "New Canvas" or press `Ctrl+N` / `Cmd+N`
3. Choose canvas type:
   - Mind Map
   - Flowchart
   - Gantt Chart
   - Kanban Board
4. Start adding elements

### Interface Overview

```
┌─────────────────────────────────────────────────────┐
│  File  Edit  View  Insert  Format  Tools  Help    │
├─────────────────────────────────────────────────────┤
│  [New] [Open] [Save] [Export] [Share]             │
├─────────────────────────────────────────────────────┤
│  Tools: [Select] [Mind Map] [Flowchart] [Gantt]   │
├─────────────────────────────────────────────────────┤
│                                             [Zoom]  │
│  ┌──────────────────────────────────┐       │ ▲   │
│  │                                  │       ◄──┼──►│
│  │        Canvas Area               │       │ ▼   │
│  │                                  │              │
│  │       ┌─────────┐                │              │
│  │       │ Central │                │              │
│  │       │  Idea   │                │              │
│  │       └─────────┘                │              │
│  │                                  │              │
│  └──────────────────────────────────┘              │
│                                                   │
├─────────────────────────────────────────────────────┤
│  [Properties] [Styles] [Layers] [History]         │
└─────────────────────────────────────────────────────┘
```

## Mind Maps

### Creating Mind Maps

**New Mind Map:**
1. **File** → **New** → **Mind Map**
2. Central node appears
3. Add child nodes
4. Build your mind map

**Adding Nodes:**
1. Select node
2. Press `Tab` to add child
3. Or `Enter` to add sibling
4. Type node text
5. Press Enter to confirm

### Mind Map Layouts

**Radial Layout:**
- Center node in middle
- Children radiate outward
- Good for brainstorming

**Tree Layout:**
- Root at top
- Children below
- Good for hierarchies

**Left-Right Layout:**
- Root on left
- Children flow right
- Good for processes

**Apply Layout:**
1. **Format** → **Layout**
2. Choose layout type
3. Apply to selected or entire map

### Mind Map Styles

**Node Styles:**
1. Select node(s)
2. **Format** → **Node Style**
3. Choose:
   - Shape (rectangle, rounded, circle, ellipse)
   - Fill color
   - Border color/width
   - Font family/size/color

**Connection Styles:**
1. Select connection(s)
2. **Format** → **Connection Style**
3. Choose:
   - Line style (straight, curved, angled)
   - Line width
   - Line color
   - Arrow style

### Mind Map Operations

**Edit Node:**
- Double-click to edit text
- Press `F2` to edit
- Press Enter to confirm

**Delete Node:**
- Select node
- Press `Del`
- Children move up or delete

**Move Node:**
- Drag node to new parent
- Children move with it
- Layout updates automatically

**Collapse/Expand:**
- Click `+`/`-` icon on node
- Or press `Space`

## Flowcharts

### Creating Flowcharts

**New Flowchart:**
1. **File** → **New** → **Flowchart**
2. Choose starter template or blank
3. Add shapes and connections

**Adding Shapes:**
1. Drag shape from palette
2. Or double-click canvas
3. Choose shape type
4. Position and resize

### Flowchart Shapes

**Basic Shapes:**
- Rectangle: Process
- Rounded Rectangle: Start/End
- Diamond: Decision
- Parallelogram: Input/Output
- Oval: Terminal
- Circle: Connector

**Advanced Shapes:**
- Document
- Data
- Manual Input
- Preparation
- Database
- Card

### Flowchart Connections

**Connect Shapes:**
1. Select connection tool
2. Click source shape
3. Drag to target shape
4. Release to connect

**Connection Properties:**
1. Select connection
2. **Format** → **Connection**
3. Configure:
   - Line style (straight, angled, curved)
   - Arrow heads (start, end, both, none)
   - Label text
   - Color and width

**Intelligent Routing:**
- Connections avoid shapes
- Right-angle routing
- Optimal path calculated
- Drag to adjust

### Flowchart Validation

**Validate Flowchart:**
1. **Tools** → **Validate**
2. Check for:
   - Unconnected shapes
   - Multiple starts/ends
   - Dead ends
   - Infinite loops

## Gantt Charts

### Creating Gantt Charts

**New Gantt Chart:**
1. **File** → **New** → **Gantt Chart**
2. Add tasks
3. Set dependencies
4. View timeline

### Task Management

**Add Task:**
1. Click `+` in task list
2. Enter task details:
   - Name
   - Start date
   - End date
   - Duration
   - Assignee
   - Progress
3. Task appears on timeline

**Task Properties:**
- Name: Task description
- Start: Start date
- End: End date
- Duration: Auto-calculated or manual
- Assignee: Person responsible
- Progress: 0-100%
- Dependencies: Predecessor tasks
- Color: Task bar color

### Dependencies

**Add Dependency:**
1. Select task
2. Drag from predecessor
3. Drop on successor
4. Dependency line appears

**Dependency Types:**
- Finish-to-Start (FS): Predecessor finishes before successor starts
- Start-to-Start (SS): Both start together
- Finish-to-Finish (FF): Both finish together
- Start-to-Finish (SF): Successor finishes when predecessor starts

**Edit Dependency:**
1. Double-click dependency line
2. Change type or lag
3. Click OK

### Timeline View

**Navigate Timeline:**
- Scroll horizontally for time
- Scroll vertically for tasks
- Zoom in/out with `Ctrl++`/`Ctrl+-`

**Timeline Scale:**
- Days
- Weeks
- Months
- Quarters

**Show/Hide Elements:**
- Show weekends
- Show holidays
- Show milestones
- Show dependencies

### Milestones

**Add Milestone:**
1. Right-click timeline
2. **Add Milestone**
3. Set date and name
4. Diamond marker appears

**Milestone Properties:**
- Name
- Date
- Color
- Notes

## Kanban Boards

### Creating Kanban Boards

**New Kanban:**
1. **File** → **New** → **Kanban Board**
2. Default columns: To Do, In Progress, Done
3. Add and customize columns

### Columns

**Add Column:**
1. Click `+ Add Column`
2. Enter column name
3. Set WIP limit (Work in Progress)
4. Choose color

**Column Settings:**
- Name
- WIP Limit: Maximum cards
- Color
- Auto-move on due date

**Delete Column:**
1. Click column menu (⋮)
2. **Delete Column**
3. Choose to delete or move cards

### Cards

**Add Card:**
1. Click `+` in column
2. Enter card title
3. Add details:
   - Description
   - Assignee
   - Due date
   - Tags
   - Priority

**Card Properties:**
- Title
- Description
- Assignee(s)
- Due date
- Tags
- Priority (Low, Medium, High, Urgent)
- Attachments
- Comments

**Move Cards:**
- Drag card to different column
- Card moves immediately
- Activity logged

### Kanban Settings

**Swimlanes:**
1. **View** → **Swimlanes**
2. Group by:
   - Assignee
   - Priority
   - Tag
   - Custom

**Quick Filters:**
1. Click filter icon
2. Filter by:
   - Assignee
   - Due date
   - Tag
   - Priority

## Task Management

### Task Properties

**Basic Properties:**
- Name: Task title
- Description: Detailed description
- Status: Not Started, In Progress, Completed, Blocked
- Priority: Low, Medium, High, Urgent
- Assignee: Person responsible
- Start Date: When work begins
- Due Date: Deadline
- Tags: Categories/labels

### Task Operations

**Create Task:**
1. **Insert** → **New Task**
2. Fill in properties
3. Save

**Edit Task:**
1. Double-click task
2. Modify properties
3. Save changes

**Delete Task:**
1. Select task
2. Press `Del`
3. Confirm deletion

**Duplicate Task:**
1. Select task
2. **Edit** → **Duplicate**
3. Modify copy as needed

### Task Dependencies

**Create Dependency:**
1. Select task
2. **Edit** → **Dependencies**
3. Add predecessor/successor
4. Set dependency type

**Dependency Types:**
- Finish-to-Start (FS)
- Start-to-Start (SS)
- Finish-to-Finish (FF)
- Start-to-Finish (SF)

## Collaboration

### Real-Time Collaboration

**Share Canvas:**
1. **File** → **Share**
2. Generate share link
3. Send to collaborators
4. Work together in real-time

**See Collaborators:**
- Cursors show in real-time
- Selections highlighted
- Changes sync automatically

**Comments:**
1. Select element
2. **Insert** → **Comment**
3. Type comment
4. Collaborators can reply

### Version History

**View History:**
1. **View** → **History**
2. See all changes
3. Restore previous version

## Export Options

### Supported Formats

1. **SVG** (.svg) - Vector graphics
2. **JSON** (.json) - Native format
3. **PNG** (.png) - Raster image
4. **PDF** (.pdf) - Document format

### Exporting

1. **File** → **Export**
2. Choose format
3. Set options:
   - Include/exclude grid
   - Page size (for PDF/PNG)
   - Resolution
4. Export

## Keyboard Shortcuts

### General

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Canvas | `Ctrl+N` | `Cmd+N` |
| Open | `Ctrl+O` | `Cmd+O` |
| Save | `Ctrl+S` | `Cmd+S` |
| Undo | `Ctrl+Z` | `Cmd+Z` |
| Redo | `Ctrl+Y` | `Cmd+Y` |
| Delete | `Del` | `Del` |

### Mind Map

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Add Child | `Tab` | `Tab` |
| Add Sibling | `Enter` | `Enter` |
| Edit Node | `F2` | `F2` |
| Collapse/Expand | `Space` | `Space` |

### Navigation

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Zoom In | `Ctrl++` | `Cmd++` |
| Zoom Out | `Ctrl+-` | `Cmd+-` |
| Fit to Screen | `Ctrl+0` | `Cmd+0` |
| Pan | `Space + Drag` | `Space + Drag` |

## Tips and Tricks

### Mind Map Tips

1. **Use Keyboard Shortcuts**: Faster than mouse
2. **Organize Hierarchically**: 5-9 children per node max
3. **Use Colors**: Color-code related nodes
4. **Add Notes**: Double-click for detailed notes
5. **Export Regularly**: Save backups

### Flowchart Tips

1. **Start Simple**: Draft before detailing
2. **Use Alignment**: Keep diagram neat
3. **Label Connections**: Add flow descriptions
4. **Validate**: Check for errors before finalizing
5. **Use Swimlanes**: Show responsibilities

### Gantt Chart Tips

1. **Set Dependencies**: Show task relationships
2. **Add Milestones**: Mark key dates
3. **Update Progress**: Keep chart current
4. **Use Critical Path**: Identify longest path
5. **Share with Team**: Keep everyone informed

### Kanban Tips

1. **Limit WIP**: Enforce work-in-progress limits
2. **Daily Standups**: Review board daily
3. **Move Cards Promptly**: Update status quickly
4. **Use Tags**: Categorize for filtering
5. **Review Regularly**: Archive completed cards

## Troubleshooting

### Common Issues

**Mind Map Layout Issues:**
- Reset layout: **Format** → **Layout** → **Reset**
- Check for circular references
- Reduce node count

**Flowchart Connection Issues:**
- Check routing mode
- Adjust shapes to avoid overlaps
- Use manual routing

**Gantt Chart Display Issues:**
- Check date ranges
- Verify dependencies
- Adjust zoom level

**Kanban Not Syncing:**
- Check network connection
- Verify session is active
- Refresh page

### Getting Help

- Check specific feature documentation
- Review keyboard shortcuts
- Report issues at: https://github.com/vantisCorp/VantisOffice/issues

---

**Last Updated**: 2024-03-03  
**VantisFlow Version**: 0.2.0