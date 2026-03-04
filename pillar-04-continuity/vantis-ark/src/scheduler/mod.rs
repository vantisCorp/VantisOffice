//! Scheduler module for backup scheduling

use chrono::{DateTime, Datelike, Timelike, Utc, Weekday as ChronoWeekday};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Backup scheduler
pub struct BackupScheduler {
    schedules: Vec<Schedule>,
}

impl BackupScheduler {
    pub fn new() -> Self {
        BackupScheduler {
            schedules: Vec::new(),
        }
    }

    pub fn add_schedule(&mut self, schedule: Schedule) {
        self.schedules.push(schedule);
    }

    pub fn get_due_tasks(&self, now: DateTime<Utc>) -> Vec<BackupTask> {
        let mut tasks = Vec::new();

        for schedule in &self.schedules {
            if self.is_due(schedule, now) {
                tasks.push(BackupTask {
                    id: Uuid::new_v4().to_string(),
                    schedule_id: schedule.id.clone(),
                    config: TaskConfig {
                        backup_name: schedule.name.clone(),
                        parts: 10,
                        threshold: 3,
                    },
                    status: TaskStatus::Pending,
                    created_at: now,
                    started_at: None,
                    completed_at: None,
                    result: None,
                });
            }
        }

        tasks
    }

    fn is_due(&self, schedule: &Schedule, now: DateTime<Utc>) -> bool {
        match schedule.frequency {
            ScheduleFrequency::Hourly => now.minute() == 0 && now.second() == 0,
            ScheduleFrequency::Daily => {
                now.hour() == schedule.config.hour && now.minute() == 0 && now.second() == 0
            }
            ScheduleFrequency::Weekly => {
                let weekday = now.weekday();
                schedule.config.weekdays.contains(&weekday.into())
                    && now.hour() == schedule.config.hour
                    && now.minute() == 0
                    && now.second() == 0
            }
            ScheduleFrequency::Monthly => {
                now.day() == schedule.config.day_of_month
                    && now.hour() == schedule.config.hour
                    && now.minute() == 0
                    && now.second() == 0
            }
        }
    }
}

/// Schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub frequency: ScheduleFrequency,
    pub config: ScheduleConfig,
    pub retention: RetentionPolicy,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleFrequency {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub hour: u32,
    pub minute: u32,
    pub day_of_month: u32,
    pub weekdays: Vec<crate::shamir::Weekday>,
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        ScheduleConfig {
            hour: 0,
            minute: 0,
            day_of_month: 1,
            weekdays: vec![crate::shamir::Weekday::Monday],
        }
    }
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub keep_daily: u32,
    pub keep_weekly: u32,
    pub keep_monthly: u32,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        RetentionPolicy {
            keep_daily: 7,
            keep_weekly: 4,
            keep_monthly: 12,
        }
    }
}

/// Backup task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupTask {
    pub id: String,
    pub schedule_id: String,
    pub config: TaskConfig,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<TaskResult>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    pub backup_name: String,
    pub parts: usize,
    pub threshold: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub backup_id: String,
    pub parts_distributed: usize,
    pub error: Option<String>,
}
