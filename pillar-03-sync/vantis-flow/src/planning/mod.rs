//! Planning module for Vantis Flow
//! 
//! Provides task management, project planning, Gantt charts, and Kanban boards.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

use crate::FlowResult;
use crate::core::{Canvas, Element, ElementType, Connection, ConnectionType, Style, Color};

/// Task in a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier
    pub id: Uuid,
    
    /// Task title
    pub title: String,
    
    /// Task description
    pub description: Option<String>,
    
    /// Task status
    pub status: TaskStatus,
    
    /// Task priority
    pub priority: TaskPriority,
    
    /// Assigned user IDs
    pub assignees: Vec<Uuid>,
    
    /// Start date
    pub start_date: Option<DateTime<Utc>>,
    
    /// Due date
    pub due_date: Option<DateTime<Utc>>,
    
    /// Estimated hours
    pub estimated_hours: Option<f64>,
    
    /// Actual hours spent
    pub actual_hours: f64,
    
    /// Task dependencies
    pub dependencies: Vec<TaskDependency>,
    
    /// Task tags
    pub tags: Vec<String>,
    
    /// Task metadata
    pub metadata: HashMap<String, String>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
}

impl Task {
    /// Create a new task
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            description: None,
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            assignees: Vec::new(),
            start_date: None,
            due_date: None,
            estimated_hours: None,
            actual_hours: 0.0,
            dependencies: Vec::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
            created_at: now,
            modified_at: now,
        }
    }
    
    /// Set task status
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.modified_at = Utc::now();
    }
    
    /// Set task priority
    pub fn set_priority(&mut self, priority: TaskPriority) {
        self.priority = priority;
        self.modified_at = Utc::now();
    }
    
    /// Add assignee
    pub fn add_assignee(&mut self, user_id: Uuid) {
        if !self.assignees.contains(&user_id) {
            self.assignees.push(user_id);
            self.modified_at = Utc::now();
        }
    }
    
    /// Add dependency
    pub fn add_dependency(&mut self, task_id: Uuid, dependency_type: DependencyType) {
        let dependency = TaskDependency {
            task_id,
            dependency_type,
        };
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
            self.modified_at = Utc::now();
        }
    }
    
    /// Check if task is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            return Utc::now() > due_date && self.status != TaskStatus::Done;
        }
        false
    }
    
    /// Get task progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        match self.status {
            TaskStatus::Todo => 0.0,
            TaskStatus::InProgress => 0.5,
            TaskStatus::Review => 0.75,
            TaskStatus::Done => 1.0,
            TaskStatus::Blocked => 0.0,
        }
    }
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    /// Todo
    Todo,
    
    /// In progress
    InProgress,
    
    /// Review
    Review,
    
    /// Done
    Done,
    
    /// Blocked
    Blocked,
}

/// Task priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    /// Low priority
    Low,
    
    /// Medium priority
    Medium,
    
    /// High priority
    High,
    
    /// Critical priority
    Critical,
}

/// Task dependency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskDependency {
    /// Task ID this task depends on
    pub task_id: Uuid,
    
    /// Dependency type
    pub dependency_type: DependencyType,
}

/// Dependency type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DependencyType {
    /// Finish-to-start (default)
    FinishToStart,
    
    /// Start-to-start
    StartToStart,
    
    /// Finish-to-finish
    FinishToFinish,
    
    /// Start-to-finish
    StartToFinish,
}

/// Project containing tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Unique identifier
    pub id: Uuid,
    
    /// Project name
    pub name: String,
    
    /// Project description
    pub description: Option<String>,
    
    /// Tasks in the project
    pub tasks: HashMap<Uuid, Task>,
    
    /// Project start date
    pub start_date: Option<DateTime<Utc>>,
    
    /// Project end date
    pub end_date: Option<DateTime<Utc>>,
    
    /// Project status
    pub status: ProjectStatus,
    
    /// Project owner
    pub owner: Uuid,
    
    /// Project members
    pub members: Vec<Uuid>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
}

impl Project {
    /// Create a new project
    pub fn new(name: impl Into<String>, owner: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            tasks: HashMap::new(),
            start_date: None,
            end_date: None,
            status: ProjectStatus::Planning,
            owner,
            members: vec![owner],
            created_at: now,
            modified_at: now,
        }
    }
    
    /// Add a task to the project
    pub fn add_task(&mut self, task: Task) -> FlowResult<()> {
        let task_id = task.id;
        self.tasks.insert(task_id, task);
        self.modified_at = Utc::now();
        Ok(())
    }
    
    /// Remove a task from the project
    pub fn remove_task(&mut self, task_id: Uuid) -> FlowResult<()> {
        self.tasks.remove(&task_id);
        self.modified_at = Utc::now();
        Ok(())
    }
    
    /// Get a task by ID
    pub fn get_task(&self, task_id: Uuid) -> Option<&Task> {
        self.tasks.get(&task_id)
    }
    
    /// Get all tasks with a specific status
    pub fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.status == status)
            .collect()
    }
    
    /// Get all tasks with a specific priority
    pub fn get_tasks_by_priority(&self, priority: TaskPriority) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.priority == priority)
            .collect()
    }
    
    /// Get overdue tasks
    pub fn get_overdue_tasks(&self) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.is_overdue())
            .collect()
    }
    
    /// Calculate project progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        
        let total_progress: f64 = self.tasks.values().map(|task| task.progress()).sum();
        total_progress / self.tasks.len() as f64
    }
    
    /// Get project duration in days
    pub fn duration(&self) -> Option<i64> {
        if let (Some(start), Some(end)) = (self.start_date, self.end_date) {
            Some((end - start).num_days())
        } else {
            None
        }
    }
}

/// Project status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    /// Planning phase
    Planning,
    
    /// Active phase
    Active,
    
    /// On hold
    OnHold,
    
    /// Completed
    Completed,
    
    /// Cancelled
    Cancelled,
}

/// Gantt chart for project visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GanttChart {
    /// Unique identifier
    pub id: Uuid,
    
    /// Gantt chart name
    pub name: String,
    
    /// Project reference
    pub project_id: Uuid,
    
    /// Tasks to display
    pub tasks: Vec<Uuid>,
    
    /// Timeline start date
    pub start_date: DateTime<Utc>,
    
    /// Timeline end date
    pub end_date: DateTime<Utc>,
    
    /// Day width in pixels
    pub day_width: f64,
    
    /// Row height in pixels
    pub row_height: f64,
    
    /// Canvas for rendering
    pub canvas: Canvas,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
}

impl GanttChart {
    /// Create a new Gantt chart
    pub fn new(name: impl Into<String>, project_id: Uuid) -> Self {
        let now = Utc::now();
        let start_date = now - Duration::days(7);
        let end_date = now + Duration::days(30);
        let name_str = name.into();
        
        Self {
            id: Uuid::new_v4(),
            name: name_str.clone(),
            project_id,
            tasks: Vec::new(),
            start_date,
            end_date,
            day_width: 40.0,
            row_height: 50.0,
            canvas: Canvas::new(name_str),
            created_at: now,
            modified_at: now,
        }
    }
    
    /// Add a task to the Gantt chart
    pub fn add_task(&mut self, task_id: Uuid) -> FlowResult<()> {
        self.tasks.push(task_id);
        self.modified_at = Utc::now();
        Ok(())
    }
    
    /// Generate Gantt chart from project tasks
    pub fn generate_from_project(&mut self, project: &Project) -> FlowResult<()> {
        // Clear existing tasks
        self.tasks.clear();
        self.canvas.elements.clear();
        self.canvas.connections.clear();
        
        // Add all project tasks
        for task_id in project.tasks.keys() {
            self.add_task(*task_id)?;
        }
        
        // Calculate date range
        let mut min_date = Utc::now();
        let mut max_date = Utc::now();
        let mut has_dates = false;
        
        for task in project.tasks.values() {
            if let Some(start) = task.start_date {
                if !has_dates || start < min_date {
                    min_date = start;
                }
                has_dates = true;
            }
            if let Some(due) = task.due_date {
                if !has_dates || due > max_date {
                    max_date = due;
                }
                has_dates = true;
            }
        }
        
        if has_dates {
            self.start_date = min_date - Duration::days(3);
            self.end_date = max_date + Duration::days(7);
        }
        
        // Render tasks
        let mut row = 0;
        for task in project.tasks.values() {
            if let (Some(start), Some(due)) = (task.start_date, task.due_date) {
                let x = ((start - self.start_date).num_days() as f64) * self.day_width;
                let width = ((due - start).num_days() as f64) * self.day_width;
                let y = 100.0 + (row as f64) * self.row_height;
                
                let color = match task.status {
                    TaskStatus::Todo => Color::LIGHT_GRAY,
                    TaskStatus::InProgress => Color::BLUE,
                    TaskStatus::Review => Color::YELLOW,
                    TaskStatus::Done => Color::GREEN,
                    TaskStatus::Blocked => Color::RED,
                };
                
                let mut style = Style::default();
                style.fill = color;
                
                let element = Element::new(ElementType::RoundedRectangle, x, y)
                    .with_text(&task.title)
                    .with_size(width, self.row_height - 10.0)
                    .with_style(style);
                self.canvas.add_element(element)?;
                
                row += 1;
            }
        }
        
        // Render dependencies
        for task in project.tasks.values() {
            for dep in &task.dependencies {
                if let (Some(source_task), Some(target_task)) = (
                    project.tasks.get(&dep.task_id),
                    project.tasks.get(&task.id),
                ) {
                    if let (Some(_source_element), Some(_target_element)) = (
                        self.canvas.elements.get(&source_task.id),
                        self.canvas.elements.get(&target_task.id),
                    ) {
                        let connection = Connection::new(source_task.id, target_task.id, ConnectionType::Curved);
                        self.canvas.add_connection(connection)?;
                    }
                }
            }
        }
        
        self.modified_at = Utc::now();
        Ok(())
    }
}

/// Kanban board for task management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    /// Unique identifier
    pub id: Uuid,
    
    /// Board name
    pub name: String,
    
    /// Columns in the board
    pub columns: Vec<KanbanColumn>,
    
    /// Tasks in the board
    pub tasks: HashMap<Uuid, KanbanTask>,
    
    /// Canvas for rendering
    pub canvas: Canvas,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
}

impl KanbanBoard {
    /// Create a new Kanban board
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        let name_str = name.into();
        let mut board = Self {
            id: Uuid::new_v4(),
            name: name_str.clone(),
            columns: Vec::new(),
            tasks: HashMap::new(),
            canvas: Canvas::new(name_str),
            created_at: now,
            modified_at: now,
        };
        
        // Add default columns
        board.add_column("Todo".to_string(), Color::LIGHT_GRAY);
        board.add_column("In Progress".to_string(), Color::BLUE);
        board.add_column("Review".to_string(), Color::YELLOW);
        board.add_column("Done".to_string(), Color::GREEN);
        
        board
    }
    
    /// Add a column to the board
    pub fn add_column(&mut self, name: String, color: Color) {
        let column = KanbanColumn {
            id: Uuid::new_v4(),
            name,
            color,
            task_ids: Vec::new(),
        };
        self.columns.push(column);
        self.modified_at = Utc::now();
    }
    
    /// Add a task to the board
    pub fn add_task(&mut self, task: KanbanTask, column_id: Uuid) -> FlowResult<()> {
        let task_id = task.id;
        self.tasks.insert(task_id, task);
        
        // Add to column
        if let Some(column) = self.columns.iter_mut().find(|c| c.id == column_id) {
            column.task_ids.push(task_id);
        }
        
        self.modified_at = Utc::now();
        Ok(())
    }
    
    /// Move a task to a different column
    pub fn move_task(&mut self, task_id: Uuid, to_column_id: Uuid) -> FlowResult<()> {
        // Remove from current column
        for column in &mut self.columns {
            column.task_ids.retain(|id| *id != task_id);
        }
        
        // Add to new column
        if let Some(column) = self.columns.iter_mut().find(|c| c.id == to_column_id) {
            column.task_ids.push(task_id);
        }
        
        // Update task status
        if let Some(task) = self.tasks.get_mut(&task_id) {
            if let Some(column) = self.columns.iter().find(|c| c.id == to_column_id) {
                task.status = match column.name.as_str() {
                    "Todo" => TaskStatus::Todo,
                    "In Progress" => TaskStatus::InProgress,
                    "Review" => TaskStatus::Review,
                    "Done" => TaskStatus::Done,
                    _ => TaskStatus::Todo,
                };
            }
        }
        
        self.modified_at = Utc::now();
        Ok(())
    }
    
    /// Generate Kanban board from project tasks
    pub fn generate_from_project(&mut self, project: &Project) -> FlowResult<()> {
        // Clear existing tasks
        self.tasks.clear();
        for column in &mut self.columns {
            column.task_ids.clear();
        }
        
        // Add tasks to appropriate columns
        for task in project.tasks.values() {
            let column_id = match task.status {
                TaskStatus::Todo => self.columns[0].id,
                TaskStatus::InProgress => self.columns[1].id,
                TaskStatus::Review => self.columns[2].id,
                TaskStatus::Done => self.columns[3].id,
                TaskStatus::Blocked => self.columns[0].id, // Blocked goes to Todo
            };
            
            let kanban_task = KanbanTask {
                id: task.id,
                title: task.title.clone(),
                description: task.description.clone(),
                status: task.status.clone(),
                priority: task.priority.clone(),
                assignees: task.assignees.clone(),
                tags: task.tags.clone(),
                created_at: task.created_at,
                modified_at: task.modified_at,
            };
            
            self.add_task(kanban_task, column_id)?;
        }
        
        self.modified_at = Utc::now();
        Ok(())
    }
}

/// Kanban column
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanColumn {
    /// Unique identifier
    pub id: Uuid,
    
    /// Column name
    pub name: String,
    
    /// Column color
    pub color: Color,
    
    /// Task IDs in this column
    pub task_ids: Vec<Uuid>,
}

/// Kanban task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanTask {
    /// Unique identifier
    pub id: Uuid,
    
    /// Task title
    pub title: String,
    
    /// Task description
    pub description: Option<String>,
    
    /// Task status
    pub status: TaskStatus,
    
    /// Task priority
    pub priority: TaskPriority,
    
    /// Assigned user IDs
    pub assignees: Vec<Uuid>,
    
    /// Task tags
    pub tags: Vec<String>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
}

/// Timeline for project visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    /// Unique identifier
    pub id: Uuid,
    
    /// Timeline name
    pub name: String,
    
    /// Milestones
    pub milestones: Vec<Milestone>,
    
    /// Canvas for rendering
    pub canvas: Canvas,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
}

impl Timeline {
    /// Create a new timeline
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        let name_str = name.into();
        Self {
            id: Uuid::new_v4(),
            name: name_str.clone(),
            milestones: Vec::new(),
            canvas: Canvas::new(name_str),
            created_at: now,
            modified_at: now,
        }
    }
    
    /// Add a milestone
    pub fn add_milestone(&mut self, milestone: Milestone) -> FlowResult<()> {
        self.milestones.push(milestone);
        self.modified_at = Utc::now();
        Ok(())
    }
    
    /// Generate timeline from project
    pub fn generate_from_project(&mut self, project: &Project) -> FlowResult<()> {
        // Clear existing milestones
        self.milestones.clear();
        self.canvas.elements.clear();
        
        // Add project start as milestone
        if let Some(start_date) = project.start_date {
            let milestone = Milestone {
                id: Uuid::new_v4(),
                title: format!("{} Start", project.name),
                description: Some("Project kickoff".to_string()),
                date: start_date,
                completed: true,
                color: Color::GREEN,
            };
            self.add_milestone(milestone)?;
        }
        
        // Add project end as milestone
        if let Some(end_date) = project.end_date {
            let milestone = Milestone {
                id: Uuid::new_v4(),
                title: format!("{} End", project.name),
                description: Some("Project completion".to_string()),
                date: end_date,
                completed: project.status == ProjectStatus::Completed,
                color: Color::BLUE,
            };
            self.add_milestone(milestone)?;
        }
        
        // Sort milestones by date
        self.milestones.sort_by(|a, b| a.date.cmp(&b.date));
        
        // Render milestones
        let mut x = 100.0;
        let y = 540.0;
        let spacing = 300.0;
        
        for milestone in &self.milestones {
            let color = if milestone.completed {
                Color::GREEN
            } else {
                Color::GRAY
            };
            
            let mut style = Style::default();
            style.fill = color;
            
            let element = Element::new(ElementType::Circle, x, y)
                .with_text(&milestone.title)
                .with_size(40.0, 40.0)
                .with_style(style);
            self.canvas.add_element(element)?;
            
            x += spacing;
        }
        
        self.modified_at = Utc::now();
        Ok(())
    }
}

/// Milestone in a timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    /// Unique identifier
    pub id: Uuid,
    
    /// Milestone title
    pub title: String,
    
    /// Milestone description
    pub description: Option<String>,
    
    /// Milestone date
    pub date: DateTime<Utc>,
    
    /// Whether milestone is completed
    pub completed: bool,
    
    /// Milestone color
    pub color: Color,
}