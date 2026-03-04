//! Scheduling module for meeting suggestions and conflict detection

use crate::core::{Calendar, DateRange, Event, EventStatus, TimeRange};
use chrono::{DateTime, Datelike, Duration, Timelike, Utc, Weekday};
use std::collections::HashMap;

/// Suggester for AI-powered meeting suggestions
pub struct Suggester {
    working_hours: TimeRange,
    working_days: Vec<Weekday>,
}

impl Suggester {
    pub fn new(working_hours: TimeRange, working_days: Vec<Weekday>) -> Self {
        Suggester {
            working_hours,
            working_days,
        }
    }

    pub fn suggest_meeting_times(
        &self,
        calendars: &[&Calendar],
        duration: Duration,
        preferred_start: DateTime<Utc>,
        preferred_end: DateTime<Utc>,
        criteria: &SuggestionCriteria,
    ) -> Vec<MeetingSuggestion> {
        let mut suggestions = Vec::new();
        let mut current = preferred_start;

        while current < preferred_end {
            if self.is_working_time(current) {
                let end = current + duration;

                if self.check_availability(calendars, current, end) {
                    let score = self.calculate_score(calendars, current, end, criteria);
                    suggestions.push(MeetingSuggestion {
                        start: current,
                        end,
                        score,
                        conflicts: Vec::new(),
                    });
                }
            }

            current = current + Duration::hours(1);
        }

        suggestions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        suggestions.truncate(10);
        suggestions
    }

    fn is_working_time(&self, datetime: DateTime<Utc>) -> bool {
        let weekday = datetime.weekday();
        if !self.working_days.contains(&weekday) {
            return false;
        }

        let time = datetime.time();
        time >= self.working_hours.start && time <= self.working_hours.end
    }

    fn check_availability(
        &self,
        calendars: &[&Calendar],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> bool {
        for calendar in calendars {
            for event in &calendar.events {
                if event.status == EventStatus::Confirmed {
                    let event_start = event.start;
                    let event_end = event.end;

                    if start < event_end && end > event_start {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn calculate_score(
        &self,
        calendars: &[&Calendar],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        criteria: &SuggestionCriteria,
    ) -> f64 {
        let mut score = 0.0;

        // Prefer times closer to preferred time
        if let Some(preferred) = criteria.preferred_time {
            let diff = (start - preferred).num_minutes().abs() as f64;
            score += 100.0 / (diff + 1.0);
        }

        // Prefer morning times
        if start.hour() < 12 {
            score += 20.0;
        }

        // Avoid times near existing events
        for calendar in calendars {
            for event in &calendar.events {
                let event_start = event.start;
                let event_end = event.end;

                let before_diff = (start - event_end).num_minutes().abs() as f64;
                let after_diff = (event_start - end).num_minutes().abs() as f64;

                let min_diff = before_diff.min(after_diff);
                if min_diff < 60.0 {
                    score -= (60.0 - min_diff) / 2.0;
                }
            }
        }

        score
    }
}

#[derive(Debug, Clone)]
pub struct MeetingSuggestion {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub score: f64,
    pub conflicts: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SuggestionCriteria {
    pub preferred_time: Option<DateTime<Utc>>,
    pub min_score: f64,
}

impl Default for SuggestionCriteria {
    fn default() -> Self {
        SuggestionCriteria {
            preferred_time: None,
            min_score: 0.0,
        }
    }
}

/// Conflict detector
pub struct ConflictDetector;

impl ConflictDetector {
    pub fn detect_conflicts(calendar: &Calendar) -> Vec<Conflict> {
        let mut conflicts = Vec::new();
        let events: Vec<&Event> = calendar.events.iter().collect();

        for i in 0..events.len() {
            for j in (i + 1)..events.len() {
                let event1 = events[i];
                let event2 = events[j];

                if event1.status == EventStatus::Confirmed
                    && event2.status == EventStatus::Confirmed
                {
                    if event1.overlaps(event2) {
                        let severity = Self::calculate_severity(event1, event2);
                        conflicts.push(Conflict {
                            id: format!("{}_{}", event1.id, event2.id),
                            event1_id: event1.id.clone(),
                            event2_id: event2.id.clone(),
                            severity,
                            start: event1.start.max(event2.start),
                            end: event1.end.min(event2.end),
                        });
                    }
                }
            }
        }

        conflicts
    }

    fn calculate_severity(event1: &Event, event2: &Event) -> ConflictSeverity {
        let overlap_start = event1.start.max(event2.start);
        let overlap_end = event1.end.min(event2.end);
        let overlap_duration = overlap_end.signed_duration_since(overlap_start);

        let min_duration = event1.duration().min(event2.duration());
        let overlap_ratio =
            overlap_duration.num_minutes() as f64 / min_duration.num_minutes() as f64;

        if overlap_ratio > 0.5 {
            ConflictSeverity::High
        } else if overlap_ratio > 0.25 {
            ConflictSeverity::Medium
        } else {
            ConflictSeverity::Low
        }
    }
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub id: String,
    pub event1_id: String,
    pub event2_id: String,
    pub severity: ConflictSeverity,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
}

/// Availability manager
pub struct Availability {
    busy_times: HashMap<String, Vec<(DateTime<Utc>, DateTime<Utc>)>>,
}

impl Availability {
    pub fn new() -> Self {
        Availability {
            busy_times: HashMap::new(),
        }
    }

    pub fn set_busy(&mut self, user_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) {
        self.busy_times
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push((start, end));
    }

    pub fn find_available_slots(
        &self,
        user_ids: &[&str],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        duration: Duration,
    ) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
        let mut available_slots = Vec::new();
        let mut current = start;

        while current + duration <= end {
            let slot_end = current + duration;

            if self.is_available(user_ids, current, slot_end) {
                available_slots.push((current, slot_end));
            }

            current = current + Duration::minutes(30);
        }

        available_slots
    }

    fn is_available(&self, user_ids: &[&str], start: DateTime<Utc>, end: DateTime<Utc>) -> bool {
        for user_id in user_ids {
            if let Some(busy_times) = self.busy_times.get(*user_id) {
                for (busy_start, busy_end) in busy_times {
                    if start < *busy_end && end > *busy_start {
                        return false;
                    }
                }
            }
        }
        true
    }
}

/// Schedule optimizer
pub struct ScheduleOptimizer;

impl ScheduleOptimizer {
    pub fn optimize_schedule(&self, calendar: &mut Calendar) {
        // Sort events by start time
        calendar.events.sort_by(|a, b| a.start.cmp(&b.start));

        // Remove duplicate events
        let mut seen = std::collections::HashSet::new();
        calendar.events.retain(|e| seen.insert(e.id.clone()));
    }
}

/// Initialize scheduling module
pub fn init() -> Result<(), String> {
    Ok(())
}
