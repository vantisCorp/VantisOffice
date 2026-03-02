//! UI module for calendar views and editors

use crate::core::{Calendar, Event, Color, DateRange, TimeRange};
use chrono::{DateTime, Utc, NaiveDate, Weekday};

/// Calendar view
#[derive(Debug, Clone)]
pub struct CalendarView {
    pub view_type: ViewType,
    pub calendar: Calendar,
    pub settings: ViewSettings,
}

impl CalendarView {
    pub fn new(calendar: Calendar, view_type: ViewType) -> Self {
        CalendarView {
            view_type,
            calendar,
            settings: ViewSettings::default(),
        }
    }
    
    pub fn render(&self) -> String {
        match self.view_type {
            ViewType::Day => self.render_day_view(),
            ViewType::Week => self.render_week_view(),
            ViewType::Month => self.render_month_view(),
            ViewType::Year => self.render_year_view(),
            ViewType::Agenda => self.render_agenda_view(),
            ViewType::Timeline => self.render_timeline_view(),
        }
    }
    
    fn render_day_view(&self) -> String {
        format!("Day View for {}\nEvents: {}", 
            self.calendar.name,
            self.calendar.events.len()
        )
    }
    
    fn render_week_view(&self) -> String {
        format!("Week View for {}\nEvents: {}", 
            self.calendar.name,
            self.calendar.events.len()
        )
    }
    
    fn render_month_view(&self) -> String {
        format!("Month View for {}\nEvents: {}", 
            self.calendar.name,
            self.calendar.events.len()
        )
    }
    
    fn render_year_view(&self) -> String {
        format!("Year View for {}\nEvents: {}", 
            self.calendar.name,
            self.calendar.events.len()
        )
    }
    
    fn render_agenda_view(&self) -> String {
        format!("Agenda View for {}\nEvents: {}", 
            self.calendar.name,
            self.calendar.events.len()
        )
    }
    
    fn render_timeline_view(&self) -> String {
        format!("Timeline View for {}\nEvents: {}", 
            self.calendar.name,
            self.calendar.events.len()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    Day,
    Week,
    Month,
    Year,
    Agenda,
    Timeline,
}

/// Event editor
#[derive(Debug, Clone)]
pub struct EventEditor {
    pub event: Option<Event>,
}

impl EventEditor {
    pub fn new() -> Self {
        EventEditor {
            event: None,
        }
    }
    
    pub fn with_event(event: Event) -> Self {
        EventEditor {
            event: Some(event),
        }
    }
    
    pub fn create_event(&self, title: String, start: DateTime<Utc>, end: DateTime<Utc>) -> Event {
        Event::new(title, start, end)
    }
    
    pub fn update_event(&self, event: &mut Event, title: Option<String>, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) {
        if let Some(new_title) = title {
            event.title = new_title;
        }
        if let Some(new_start) = start {
            event.start = new_start;
        }
        if let Some(new_end) = end {
            event.end = new_end;
        }
        event.updated_at = Utc::now();
    }
}

/// Timeline view
#[derive(Debug, Clone)]
pub struct TimelineView {
    pub events: Vec<Event>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl TimelineView {
    pub fn new(events: Vec<Event>, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        TimelineView {
            events,
            start_date,
            end_date,
        }
    }
    
    pub fn render(&self) -> String {
        let mut output = String::from("Timeline View\n");
        output.push_str(&format!("Period: {} to {}\n\n", 
            self.start_date.format("%Y-%m-%d"),
            self.end_date.format("%Y-%m-%d")
        ));
        
        for event in &self.events {
            output.push_str(&format!("  - {} ({} to {})\n",
                event.title,
                event.start.format("%Y-%m-%d %H:%M"),
                event.end.format("%Y-%m-%d %H:%M")
            ));
        }
        
        output
    }
}

/// Agenda view
#[derive(Debug, Clone)]
pub struct AgendaView {
    pub events: Vec<Event>,
    pub date: NaiveDate,
}

impl AgendaView {
    pub fn new(events: Vec<Event>, date: NaiveDate) -> Self {
        AgendaView {
            events,
            date,
        }
    }
    
    pub fn render(&self) -> String {
        let mut output = format!("Agenda for {}\n\n", self.date);
        
        for event in &self.events {
            output.push_str(&format!("  {} - {}: {}\n",
                event.start.format("%H:%M"),
                event.end.format("%H:%M"),
                event.title
            ));
        }
        
        output
    }
}

/// View settings
#[derive(Debug, Clone)]
pub struct ViewSettings {
    pub show_weekend: bool,
    pub start_hour: u32,
    pub end_hour: u32,
    pub time_slot_duration: u32,
}

impl Default for ViewSettings {
    fn default() -> Self {
        ViewSettings {
            show_weekend: true,
            start_hour: 8,
            end_hour: 18,
            time_slot_duration: 30,
        }
    }
}