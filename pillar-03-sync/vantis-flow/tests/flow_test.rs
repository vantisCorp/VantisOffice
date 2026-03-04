//! Comprehensive tests for Vantis Flow module

use vantis_flow::{
    core::*,
    planning::*,
    FlowResult,
};
use uuid::Uuid;
use chrono::{Utc, Duration};

// ============================================================================
// Canvas Tests
// ============================================================================

#[test]
fn test_canvas_creation() {
    let canvas = Canvas::new("Test Canvas");
    
    assert_eq!(canvas.name, "Test Canvas");
    assert!(canvas.description.is_none());
    assert!(canvas.elements.is_empty());
    assert!(canvas.connections.is_empty());
    assert_eq!(canvas.width, 1920.0);
    assert_eq!(canvas.height, 1080.0);
}

#[test]
fn test_canvas_resize() {
    let mut canvas = Canvas::new("Test Canvas");
    canvas.resize(2560.0, 1440.0);
    
    assert_eq!(canvas.width, 2560.0);
    assert_eq!(canvas.height, 1440.0);
}

#[test]
fn test_canvas_add_element() -> FlowResult<()> {
    let mut canvas = Canvas::new("Test Canvas");
    let element = Element::new(ElementType::Rectangle, 100.0, 100.0);
    
    canvas.add_element(element)?;
    
    assert_eq!(canvas.elements.len(), 1);
    Ok(())
}

#[test]
fn test_canvas_remove_element() -> FlowResult<()> {
    let mut canvas = Canvas::new("Test Canvas");
    let element = Element::new(ElementType::Rectangle, 100.0, 100.0);
    let element_id = element.id;
    
    canvas.add_element(element)?;
    canvas.remove_element(element_id)?;
    
    assert_eq!(canvas.elements.len(), 0);
    Ok(())
}

#[test]
fn test_canvas_get_element() -> FlowResult<()> {
    let mut canvas = Canvas::new("Test Canvas");
    let element = Element::new(ElementType::Rectangle, 100.0, 100.0);
    let element_id = element.id;
    
    canvas.add_element(element)?;
    
    let retrieved = canvas.get_element(element_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, element_id);
    Ok(())
}

#[test]
fn test_canvas_add_connection() -> FlowResult<()> {
    let mut canvas = Canvas::new("Test Canvas");
    let element1 = Element::new(ElementType::Rectangle, 100.0, 100.0);
    let element2 = Element::new(ElementType::Circle, 200.0, 200.0);
    
    canvas.add_element(element1.clone())?;
    canvas.add_element(element2.clone())?;
    
    let connection = Connection::new(element1.id, element2.id, ConnectionType::Straight);
    canvas.add_connection(connection)?;
    
    assert_eq!(canvas.connections.len(), 1);
    Ok(())
}

#[test]
fn test_canvas_add_connection_invalid_elements() -> FlowResult<()> {
    let mut canvas = Canvas::new("Test Canvas");
    let element = Element::new(ElementType::Rectangle, 100.0, 100.0);
    
    canvas.add_element(element)?;
    
    let connection = Connection::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        ConnectionType::Straight
    );
    
    let result = canvas.add_connection(connection);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_canvas_set_background() {
    let mut canvas = Canvas::new("Test Canvas");
    canvas.set_background(Background::Color(Color::BLUE));
    
    match canvas.background {
        Background::Color(color) => assert_eq!(color, Color::BLUE),
        _ => panic!("Expected Color background"),
    }
}

// ============================================================================
// Element Tests
// ============================================================================

#[test]
fn test_element_creation() {
    let element = Element::new(ElementType::Rectangle, 100.0, 100.0);
    
    assert_eq!(element.element_type, ElementType::Rectangle);
    assert_eq!(element.x, 100.0);
    assert_eq!(element.y, 100.0);
    assert_eq!(element.width, 100.0);
    assert_eq!(element.height, 50.0);
    assert_eq!(element.layer, 0);
    assert!(!element.locked);
    assert!(element.visible);
}

#[test]
fn test_element_with_text() {
    let element = Element::new(ElementType::Text, 100.0, 100.0)
        .with_text("Hello World");
    
    assert_eq!(element.text, Some("Hello World".to_string()));
}

#[test]
fn test_element_with_size() {
    let element = Element::new(ElementType::Rectangle, 100.0, 100.0)
        .with_size(200.0, 150.0);
    
    assert_eq!(element.width, 200.0);
    assert_eq!(element.height, 150.0);
}

#[test]
fn test_element_move() {
    let mut element = Element::new(ElementType::Rectangle, 100.0, 100.0);
    element.move_to(200.0, 300.0);
    
    assert_eq!(element.x, 200.0);
    assert_eq!(element.y, 300.0);
}

#[test]
fn test_element_resize() {
    let mut element = Element::new(ElementType::Rectangle, 100.0, 100.0);
    element.resize(300.0, 200.0);
    
    assert_eq!(element.width, 300.0);
    assert_eq!(element.height, 200.0);
}

#[test]
fn test_element_types() {
    let types = vec![
        ElementType::Rectangle,
        ElementType::Circle,
        ElementType::Diamond,
        ElementType::Parallelogram,
        ElementType::RoundedRectangle,
        ElementType::Hexagon,
        ElementType::Triangle,
        ElementType::Star,
        ElementType::Arrow,
        ElementType::Text,
        ElementType::Image,
        ElementType::Custom("custom_shape".to_string()),
    ];
    
    for element_type in types {
        let element = Element::new(element_type.clone(), 0.0, 0.0);
        assert_eq!(element.element_type, element_type);
    }
}

// ============================================================================
// Connection Tests
// ============================================================================

#[test]
fn test_connection_creation() {
    let connection = Connection::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        ConnectionType::Straight
    );
    
    assert_eq!(connection.connection_type, ConnectionType::Straight);
    assert!(!connection.bidirectional);
}

#[test]
fn test_connection_with_label() {
    let connection = Connection::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        ConnectionType::Straight
    ).with_label("Test Label");
    
    assert_eq!(connection.label, Some("Test Label".to_string()));
}

#[test]
fn test_connection_types() {
    let types = vec![
        ConnectionType::Straight,
        ConnectionType::Orthogonal,
        ConnectionType::Curved,
        ConnectionType::Step,
        ConnectionType::Freeform,
    ];
    
    for connection_type in types {
        let connection = Connection::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            connection_type.clone()
        );
        assert_eq!(connection.connection_type, connection_type);
    }
}

// ============================================================================
// Color Tests
// ============================================================================

#[test]
fn test_color_creation() {
    let color = Color::new(255, 128, 64, 255);
    
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);
    assert_eq!(color.a, 255);
}

#[test]
fn test_color_rgb() {
    let color = Color::rgb(255, 128, 64);
    
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);
    assert_eq!(color.a, 255);
}

#[test]
fn test_color_common_colors() {
    assert_eq!(Color::WHITE, Color::rgb(255, 255, 255));
    assert_eq!(Color::BLACK, Color::rgb(0, 0, 0));
    assert_eq!(Color::RED, Color::rgb(255, 0, 0));
    assert_eq!(Color::GREEN, Color::rgb(0, 255, 0));
    assert_eq!(Color::BLUE, Color::rgb(0, 0, 255));
    assert_eq!(Color::YELLOW, Color::rgb(255, 255, 0));
}

#[test]
fn test_color_from_hex() -> FlowResult<()> {
    let color = Color::from_hex("#FF8000")?;
    
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 0);
    assert_eq!(color.a, 255);
    Ok(())
}

#[test]
fn test_color_from_hex_with_alpha() -> FlowResult<()> {
    let color = Color::from_hex("#FF800080")?;
    
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 0);
    assert_eq!(color.a, 128);
    Ok(())
}

#[test]
fn test_color_from_hex_invalid() {
    let result = Color::from_hex("invalid");
    assert!(result.is_err());
}

#[test]
fn test_color_to_hex() {
    let color = Color::rgb(255, 128, 64);
    let hex = color.to_hex();
    
    assert_eq!(hex, "#FF8040FF");
}

// ============================================================================
// Style Tests
// ============================================================================

#[test]
fn test_style_default() {
    let style = Style::default();
    
    assert_eq!(style.fill, Color::WHITE);
    assert_eq!(style.stroke.color, Color::BLACK);
    assert_eq!(style.stroke.width, 1.0);
}

// ============================================================================
// Stroke Tests
// ============================================================================

#[test]
fn test_line_styles() {
    let styles = vec![
        LineStyle::Solid,
        LineStyle::Dashed,
        LineStyle::Dotted,
        LineStyle::DashDot,
        LineStyle::DashDotDot,
    ];
    
    for line_style in styles {
        let mut stroke = Stroke::default();
        stroke.line_style = line_style.clone();
        assert_eq!(stroke.line_style, line_style);
    }
}

// ============================================================================
// Font Tests
// ============================================================================

#[test]
fn test_font_default() {
    let font = Font::default();
    
    assert_eq!(font.family, "Arial");
    assert_eq!(font.size, 12.0);
}

#[test]
fn test_font_weights() {
    let weights = vec![
        FontWeight::Thin,
        FontWeight::ExtraLight,
        FontWeight::Light,
        FontWeight::Normal,
        FontWeight::Medium,
        FontWeight::SemiBold,
        FontWeight::Bold,
        FontWeight::ExtraBold,
        FontWeight::Black,
    ];
    
    for weight in weights {
        let mut font = Font::default();
        font.weight = weight.clone();
        assert_eq!(font.weight, weight);
    }
}

// ============================================================================
// Task Tests
// ============================================================================

#[test]
fn test_task_creation() {
    let task = Task::new("Test Task");
    
    assert_eq!(task.title, "Test Task");
    assert_eq!(task.status, TaskStatus::Todo);
    assert_eq!(task.priority, TaskPriority::Medium);
    assert_eq!(task.actual_hours, 0.0);
    assert!(task.assignees.is_empty());
    assert!(task.dependencies.is_empty());
}

#[test]
fn test_task_set_status() {
    let mut task = Task::new("Test Task");
    task.set_status(TaskStatus::InProgress);
    
    assert_eq!(task.status, TaskStatus::InProgress);
}

#[test]
fn test_task_set_priority() {
    let mut task = Task::new("Test Task");
    task.set_priority(TaskPriority::High);
    
    assert_eq!(task.priority, TaskPriority::High);
}

#[test]
fn test_task_add_assignee() {
    let mut task = Task::new("Test Task");
    let user_id = Uuid::new_v4();
    
    task.add_assignee(user_id);
    
    assert_eq!(task.assignees.len(), 1);
    assert!(task.assignees.contains(&user_id));
}

#[test]
fn test_task_add_assignee_duplicate() {
    let mut task = Task::new("Test Task");
    let user_id = Uuid::new_v4();
    
    task.add_assignee(user_id);
    task.add_assignee(user_id);
    
    assert_eq!(task.assignees.len(), 1);
}

#[test]
fn test_task_add_dependency() {
    let mut task = Task::new("Test Task");
    let task_id = Uuid::new_v4();
    
    task.add_dependency(task_id, DependencyType::FinishToStart);
    
    assert_eq!(task.dependencies.len(), 1);
}

#[test]
fn test_task_is_overdue() {
    let mut task = Task::new("Test Task");
    task.status = TaskStatus::Todo;
    task.due_date = Some(Utc::now() - Duration::days(1));
    
    assert!(task.is_overdue());
}

#[test]
fn test_task_is_not_overdue() {
    let mut task = Task::new("Test Task");
    task.status = TaskStatus::Todo;
    task.due_date = Some(Utc::now() + Duration::days(1));
    
    assert!(!task.is_overdue());
}

#[test]
fn test_task_is_not_overdue_when_done() {
    let mut task = Task::new("Test Task");
    task.status = TaskStatus::Done;
    task.due_date = Some(Utc::now() - Duration::days(1));
    
    assert!(!task.is_overdue());
}

#[test]
fn test_task_progress() {
    let mut task = Task::new("Test Task");
    
    assert_eq!(task.progress(), 0.0);
    
    task.set_status(TaskStatus::InProgress);
    assert_eq!(task.progress(), 0.5);
    
    task.set_status(TaskStatus::Review);
    assert_eq!(task.progress(), 0.75);
    
    task.set_status(TaskStatus::Done);
    assert_eq!(task.progress(), 1.0);
}

#[test]
fn test_task_statuses() {
    let statuses = vec![
        TaskStatus::Todo,
        TaskStatus::InProgress,
        TaskStatus::Review,
        TaskStatus::Done,
        TaskStatus::Blocked,
    ];
    
    for status in statuses {
        let mut task = Task::new("Test Task");
        task.set_status(status.clone());
        assert_eq!(task.status, status);
    }
}

#[test]
fn test_task_priorities() {
    let priorities = vec![
        TaskPriority::Low,
        TaskPriority::Medium,
        TaskPriority::High,
        TaskPriority::Critical,
    ];
    
    for priority in priorities {
        let mut task = Task::new("Test Task");
        task.set_priority(priority.clone());
        assert_eq!(task.priority, priority);
    }
}

#[test]
fn test_dependency_types() {
    let types = vec![
        DependencyType::FinishToStart,
        DependencyType::StartToStart,
        DependencyType::FinishToFinish,
        DependencyType::StartToFinish,
    ];
    
    for dep_type in types {
        let mut task = Task::new("Test Task");
        let task_id = Uuid::new_v4();
        task.add_dependency(task_id, dep_type.clone());
        assert_eq!(task.dependencies[0].dependency_type, dep_type);
    }
}

// ============================================================================
// Project Tests
// ============================================================================

#[test]
fn test_project_creation() {
    let owner_id = Uuid::new_v4();
    let project = Project::new("Test Project", owner_id);
    
    assert_eq!(project.name, "Test Project");
    assert!(project.tasks.is_empty());
}

#[test]
fn test_project_add_task() {
    let owner_id = Uuid::new_v4();
    let mut project = Project::new("Test Project", owner_id);
    let task = Task::new("Test Task");
    
    project.add_task(task);
    
    assert_eq!(project.tasks.len(), 1);
}

#[test]
fn test_project_remove_task() {
    let owner_id = Uuid::new_v4();
    let mut project = Project::new("Test Project", owner_id);
    let task = Task::new("Test Task");
    let task_id = task.id;
    
    project.add_task(task);
    project.remove_task(task_id);
    
    assert_eq!(project.tasks.len(), 0);
}

#[test]
fn test_project_get_task() {
    let owner_id = Uuid::new_v4();
    let mut project = Project::new("Test Project", owner_id);
    let task = Task::new("Test Task");
    let task_id = task.id;
    
    project.add_task(task);
    
    let retrieved = project.get_task(task_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, task_id);
}

// ============================================================================
// Kanban Board Tests
// ============================================================================

#[test]
fn test_kanban_board_creation() {
    let board = KanbanBoard::new("Test Board");
    
    assert_eq!(board.name, "Test Board");
    assert_eq!(board.columns.len(), 4); // Default columns
    assert_eq!(board.columns[0].name, "Todo");
    assert_eq!(board.columns[1].name, "In Progress");
    assert_eq!(board.columns[2].name, "Review");
    assert_eq!(board.columns[3].name, "Done");
}

#[test]
fn test_kanban_board_add_column() {
    let mut board = KanbanBoard::new("Test Board");
    board.add_column("Blocked".to_string(), Color::RED);
    
    assert_eq!(board.columns.len(), 5); // 4 default + 1 custom
    assert_eq!(board.columns[4].name, "Blocked");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_complete_canvas_workflow() -> FlowResult<()> {
    // Create canvas
    let mut canvas = Canvas::new("Project Plan");
    canvas.resize(2560.0, 1440.0);
    
    // Add elements
    let task1 = Element::new(ElementType::RoundedRectangle, 100.0, 100.0)
        .with_text("Task 1: Start")
        .with_size(200.0, 80.0);
    
    let task2 = Element::new(ElementType::RoundedRectangle, 400.0, 100.0)
        .with_text("Task 2: Process")
        .with_size(200.0, 80.0);
    
    let task3 = Element::new(ElementType::Diamond, 700.0, 100.0)
        .with_text("Decision?")
        .with_size(150.0, 100.0);
    
    let task1_id = task1.id;
    let task2_id = task2.id;
    let task3_id = task3.id;
    
    canvas.add_element(task1)?;
    canvas.add_element(task2)?;
    canvas.add_element(task3)?;
    
    // Add connections
    let conn1 = Connection::new(task1_id, task2_id, ConnectionType::Straight)
        .with_label("Next");
    
    let conn2 = Connection::new(task2_id, task3_id, ConnectionType::Curved)
        .with_label("Review");
    
    canvas.add_connection(conn1)?;
    canvas.add_connection(conn2)?;
    
    // Verify
    assert_eq!(canvas.elements.len(), 3);
    assert_eq!(canvas.connections.len(), 2);
    
    Ok(())
}

#[test]
fn test_complete_project_workflow() -> FlowResult<()> {
    // Create project
    let owner_id = Uuid::new_v4();
    let mut project = Project::new("Website Redesign", owner_id);
    
    // Add tasks
    let mut task1 = Task::new("Design Mockups");
    task1.set_status(TaskStatus::Done);
    task1.actual_hours = 8.0;
    
    let mut task2 = Task::new("Develop Frontend");
    task2.set_status(TaskStatus::InProgress);
    task2.set_priority(TaskPriority::High);
    
    let mut task3 = Task::new("Test and QA");
    task3.set_status(TaskStatus::Todo);
    task3.add_dependency(task2.id, DependencyType::FinishToStart);
    
    let task1_id = task1.id;
    let task2_id = task2.id;
    
    project.add_task(task1);
    project.add_task(task2);
    project.add_task(task3);
    
    // Verify
    assert_eq!(project.tasks.len(), 3);
    
    let t1 = project.get_task(task1_id).unwrap();
    let t2 = project.get_task(task2_id).unwrap();
    
    assert_eq!(t1.progress(), 1.0);
    assert_eq!(t2.progress(), 0.5);
    
    Ok(())
}