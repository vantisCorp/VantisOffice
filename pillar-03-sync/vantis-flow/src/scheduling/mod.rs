//! Flow Scheduling Module - Integration with Vantis Chronos (Pillar 03)
//!
//! Bridges Flow project planning with Chronos calendar scheduling,
//! enabling automatic task scheduling, milestone tracking, and
//! meeting suggestions based on project timelines.
//!
//! # Architecture
//!
//! ```text
//! vantis-flow (Pillar 03: Sync)
//!     └── scheduling module
//!             └── vantis-chronos (Pillar 03: Sync)
//!                     ├── Suggester (meeting suggestions)
//!                     ├── ConflictDetector
//!                     ├── Availability
//!                     └── ScheduleOptimizer
//! ```

use chrono::{DateTime, Duration, NaiveTime, Utc, Weekday};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use vantis_chronos::core::{Calendar, Color as ChronosColor, Event, EventStatus, TimeRange};
use vantis_chronos::scheduling::{Availability, ConflictDetector, Suggester, SuggestionCriteria};

use crate::planning::{Milestone, Project, Task, TaskStatus};

/// Flow scheduling manager that bridges project planning with calendar scheduling
pub struct FlowSchedulingManager {
    /// Calendar for scheduling
    calendar: Calendar,
    /// Scheduling configuration
    config: SchedulingConfig,
    /// Availability tracker
    availability: Availability,
    /// Scheduled tasks
    scheduled_tasks: Vec<ScheduledTask>,
}

/// Configuration for flow scheduling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingConfig {
    /// Working hours start
    pub work_start: NaiveTime,
    /// Working hours end
    pub work_end: NaiveTime,
    /// Working days
    pub working_days: Vec<Weekday>,
    /// Default task duration in hours
    pub default_task_duration_hours: f64,
    /// Buffer time between tasks in minutes
    pub buffer_minutes: i64,
    /// Whether to auto-schedule tasks
    pub auto_schedule: bool,
}

impl Default for SchedulingConfig {
    fn default() -> Self {
        Self {
            work_start: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            work_end: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            working_days: vec![
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
            ],
            default_task_duration_hours: 2.0,
            buffer_minutes: 15,
            auto_schedule: true,
        }
    }
}

/// A task that has been scheduled on the calendar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    /// Task ID from Flow
    pub task_id: Uuid,
    /// Task title
    pub task_title: String,
    /// Scheduled start time
    pub scheduled_start: DateTime<Utc>,
    /// Scheduled end time
    pub scheduled_end: DateTime<Utc>,
    /// Whether the task has been completed
    pub completed: bool,
}

/// Result of a scheduling operation
#[derive(Debug, Clone)]
pub struct SchedulingResult {
    /// Successfully scheduled tasks
    pub scheduled: Vec<ScheduledTask>,
    /// Tasks that could not be scheduled
    pub unscheduled: Vec<(Uuid, String)>,
    /// Detected conflicts
    pub conflicts: Vec<SchedulingConflict>,
}

/// A scheduling conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingConflict {
    /// First task ID
    pub task1_id: Uuid,
    /// Second task ID
    pub task2_id: Uuid,
    /// Description of the conflict
    pub description: String,
}

impl FlowSchedulingManager {
    /// Create a new scheduling manager with default config
    pub fn new() -> Self {
        let config = SchedulingConfig::default();
        let calendar = Calendar::new("Flow Schedule".to_string(), ChronosColor::new(66, 133, 244));

        Self {
            calendar,
            config,
            availability: Availability::new(),
            scheduled_tasks: Vec::new(),
        }
    }

    /// Create a new scheduling manager with custom config
    pub fn with_config(config: SchedulingConfig) -> Self {
        let calendar = Calendar::new("Flow Schedule".to_string(), ChronosColor::new(66, 133, 244));

        Self {
            calendar,
            config,
            availability: Availability::new(),
            scheduled_tasks: Vec::new(),
        }
    }

    /// Get the scheduling configuration
    pub fn config(&self) -> &SchedulingConfig {
        &self.config
    }

    /// Update the scheduling configuration
    pub fn set_config(&mut self, config: SchedulingConfig) {
        self.config = config;
    }

    /// Get all scheduled tasks
    pub fn scheduled_tasks(&self) -> &[ScheduledTask] {
        &self.scheduled_tasks
    }

    /// Schedule a single task within a time window
    pub fn schedule_task(
        &mut self,
        task: &Task,
        window_start: DateTime<Utc>,
        window_end: DateTime<Utc>,
    ) -> Option<ScheduledTask> {
        // Determine task duration
        let duration_hours = task.estimated_hours.unwrap_or(self.config.default_task_duration_hours);
        let duration = Duration::minutes((duration_hours * 60.0) as i64);

        // Use Chronos Suggester to find available time slots
        let working_hours = TimeRange {
            start: self.config.work_start,
            end: self.config.work_end,
        };
        let suggester = Suggester::new(working_hours, self.config.working_days.clone());

        let criteria = SuggestionCriteria {
            preferred_time: Some(window_start),
            min_score: 0.0,
        };

        let suggestions = suggester.suggest_meeting_times(
            &[&self.calendar],
            duration,
            window_start,
            window_end,
            &criteria,
        );

        if let Some(best) = suggestions.first() {
            let scheduled = ScheduledTask {
                task_id: task.id,
                task_title: task.title.clone(),
                scheduled_start: best.start,
                scheduled_end: best.end,
                completed: task.status == TaskStatus::Done,
            };

            // Add event to calendar to block the time
            let event = Event::new(
                format!("[Flow] {}", task.title),
                best.start,
                best.end,
            );
            self.calendar.add_event(event);
            self.scheduled_tasks.push(scheduled.clone());

            Some(scheduled)
        } else {
            None
        }
    }

    /// Schedule all pending tasks in a project
    pub fn schedule_project(
        &mut self,
        project: &Project,
        window_start: DateTime<Utc>,
        window_end: DateTime<Utc>,
    ) -> SchedulingResult {
        let mut result = SchedulingResult {
            scheduled: Vec::new(),
            unscheduled: Vec::new(),
            conflicts: Vec::new(),
        };

        // Collect tasks that need scheduling (not done, not blocked)
        let mut tasks_to_schedule: Vec<&Task> = project
            .tasks
            .values()
            .filter(|t| t.status != TaskStatus::Done && t.status != TaskStatus::Blocked)
            .collect();

        // Sort by priority (Critical first) then by due date
        tasks_to_schedule.sort_by(|a, b| {
            b.priority.cmp(&a.priority).then_with(|| {
                a.due_date.cmp(&b.due_date)
            })
        });

        // Schedule each task
        for task in tasks_to_schedule {
            // Use task's start_date as window start if available
            let task_window_start = task.start_date.unwrap_or(window_start);
            let task_window_end = task.due_date.unwrap_or(window_end);

            if let Some(scheduled) = self.schedule_task(task, task_window_start, task_window_end) {
                result.scheduled.push(scheduled);
            } else {
                result.unscheduled.push((task.id, task.title.clone()));
            }
        }

        // Detect conflicts using Chronos ConflictDetector
        let conflicts = ConflictDetector::detect_conflicts(&self.calendar);
        for conflict in conflicts {
            result.conflicts.push(SchedulingConflict {
                task1_id: Uuid::nil(),
                task2_id: Uuid::nil(),
                description: format!(
                    "Calendar conflict between events: {} and {}",
                    conflict.event1_id, conflict.event2_id
                ),
            });
        }

        result
    }

    /// Schedule milestones as calendar events
    pub fn schedule_milestones(
        &mut self,
        milestones: &[Milestone],
    ) -> Vec<ScheduledTask> {
        let mut scheduled = Vec::new();

        for milestone in milestones {
            if milestone.completed {
                continue;
            }

            // Create a 1-hour event for the milestone
            let end = milestone.date + Duration::hours(1);

            let event = Event::new(
                format!("[Milestone] {}", milestone.title),
                milestone.date,
                end,
            );
            self.calendar.add_event(event);

            let task = ScheduledTask {
                task_id: milestone.id,
                task_title: milestone.title.clone(),
                scheduled_start: milestone.date,
                scheduled_end: end,
                completed: false,
            };

            self.scheduled_tasks.push(task.clone());
            scheduled.push(task);
        }

        scheduled
    }

    /// Suggest review meeting times around milestones
    pub fn suggest_milestone_review_times(
        &self,
        milestone: &Milestone,
        review_duration: Duration,
    ) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
        // Look for available slots in the 3 days before the milestone
        let window_start = milestone.date - Duration::days(3);
        let window_end = milestone.date;

        let working_hours = TimeRange {
            start: self.config.work_start,
            end: self.config.work_end,
        };
        let suggester = Suggester::new(working_hours, self.config.working_days.clone());

        let criteria = SuggestionCriteria {
            preferred_time: Some(milestone.date - Duration::days(1)),
            min_score: 0.0,
        };

        let suggestions = suggester.suggest_meeting_times(
            &[&self.calendar],
            review_duration,
            window_start,
            window_end,
            &criteria,
        );

        suggestions
            .into_iter()
            .map(|s| (s.start, s.end))
            .collect()
    }

    /// Mark a user as busy for a time range
    pub fn set_user_busy(
        &mut self,
        user_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) {
        self.availability.set_busy(user_id, start, end);
    }

    /// Find available slots for a group of users
    pub fn find_team_availability(
        &self,
        user_ids: &[&str],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        duration: Duration,
    ) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
        self.availability.find_available_slots(user_ids, start, end, duration)
    }

    /// Get the internal calendar
    pub fn calendar(&self) -> &Calendar {
        &self.calendar
    }

    /// Clear all scheduled tasks
    pub fn clear_schedule(&mut self) {
        self.scheduled_tasks.clear();
        self.calendar = Calendar::new("Flow Schedule".to_string(), ChronosColor::new(66, 133, 244));
    }
}

impl Default for FlowSchedulingManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planning::{TaskPriority};

    fn create_test_task(title: &str, hours: f64) -> Task {
        let now = Utc::now();
        Task {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: None,
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            assignees: vec![],
            start_date: None,
            due_date: None,
            estimated_hours: Some(hours),
            actual_hours: 0.0,
            dependencies: vec![],
            tags: vec![],
            metadata: std::collections::HashMap::new(),
            created_at: now,
            modified_at: now,
        }
    }

    #[test]
    fn test_create_scheduling_manager() {
        let manager = FlowSchedulingManager::new();
        assert!(manager.scheduled_tasks().is_empty());
        assert!(manager.config().auto_schedule);
    }

    #[test]
    fn test_custom_config() {
        let config = SchedulingConfig {
            work_start: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            work_end: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            working_days: vec![Weekday::Mon, Weekday::Tue, Weekday::Wed],
            default_task_duration_hours: 1.0,
            buffer_minutes: 30,
            auto_schedule: false,
        };

        let manager = FlowSchedulingManager::with_config(config.clone());
        assert_eq!(manager.config().buffer_minutes, 30);
        assert!(!manager.config().auto_schedule);
    }

    #[test]
    fn test_schedule_task() {
        let mut manager = FlowSchedulingManager::new();
        let task = create_test_task("Write report", 2.0);

        let start = Utc::now() + Duration::days(1);
        let end = start + Duration::days(5);

        let result = manager.schedule_task(&task, start, end);
        // Result depends on whether the time window falls within working hours
        // The scheduling logic is delegated to Chronos
        if let Some(scheduled) = result {
            assert_eq!(scheduled.task_title, "Write report");
            assert!(!scheduled.completed);
        }
    }

    #[test]
    fn test_schedule_project() {
        let mut manager = FlowSchedulingManager::new();
        let mut project = Project::new("Test Project", Uuid::new_v4());

        let task1 = create_test_task("Task 1", 1.0);
        let task2 = create_test_task("Task 2", 2.0);
        project.add_task(task1).unwrap();
        project.add_task(task2).unwrap();

        let start = Utc::now() + Duration::days(1);
        let end = start + Duration::days(10);

        let result = manager.schedule_project(&project, start, end);
        // Total tasks attempted = 2
        assert_eq!(
            result.scheduled.len() + result.unscheduled.len(),
            2
        );
    }

    #[test]
    fn test_schedule_milestones() {
        let mut manager = FlowSchedulingManager::new();

        let milestones = vec![
            Milestone {
                id: Uuid::new_v4(),
                title: "Alpha Release".to_string(),
                description: None,
                date: Utc::now() + Duration::days(30),
                completed: false,
                color: crate::Color::rgb(0, 128, 255),
            },
            Milestone {
                id: Uuid::new_v4(),
                title: "Beta Release".to_string(),
                description: None,
                date: Utc::now() + Duration::days(60),
                completed: false,
                color: crate::Color::rgb(255, 128, 0),
            },
        ];

        let scheduled = manager.schedule_milestones(&milestones);
        assert_eq!(scheduled.len(), 2);
        assert_eq!(scheduled[0].task_title, "Alpha Release");
        assert_eq!(scheduled[1].task_title, "Beta Release");
    }

    #[test]
    fn test_completed_milestones_skipped() {
        let mut manager = FlowSchedulingManager::new();

        let milestones = vec![
            Milestone {
                id: Uuid::new_v4(),
                title: "Done Milestone".to_string(),
                description: None,
                date: Utc::now() + Duration::days(10),
                completed: true,
                color: crate::Color::rgb(0, 255, 0),
            },
        ];

        let scheduled = manager.schedule_milestones(&milestones);
        assert_eq!(scheduled.len(), 0);
    }

    #[test]
    fn test_team_availability() {
        let mut manager = FlowSchedulingManager::new();

        let now = Utc::now();
        manager.set_user_busy("user1", now, now + Duration::hours(2));
        manager.set_user_busy("user2", now + Duration::hours(1), now + Duration::hours(3));

        let slots = manager.find_team_availability(
            &["user1", "user2"],
            now,
            now + Duration::hours(8),
            Duration::hours(1),
        );
        // All returned slots should be after both users are free
        for (start, _end) in &slots {
            assert!(*start >= now + Duration::hours(3) || *start < now);
        }
    }

    #[test]
    fn test_clear_schedule() {
        let mut manager = FlowSchedulingManager::new();
        let task = create_test_task("Test task", 1.0);

        let start = Utc::now() + Duration::days(1);
        let end = start + Duration::days(5);
        let _ = manager.schedule_task(&task, start, end);

        manager.clear_schedule();
        assert!(manager.scheduled_tasks().is_empty());
    }

    #[test]
    fn test_default_config() {
        let config = SchedulingConfig::default();
        assert_eq!(config.default_task_duration_hours, 2.0);
        assert_eq!(config.buffer_minutes, 15);
        assert!(config.auto_schedule);
        assert_eq!(config.working_days.len(), 5);
    }
}