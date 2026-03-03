// Performance benchmarks for Vantis Link
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use vantis_link::core::{Session, User, Document, Change, ChangeType, UserRole};

fn benchmark_session_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("session_creation");
    
    group.bench_function("create_session", |b| {
        b.iter(|| {
            black_box(Session::new("doc-123".to_string()))
        })
    });
    
    group.bench_function("create_session_with_metadata", |b| {
        b.iter(|| {
            let mut session = Session::new("doc-123".to_string());
            session.metadata.name = Some("Test Session".to_string());
            session.metadata.max_users = Some(10);
            black_box(session)
        })
    });
    
    group.finish();
}

fn benchmark_user_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("user_operations");
    
    let user_counts = vec![1, 5, 10, 50];
    
    for count in user_counts {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter(|| {
                let mut session = Session::new("doc-123".to_string());
                for i in 0..count {
                    let user = User::new(format!("user-{}", i), format!("User {}", i));
                    let _ = session.add_user(user);
                }
                black_box(session)
            })
        });
    }
    
    group.finish();
}

fn benchmark_session_user_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("session_user_management");
    
    group.bench_function("add_user", |b| {
        b.iter(|| {
            let mut session = Session::new("doc-123".to_string());
            let user = User::new("user-1".to_string(), "User 1".to_string());
            black_box(session.add_user(user))
        })
    });
    
    group.bench_function("remove_user", |b| {
        b.iter(|| {
            let mut session = Session::new("doc-123".to_string());
            let user = User::new("user-1".to_string(), "User 1".to_string());
            let _ = session.add_user(user);
            black_box(session.remove_user("user-1"))
        })
    });
    
    group.bench_function("get_user", |b| {
        let mut session = Session::new("doc-123".to_string());
        let user = User::new("user-1".to_string(), "User 1".to_string());
        let _ = session.add_user(user);
        
        b.iter(|| {
            black_box(session.get_user("user-1"))
        })
    });
    
    group.finish();
}

fn benchmark_document_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("document_operations");
    
    group.bench_function("create_document", |b| {
        b.iter(|| {
            black_box(Document::new("test-doc".to_string(), "Test Document".to_string()))
        })
    });
    
    group.bench_function("document_with_content", |b| {
        b.iter(|| {
            let mut doc = Document::new("test-doc".to_string(), "Test Document".to_string());
            doc.content = "Sample document content".to_string();
            black_box(doc)
        })
    });
    
    group.finish();
}

fn benchmark_change_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("change_operations");
    
    group.bench_function("create_change", |b| {
        b.iter(|| {
            black_box(Change::new("user-1".to_string(), ChangeType::Insert, 0, "test".to_string()))
        })
    });
    
    group.bench_function("create_multiple_changes", |b| {
        b.iter(|| {
            let mut changes = Vec::new();
            for i in 0..100 {
                changes.push(Change::new("user-1".to_string(), ChangeType::Insert, i, "a".to_string()));
            }
            black_box(changes)
        })
    });
    
    group.finish();
}

fn benchmark_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    group.bench_function("serialize_session", |b| {
        let session = Session::new("doc-123".to_string());
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&session)).unwrap())
        })
    });
    
    group.bench_function("deserialize_session", |b| {
        let session = Session::new("doc-123".to_string());
        let json = serde_json::to_string(&session).unwrap();
        b.iter(|| {
            black_box(serde_json::from_str::<Session>(black_box(&json)).unwrap())
        })
    });
    
    group.bench_function("serialize_user", |b| {
        let user = User::new("user-1".to_string(), "User 1".to_string());
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&user)).unwrap())
        })
    });
    
    group.finish();
}

fn benchmark_session_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("session_queries");
    
    let user_counts = vec![10, 50, 100, 500];
    
    for count in user_counts {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let mut session = Session::new("doc-123".to_string());
            for i in 0..count {
                let user = User::new(format!("user-{}", i), format!("User {}", i));
                let _ = session.add_user(user);
            }
            
            b.iter(|| {
                black_box(session.user_count())
            })
        });
    }
    
    group.finish();
}

fn benchmark_role_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("role_operations");
    
    let roles = vec![UserRole::Owner, UserRole::Admin, UserRole::Editor, UserRole::Viewer];
    
    for role in &roles {
        group.bench_with_input(BenchmarkId::from_parameter(format!("{:?}", role)), role, |b, role| {
            b.iter(|| {
                let mut user = User::new("user-1".to_string(), "User 1".to_string());
                user.role = role.clone();
                black_box(user)
            })
        });
    }
    
    group.finish();
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    
    group.bench_function("concurrent_user_adds", |b| {
        b.iter(|| {
            let mut sessions = Vec::new();
            for i in 0..10 {
                let mut session = Session::new(format!("doc-{}", i));
                for j in 0..5 {
                    let user = User::new(format!("user-{}", j), format!("User {}", j));
                    let _ = session.add_user(user);
                }
                sessions.push(session);
            }
            black_box(sessions)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_session_creation,
    benchmark_user_operations,
    benchmark_session_user_management,
    benchmark_document_operations,
    benchmark_change_operations,
    benchmark_serialization,
    benchmark_session_queries,
    benchmark_role_operations,
    benchmark_concurrent_operations
);
criterion_main!(benches);