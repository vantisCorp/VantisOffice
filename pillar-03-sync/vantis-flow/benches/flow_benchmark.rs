// Performance benchmarks for Vantis Flow
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use uuid::Uuid;
use vantis_flow::{
    Canvas, Color, Connection, ConnectionType, Element, ElementType, Flowchart, FlowchartNode,
    FlowchartNodeType, GanttChart, KanbanBoard, MindMap, Project, Stroke, Style, Task,
    TaskPriority, TaskStatus,
};

fn benchmark_canvas_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("canvas_creation");

    group.bench_function("empty_canvas", |b| {
        b.iter(|| black_box(Canvas::new("Test Canvas")))
    });

    group.bench_function("canvas_with_elements", |b| {
        b.iter(|| {
            let mut canvas = Canvas::new("Test Canvas");
            for i in 0..10 {
                let element = Element::new(ElementType::Rectangle, i as f64 * 100.0, 100.0)
                    .with_text(format!("Element {}", i));
                canvas.add_element(element).unwrap();
            }
            black_box(canvas)
        })
    });

    group.finish();
}

fn benchmark_element_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("element_creation");

    let element_types = [
        ElementType::Rectangle,
        ElementType::Circle,
        ElementType::Diamond,
        ElementType::RoundedRectangle,
        ElementType::Hexagon,
        ElementType::Triangle,
        ElementType::Star,
        ElementType::Arrow,
        ElementType::Text,
    ];

    for (idx, element_type) in element_types.iter().enumerate() {
        let name = format!("{:?}", element_type).to_lowercase();
        group.bench_with_input(BenchmarkId::new("type", name), &idx, |b, _| {
            b.iter(|| {
                let element = Element::new(element_type.clone(), 100.0, 100.0)
                    .with_text("Test Element")
                    .with_size(150.0, 80.0);
                black_box(element)
            })
        });
    }

    group.finish();
}

fn benchmark_element_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("element_operations");

    group.bench_function("move_element", |b| {
        b.iter(|| {
            let mut element = Element::new(ElementType::Rectangle, 100.0, 100.0);
            element.move_to(200.0, 200.0);
            black_box(element)
        })
    });

    group.bench_function("resize_element", |b| {
        b.iter(|| {
            let mut element = Element::new(ElementType::Rectangle, 100.0, 100.0);
            element.resize(200.0, 150.0);
            black_box(element)
        })
    });

    group.finish();
}

fn benchmark_connection_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("connection_creation");

    let source_id = Uuid::new_v4();
    let target_id = Uuid::new_v4();

    let connection_types = [
        ConnectionType::Straight,
        ConnectionType::Orthogonal,
        ConnectionType::Curved,
        ConnectionType::Step,
    ];

    for conn_type in connection_types {
        let name = format!("{:?}", conn_type).to_lowercase();
        group.bench_with_input(
            BenchmarkId::new("type", name),
            &conn_type,
            |b, conn_type| {
                b.iter(|| {
                    let connection = Connection::new(source_id, target_id, conn_type.clone())
                        .with_label("Test Connection");
                    black_box(connection)
                })
            },
        );
    }

    group.finish();
}

fn benchmark_color_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("color_operations");

    group.bench_function("create_color", |b| {
        b.iter(|| black_box(Color::rgb(128, 64, 255)))
    });

    group.bench_function("from_hex", |b| {
        b.iter(|| black_box(Color::from_hex("#FF5500").unwrap()))
    });

    group.bench_function("to_hex", |b| {
        let color = Color::rgb(255, 85, 0);
        b.iter(|| black_box(color.to_hex()))
    });

    // Predefined colors
    group.bench_function("predefined_colors", |b| {
        b.iter(|| {
            black_box((
                Color::WHITE,
                Color::BLACK,
                Color::RED,
                Color::GREEN,
                Color::BLUE,
            ))
        })
    });

    group.finish();
}

fn benchmark_style_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("style_operations");

    group.bench_function("default_style", |b| b.iter(|| black_box(Style::default())));

    group.bench_function("custom_stroke", |b| {
        b.iter(|| {
            let stroke = Stroke {
                color: Color::BLUE,
                width: 2.5,
                ..Default::default()
            };
            black_box(stroke)
        })
    });

    group.finish();
}

fn benchmark_mindmap_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("mindmap_creation");

    group.bench_function("create_mindmap", |b| {
        b.iter(|| {
            let mindmap = MindMap::new("Test Mind Map", "Root Node");
            black_box(mindmap)
        })
    });

    group.finish();
}

fn benchmark_flowchart_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("flowchart_creation");

    group.bench_function("empty_flowchart", |b| {
        b.iter(|| {
            let flowchart = Flowchart::new("Test Flowchart");
            black_box(flowchart)
        })
    });

    group.bench_function("flowchart_with_nodes", |b| {
        b.iter(|| {
            let mut flowchart = Flowchart::new("Test Flowchart");

            let start = FlowchartNode::new("Start", 100.0, 100.0, FlowchartNodeType::Start);
            flowchart.nodes.insert(start.id, start);

            let process1 =
                FlowchartNode::new("Process 1", 100.0, 200.0, FlowchartNodeType::Process);
            flowchart.nodes.insert(process1.id, process1);

            let decision =
                FlowchartNode::new("Decision?", 100.0, 300.0, FlowchartNodeType::Decision);
            flowchart.nodes.insert(decision.id, decision);

            let process2 =
                FlowchartNode::new("Process 2", 100.0, 400.0, FlowchartNodeType::Process);
            flowchart.nodes.insert(process2.id, process2);

            let end = FlowchartNode::new("End", 100.0, 500.0, FlowchartNodeType::End);
            flowchart.nodes.insert(end.id, end);

            black_box(flowchart)
        })
    });

    group.finish();
}

fn benchmark_flowchart_node_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("flowchart_node_creation");

    let node_types = [
        FlowchartNodeType::Start,
        FlowchartNodeType::End,
        FlowchartNodeType::Process,
        FlowchartNodeType::Decision,
        FlowchartNodeType::Input,
        FlowchartNodeType::Output,
        FlowchartNodeType::Connector,
    ];

    for node_type in node_types {
        let name = format!("{:?}", node_type).to_lowercase();
        group.bench_with_input(
            BenchmarkId::new("type", name),
            &node_type,
            |b, node_type| {
                b.iter(|| {
                    let node = FlowchartNode::new("Test Node", 100.0, 100.0, node_type.clone());
                    black_box(node)
                })
            },
        );
    }

    group.finish();
}

fn benchmark_task_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("task_creation");

    group.bench_function("create_task", |b| {
        b.iter(|| {
            let task = Task::new("Test Task");
            black_box(task)
        })
    });

    group.bench_function("task_with_priority", |b| {
        b.iter(|| {
            let mut task = Task::new("High Priority Task");
            task.set_priority(TaskPriority::High);
            black_box(task)
        })
    });

    group.bench_function("task_with_status", |b| {
        b.iter(|| {
            let mut task = Task::new("In Progress Task");
            task.set_status(TaskStatus::InProgress);
            black_box(task)
        })
    });

    group.finish();
}

fn benchmark_project_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("project_creation");
    let owner_id = Uuid::new_v4();

    group.bench_function("empty_project", |b| {
        b.iter(|| {
            let project = Project::new("Test Project", owner_id);
            black_box(project)
        })
    });

    group.bench_function("project_with_tasks", |b| {
        b.iter(|| {
            let mut project = Project::new("Test Project", owner_id);
            for i in 0..10 {
                let mut task = Task::new(format!("Task {}", i));
                task.set_priority(if i % 3 == 0 {
                    TaskPriority::High
                } else {
                    TaskPriority::Medium
                });
                project.add_task(task).unwrap();
            }
            black_box(project)
        })
    });

    group.finish();
}

fn benchmark_kanban_board(c: &mut Criterion) {
    let mut group = c.benchmark_group("kanban_board");

    group.bench_function("create_kanban", |b| {
        b.iter(|| {
            let kanban = KanbanBoard::new("Test Kanban");
            black_box(kanban)
        })
    });

    group.finish();
}

fn benchmark_gantt_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("gantt_chart");

    group.bench_function("create_gantt", |b| {
        let project_id = Uuid::new_v4();
        b.iter(|| {
            let gantt = GanttChart::new("Test Gantt", project_id);
            black_box(gantt)
        })
    });

    group.finish();
}

fn benchmark_large_canvas(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_canvas");

    group.bench_function("add_many_elements", |b| {
        b.iter(|| {
            let mut canvas = Canvas::new("Large Canvas");
            for i in 0..100 {
                let x = (i % 10) as f64 * 150.0;
                let y = (i / 10) as f64 * 100.0;
                let element =
                    Element::new(ElementType::Rectangle, x, y).with_text(format!("Element {}", i));
                canvas.add_element(element).unwrap();
            }
            black_box(canvas)
        })
    });

    group.bench_function("canvas_with_connections", |b| {
        b.iter(|| {
            let mut canvas = Canvas::new("Connected Canvas");

            // Create elements
            let mut element_ids = Vec::new();
            for i in 0..20 {
                let element = Element::new(ElementType::Rectangle, i as f64 * 150.0, 100.0);
                element_ids.push(element.id);
                canvas.add_element(element).unwrap();
            }

            // Create connections
            for i in 0..element_ids.len() - 1 {
                let connection =
                    Connection::new(element_ids[i], element_ids[i + 1], ConnectionType::Straight);
                canvas.add_connection(connection).unwrap();
            }

            black_box(canvas)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_canvas_creation,
    benchmark_element_creation,
    benchmark_element_operations,
    benchmark_connection_creation,
    benchmark_color_operations,
    benchmark_style_operations,
    benchmark_mindmap_creation,
    benchmark_flowchart_creation,
    benchmark_flowchart_node_creation,
    benchmark_task_creation,
    benchmark_project_creation,
    benchmark_kanban_board,
    benchmark_gantt_chart,
    benchmark_large_canvas
);
criterion_main!(benches);
