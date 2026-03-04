// Performance benchmarks for Vantis Chronos
use chrono::{Duration, Utc};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use vantis_chronos::core::{
    Calendar, Color, Event, EventQuery, Recurrence, RecurrencePattern, Reminder,
};

fn benchmark_calendar_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("calendar_creation");

    group.bench_function("create_calendar", |b| {
        b.iter(|| {
            black_box(Calendar::new(
                "Test Calendar".to_string(),
                Color::new(0, 0, 255),
            ))
        })
    });

    group.bench_function("create_calendar_with_description", |b| {
        b.iter(|| {
            let mut cal = Calendar::new("Test Calendar".to_string(), Color::new(0, 0, 255));
            cal.description = Some("Test Description".to_string());
            black_box(cal)
        })
    });

    group.finish();
}

fn benchmark_event_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_creation");

    let now = Utc::now();

    group.bench_function("create_event", |b| {
        b.iter(|| {
            black_box(Event::new(
                "Test Event".to_string(),
                now,
                now + Duration::hours(1),
            ))
        })
    });

    group.bench_function("create_event_with_properties", |b| {
        b.iter(|| {
            let mut event = Event::new("Test Event".to_string(), now, now + Duration::hours(1));
            event.description = Some("Description".to_string());
            event.location = Some("Location".to_string());
            black_box(event)
        })
    });

    group.finish();
}

fn benchmark_event_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_operations");

    let event_counts = vec![10, 50, 100, 500];

    for count in event_counts {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter(|| {
                let mut calendar =
                    Calendar::new("Test Calendar".to_string(), Color::new(0, 0, 255));
                for i in 0..count {
                    let start = Utc::now() + Duration::days(i);
                    let event =
                        Event::new(format!("Event {}", i), start, start + Duration::hours(1));
                    calendar.add_event(event);
                }
                black_box(calendar)
            })
        });
    }

    group.finish();
}

fn benchmark_event_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_management");

    group.bench_function("add_event", |b| {
        b.iter(|| {
            let mut calendar = Calendar::new("Test Calendar".to_string(), Color::new(0, 0, 255));
            let event = Event::new(
                "Test Event".to_string(),
                Utc::now(),
                Utc::now() + Duration::hours(1),
            );
            black_box(calendar.add_event(event))
        })
    });

    group.bench_function("get_event", |b| {
        let mut calendar = Calendar::new("Test Calendar".to_string(), Color::new(0, 0, 255));
        let event = Event::new(
            "Test Event".to_string(),
            Utc::now(),
            Utc::now() + Duration::hours(1),
        );
        let event_id = event.id.clone();
        calendar.add_event(event);

        b.iter(|| black_box(calendar.get_event(&event_id)))
    });

    group.finish();
}

fn benchmark_event_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_queries");

    let event_counts = vec![10, 50, 100, 500];

    for count in event_counts {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let mut calendar = Calendar::new("Test Calendar".to_string(), Color::new(0, 0, 255));
            for i in 0..count {
                let start = Utc::now() + Duration::days(i);
                let event = Event::new(format!("Event {}", i), start, start + Duration::hours(1));
                calendar.add_event(event);
            }

            let query = EventQuery::new();
            b.iter(|| black_box(calendar.query_events(black_box(&query))))
        });
    }

    group.finish();
}

fn benchmark_recurrence(c: &mut Criterion) {
    let mut group = c.benchmark_group("recurrence");

    group.bench_function("create_recurrence", |b| {
        b.iter(|| {
            black_box(Recurrence {
                pattern: RecurrencePattern::Daily,
                interval: 1,
                until: None,
                count: None,
            })
        })
    });

    group.bench_function("event_with_recurrence", |b| {
        b.iter(|| {
            let mut event = Event::new(
                "Test Event".to_string(),
                Utc::now(),
                Utc::now() + Duration::hours(1),
            );
            event.recurrence = Some(Recurrence {
                pattern: RecurrencePattern::Weekly,
                interval: 1,
                until: Some(Utc::now() + Duration::weeks(4)),
                count: None,
            });
            black_box(event)
        })
    });

    group.finish();
}

fn benchmark_reminders(c: &mut Criterion) {
    let mut group = c.benchmark_group("reminders");

    group.bench_function("create_reminder", |b| {
        b.iter(|| {
            black_box(Reminder {
                id: "rem-1".to_string(),
                minutes_before: 15,
                method: vantis_chronos::core::ReminderMethod::Notification,
                sent: false,
            })
        })
    });

    group.bench_function("event_with_reminders", |b| {
        b.iter(|| {
            let mut event = Event::new(
                "Test Event".to_string(),
                Utc::now(),
                Utc::now() + Duration::hours(1),
            );
            event.reminders = vec![
                Reminder {
                    id: "rem-1".to_string(),
                    minutes_before: 15,
                    method: vantis_chronos::core::ReminderMethod::Notification,
                    sent: false,
                },
                Reminder {
                    id: "rem-2".to_string(),
                    minutes_before: 60,
                    method: vantis_chronos::core::ReminderMethod::Email,
                    sent: false,
                },
            ];
            black_box(event)
        })
    });

    group.finish();
}

fn benchmark_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");

    group.bench_function("serialize_calendar", |b| {
        let calendar = Calendar::new("Test Calendar".to_string(), Color::new(0, 0, 255));
        b.iter(|| black_box(serde_json::to_string(black_box(&calendar)).unwrap()))
    });

    group.bench_function("deserialize_calendar", |b| {
        let calendar = Calendar::new("Test Calendar".to_string(), Color::new(0, 0, 255));
        let json = serde_json::to_string(&calendar).unwrap();
        b.iter(|| black_box(serde_json::from_str::<Calendar>(black_box(&json)).unwrap()))
    });

    group.bench_function("serialize_event", |b| {
        let event = Event::new(
            "Test Event".to_string(),
            Utc::now(),
            Utc::now() + Duration::hours(1),
        );
        b.iter(|| black_box(serde_json::to_string(black_box(&event)).unwrap()))
    });

    group.finish();
}

fn benchmark_overlap_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("overlap_detection");

    group.bench_function("check_overlap", |b| {
        let event1 = Event::new(
            "Event 1".to_string(),
            Utc::now(),
            Utc::now() + Duration::hours(1),
        );
        let event2 = Event::new(
            "Event 2".to_string(),
            Utc::now() + Duration::minutes(30),
            Utc::now() + Duration::hours(2),
        );

        b.iter(|| black_box(black_box(&event1).overlaps(black_box(&event2))))
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_calendar_creation,
    benchmark_event_creation,
    benchmark_event_operations,
    benchmark_event_management,
    benchmark_event_queries,
    benchmark_recurrence,
    benchmark_reminders,
    benchmark_serialization,
    benchmark_overlap_detection
);
criterion_main!(benches);
