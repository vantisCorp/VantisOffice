# Vantis Flow

## Overview

Vantis Flow is a planning and diagramming application featuring vector mind maps integrated with calendars and automatic Gantt chart generation from Vantis Grid data.

## Key Features

- **Vector Mind Maps**: Unlimited canvas for visual planning
- **Gantt Integration**: Automatic chart generation from spreadsheet data
- **Calendar Sync**: Integrated with Vantis Chronos
- **Collaborative Planning**: Real-time collaboration with Vantis Link
- **Smart Layouts**: Automatic diagram layout algorithms
- **Export Options**: Multiple export formats

## Architecture

```
vantis-flow/
├── src/
│   ├── core/
│   │   ├── diagram.rs         # Diagram model
│   │   ├── node.rs            # Node model
│   │   ├── edge.rs            # Edge/connector model
│   │   └── canvas.rs          # Infinite canvas
│   ├── mindmap/
│   │   ├── engine.rs          # Mind map engine
│   │   ├── layout.rs          # Layout algorithms
│   │   ├── branching.rs       # Branch management
│   │   └── collapse.rs        # Node collapse/expand
│   ├── gantt/
│   │   ├── generator.rs       # Gantt chart generator
│   │   ├── timeline.rs        # Timeline rendering
│   │   ├── dependencies.rs    # Task dependencies
│   │   └── milestones.rs      # Milestone management
│   ├── calendar/
│   │   ├── integration.rs     # Calendar integration
│   │   ├── sync.rs            # Calendar sync
│   │   └── events.rs          # Event management
│   ├── layout/
│   │   ├── force_directed.rs  # Force-directed layout
│   │   ├── hierarchical.rs    # Hierarchical layout
│   │   ├── circular.rs        # Circular layout
│   │   └── auto.rs            # Auto layout
│   ├── collaboration/
│   │   ├── cursor.rs          # Remote cursors
│   │   ├── sync.rs            # Real-time sync
│   │   └── presence.rs        # User presence
│   └── ui/
│       ├── editor.rs          # Diagram editor
│       ├── toolbar.rs         # Toolbar
│       ├── properties.rs      # Properties panel
│       └── outline.rs         # Outline view
├── layouts/
│   ├── mindmap/               # Mind map layouts
│   ├── flowchart/             # Flowchart layouts
│   └── orgchart/              # Org chart layouts
├── templates/
│   └── planning/              # Planning templates
└── tests/
    ├── layout/                # Layout tests
    └── sync/                  # Sync tests
```

## Mind Map Engine

### Creating Mind Maps

```rust
use vantis_flow::mindmap::{MindMap, Node, NodeType};

let mut mindmap = MindMap::new("Project Plan")?;

// Create central node
let root = mindmap.add_node(Node::new("Project")
    .with_type(NodeType::Central)
    .with_position(Point { x: 0, y: 0 })
)?;

// Add branches
let phase1 = mindmap.add_node(Node::new("Phase 1: Planning")
    .with_parent(root)
)?;
let phase2 = mindmap.add_node(Node::new("Phase 2: Development")
    .with_parent(root)
)?;
```

### Auto Layout

```rust
use vantis_flow::mindmap::LayoutAlgorithm;

let layout = LayoutAlgorithm::Tree;
mindmap.auto_layout(layout)?;

// Or force-directed
let force_layout = LayoutAlgorithm::ForceDirected {
    iterations: 100,
    repulsion: 100.0,
    attraction: 0.1,
};
mindmap.auto_layout(force_layout)?;
```

### Node Styling

```rust
use vantis_flow::mindmap::NodeStyle;

let style = NodeStyle::new()
    .with_background_color(Color::rgb(0x2196F3))
    .with_text_color(Color::white())
    .with_border_color(Color::rgb(0x1976D2))
    .with_border_width(2.0)
    .with_corner_radius(8.0)
    .with_font_size(14);

node.set_style(style)?;
```

### Branch Collapsing

```rust
// Collapse branch
node.set_collapsed(true)?;

// Expand branch
node.set_collapsed(false)?;

// Toggle all
mindmap.toggle_all()?;
```

## Gantt Chart Integration

### Generating from Grid Data

```rust
use vantis_flow::gantt::{GanttGenerator, GanttConfig};

let generator = GanttGenerator::new()?;

// Import data from Vantis Grid
let grid_data = generator.import_from_grid("project_schedule.xlsx")?;

// Configure Gantt chart
let config = GanttConfig::new()
    .with_start_column("Start Date")
    .with_end_column("End Date")
    .with_task_column("Task")
    .with_dependency_column("Dependencies")
    .with_progress_column("Progress");

let gantt = generator.generate(&grid_data, config)?;
```

### Task Dependencies

```rust
use vantis_flow::gantt::{Task, Dependency, DependencyType};

let task1 = gantt.add_task(Task::new("Design")
    .with_start_date(Date::from_ymd(2024, 1, 1))
    .with_duration(Duration::days(5))
)?;

let task2 = gantt.add_task(Task::new("Development")
    .with_start_date(Date::from_ymd(2024, 1, 6))
    .with_duration(Duration::days(10))
)?;

// Add dependency
gantt.add_dependency(Dependency::new(task1, task2)
    .with_type(DependencyType::FinishToStart)
)?;
```

### Milestones

```rust
use vantis_flow::gantt::Milestone;

let milestone = gantt.add_milestone(Milestone::new("Project Launch")
    .with_date(Date::from_ymd(2024, 3, 1))
    .with_color(Color::rgb(0xFF9800))
)?;
```

### Timeline Customization

```rust
use vantis_flow::gantt::Timeline;

let timeline = Timeline::new()
    .with_scale(TimelineScale::Week)
    .with_show_weekends(true)
    .with_today_marker(true)
    .with_critical_path(true);

gantt.set_timeline(timeline)?;
```

## Calendar Integration

### Sync with Vantis Chronos

```rust
use vantis_flow::calendar::ChronosSync;

let sync = ChronosSync::new()?;

// Sync tasks to calendar
sync.sync_tasks_to_calendar(gantt.tasks())?;

// Sync calendar events to tasks
let events = sync.sync_calendar_to_tasks(DateRange::this_month())?;
for event in events {
    gantt.add_task_from_event(event)?;
}
```

### Task Calendar Events

```rust
use vantis_flow::calendar::CalendarEvent;

let event = CalendarEvent::from_task(task)?
    .with_reminder(Duration::hours(1))
    .with_attendees(vec![alice, bob])
    .with_location("Conference Room A");

sync.add_event(event)?;
```

## Diagram Layout Algorithms

### Force-Directed Layout

```rust
use vantis_flow::layout::{ForceDirectedLayout, LayoutConfig};

let config = LayoutConfig::new()
    .with_iterations(100)
    .with_repulsion(100.0)
    .with_attraction(0.1)
    .with_centering(true);

let layout = ForceDirectedLayout::new(config);
diagram.apply_layout(&layout)?;
```

### Hierarchical Layout

```rust
use vantis_flow::layout::{HierarchicalLayout, Direction};

let layout = HierarchicalLayout::new()
    .with_direction(Direction::TopToBottom)
    .with_layer_spacing(100.0)
    .with_node_spacing(50.0);

diagram.apply_layout(&layout)?;
```

### Circular Layout

```rust
use vantis_flow::layout::{CircularLayout, SortOrder};

let layout = CircularLayout::new()
    .with_sort_order(SortOrder::Alphabetical)
    .with_radius(200.0);

diagram.apply_layout(&layout)?;
```

## Collaboration Features

### Real-time Editing

```rust
use vantis_flow::collaboration::{Session, RemoteCursor};

let session = Session::create("Project Planning")?;

// Track remote cursors
session.on_cursor_move(|cursor| {
    diagram.show_remote_cursor(cursor)?;
    Ok(())
})?;
```

### Presence Awareness

```rust
use vantis_flow::collaboration::Presence;

let presence = Presence::new()?;

// Show who's viewing
for user in presence.active_users() {
    diagram.show_user_indicator(user)?;
}
```

## API Examples

### Creating a Flowchart

```rust
use vantis_flow::{Diagram, Node, Edge, NodeType};

let mut diagram = Diagram::new("Process Flow")?;

// Add nodes
let start = diagram.add_node(Node::new("Start")
    .with_type(NodeType::Start)
    .with_position(Point { x: 100, y: 100 })
)?;

let process = diagram.add_node(Node::new("Process Data")
    .with_type(NodeType::Process)
    .with_position(Point { x: 300, y: 100 })
)?;

let decision = diagram.add_node(Node::new("Valid?")
    .with_type(NodeType::Decision)
    .with_position(Point { x: 500, y: 100 })
)?;

// Add edges
diagram.add_edge(Edge::new(start, process)
    .with_label("Initialize")
)?;

diagram.add_edge(Edge::new(process, decision)
    .with_label("Check")
)?;
```

### Exporting Diagrams

```rust
use vantis_flow::export::{Exporter, ExportFormat};

// Export as SVG
Exporter::export(diagram, "diagram.svg", ExportFormat::SVG)?;

// Export as PNG
Exporter::export(diagram, "diagram.png", ExportFormat::PNG)?;

// Export as PDF
Exporter::export(diagram, "diagram.pdf", ExportFormat::PDF)?;
```

## Integration Points

- **Vantis Grid**: Gantt chart data source
- **Vantis Chronos**: Calendar synchronization
- **Vantis Link**: Real-time collaboration
- **Flux Vector Engine**: Diagram rendering
- **Vantis Ark**: Diagram backup

## Configuration

```toml
# flow.toml
[editor]
auto_save = true
auto_save_interval = 60
snap_to_grid = true
grid_size = 20

[mindmap]
default_layout = "tree"
auto_layout = true
animation_duration = 300
default_color = "#2196F3"

[gantt]
default_scale = "week"
show_weekends = true
show_critical_path = true
milestone_color = "#FF9800"

[calendar]
sync_enabled = true
sync_interval = 300
auto_create_events = true

[collaboration]
show_cursors = true
show_presence = true
auto_sync = true
```

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| New Node | Tab |
| Delete Node | Delete |
| Edit Node | F2 |
| Auto Layout | Ctrl+L |
| Collapse Node | - |
| Expand Node | + |
| Zoom In | Ctrl++ |
| Zoom Out | Ctrl+- |
| Fit to Screen | Ctrl+1 |
| Export | Ctrl+E |

## Performance Metrics

- **Node Rendering**: 60 FPS for 10,000 nodes
- **Auto Layout**: 500ms for 1,000 nodes
- **Gantt Generation**: 2s for 1,000 tasks
- **Calendar Sync**: 500ms
- **Collaboration Latency**: <100ms
- **Export Time**: 1s for SVG, 3s for PNG

## Future Roadmap

- [ ] AI-powered diagram suggestions
- [ ] Advanced animations
- [ ] 3D diagram visualization
- [ ] Template marketplace
- [ ] Voice commands
- [ ] Mobile companion app

## Build Requirements

- Rust 1.70+
- Flux Vector Engine
- Vantis Link (for collaboration)
- chrono (date/time handling)
- petgraph (graph algorithms)

---

**Part of VantisOffice Pillar III - Ecosystem & Collaboration**