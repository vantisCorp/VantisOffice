// Performance benchmarks for WASM Sandbox
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use wasm_sandbox::{
    ExecutionResult, Plugin, PluginCapability, ResourceLimits, SandboxConfig, WasmSandbox,
};

fn bench_sandbox_creation(c: &mut Criterion) {
    c.bench_function("wasm_sandbox_creation", |b| {
        b.iter(|| WasmSandbox::new(SandboxConfig::default()));
    });
}

fn bench_plugin_loading(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();

    c.bench_function("wasm_plugin_loading", |b| {
        b.iter(|| sandbox.load_plugin(black_box(&wasm_bytes)));
    });
}

fn bench_plugin_instantiation(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();

    c.bench_function("wasm_plugin_instantiation", |b| {
        b.iter(|| sandbox.instantiate_plugin(black_box(plugin_id)));
    });
}

fn bench_plugin_execution_simple(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();
    let instance_id = sandbox.instantiate_plugin(plugin_id).unwrap();

    c.bench_function("wasm_plugin_execution_simple", |b| {
        b.iter(|| {
            sandbox.execute_function(
                black_box(instance_id),
                black_box("simple_function"),
                black_box(&[]),
            )
        });
    });
}

fn bench_plugin_execution_complex(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();
    let instance_id = sandbox.instantiate_plugin(plugin_id).unwrap();

    c.bench_function("wasm_plugin_execution_complex", |b| {
        b.iter(|| {
            sandbox.execute_function(
                black_box(instance_id),
                black_box("complex_function"),
                black_box(&[1, 2, 3, 4, 5]),
            )
        });
    });
}

fn bench_plugin_execution_with_memory(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();
    let instance_id = sandbox.instantiate_plugin(plugin_id).unwrap();

    let data = vec![0u8; 1024];

    c.bench_function("wasm_plugin_execution_with_memory", |b| {
        b.iter(|| {
            sandbox.execute_with_memory(
                black_box(instance_id),
                black_box("process_data"),
                black_box(&data),
            )
        });
    });
}

fn bench_resource_limits(c: &mut Criterion) {
    let mut group = c.benchmark_group("wasm_resource_limits");

    let limits = vec![
        (
            "strict",
            ResourceLimits {
                max_memory: 1_000_000,
                max_cpu_time: 100,
                max_file_handles: 10,
                max_network_connections: 0,
            },
        ),
        (
            "standard",
            ResourceLimits {
                max_memory: 10_000_000,
                max_cpu_time: 1000,
                max_file_handles: 50,
                max_network_connections: 5,
            },
        ),
        (
            "relaxed",
            ResourceLimits {
                max_memory: 100_000_000,
                max_cpu_time: 10000,
                max_file_handles: 100,
                max_network_connections: 20,
            },
        ),
    ];

    for (name, limit) in limits.iter() {
        let config = SandboxConfig {
            resource_limits: *limit,
            ..Default::default()
        };

        group.bench_with_input(BenchmarkId::from_parameter(name), limit, |b, _| {
            b.iter(|| {
                let sandbox = WasmSandbox::new(config.clone()).unwrap();
                let wasm_bytes = create_test_wasm_module();
                sandbox.load_plugin(&wasm_bytes).unwrap();
                sandbox
            });
        });
    }

    group.finish();
}

fn bench_capability_system(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();

    c.bench_function("wasm_capability_check", |b| {
        b.iter(|| {
            sandbox.check_capability(
                black_box(plugin_id),
                black_box(PluginCapability::FileSystemRead),
            )
        });
    });
}

fn bench_capability_grant(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();

    c.bench_function("wasm_capability_grant", |b| {
        b.iter(|| {
            sandbox.grant_capability(
                black_box(plugin_id),
                black_box(PluginCapability::FileSystemRead),
            )
        });
    });
}

fn bench_capability_revoke(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();
    sandbox
        .grant_capability(plugin_id, PluginCapability::FileSystemRead)
        .unwrap();

    c.bench_function("wasm_capability_revoke", |b| {
        b.iter(|| {
            sandbox.revoke_capability(
                black_box(plugin_id),
                black_box(PluginCapability::FileSystemRead),
            )
        });
    });
}

fn bench_multiple_plugins(c: &mut Criterion) {
    let mut group = c.benchmark_group("wasm_multiple_plugins");

    for count in [1, 5, 10, 20].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, count| {
            b.iter(|| {
                let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
                let wasm_bytes = create_test_wasm_module();

                for _ in 0..*count {
                    sandbox.load_plugin(&wasm_bytes).unwrap();
                }

                sandbox
            });
        });
    }

    group.finish();
}

fn bench_plugin_hot_reload(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes_v1 = create_test_wasm_module();
    let wasm_bytes_v2 = create_test_wasm_module();

    let plugin_id = sandbox.load_plugin(&wasm_bytes_v1).unwrap();
    let instance_id = sandbox.instantiate_plugin(plugin_id).unwrap();

    c.bench_function("wasm_plugin_hot_reload", |b| {
        b.iter(|| sandbox.reload_plugin(black_box(plugin_id), black_box(&wasm_bytes_v2)));
    });
}

fn bench_inter_plugin_communication(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();

    let plugin_a = sandbox.load_plugin(&wasm_bytes).unwrap();
    let plugin_b = sandbox.load_plugin(&wasm_bytes).unwrap();

    let instance_a = sandbox.instantiate_plugin(plugin_a).unwrap();
    let instance_b = sandbox.instantiate_plugin(plugin_b).unwrap();

    let message = vec![1, 2, 3, 4, 5];

    c.bench_function("wasm_inter_plugin_communication", |b| {
        b.iter(|| {
            sandbox.send_message(
                black_box(instance_a),
                black_box(instance_b),
                black_box(&message),
            )
        });
    });
}

fn bench_plugin_state_serialization(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();
    let instance_id = sandbox.instantiate_plugin(plugin_id).unwrap();

    c.bench_function("wasm_plugin_state_serialization", |b| {
        b.iter(|| sandbox.serialize_state(black_box(instance_id)));
    });
}

fn bench_plugin_state_deserialization(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig::default()).unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();
    let instance_id = sandbox.instantiate_plugin(plugin_id).unwrap();

    let serialized = sandbox.serialize_state(instance_id).unwrap();

    c.bench_function("wasm_plugin_state_deserialization", |b| {
        b.iter(|| sandbox.deserialize_state(black_box(&serialized)));
    });
}

fn bench_security_monitoring(c: &mut Criterion) {
    let sandbox = WasmSandbox::new(SandboxConfig {
        enable_monitoring: true,
        ..Default::default()
    })
    .unwrap();
    let wasm_bytes = create_test_wasm_module();
    let plugin_id = sandbox.load_plugin(&wasm_bytes).unwrap();
    let instance_id = sandbox.instantiate_plugin(plugin_id).unwrap();

    c.bench_function("wasm_security_monitoring", |b| {
        b.iter(|| {
            sandbox.execute_function(
                black_box(instance_id),
                black_box("simple_function"),
                black_box(&[]),
            )
        });
    });
}

// Helper function to create test WASM module
fn create_test_wasm_module() -> Vec<u8> {
    // Minimal WASM module bytes (placeholder - in production would load actual .wasm file)
    vec![
        0x00, 0x61, 0x73, 0x6d, // WASM magic
        0x01, 0x00, 0x00, 0x00, // Version
              // ... minimal module structure
    ]
}

criterion_group!(
    benches,
    bench_sandbox_creation,
    bench_plugin_loading,
    bench_plugin_instantiation,
    bench_plugin_execution_simple,
    bench_plugin_execution_complex,
    bench_plugin_execution_with_memory,
    bench_resource_limits,
    bench_capability_system,
    bench_capability_grant,
    bench_capability_revoke,
    bench_multiple_plugins,
    bench_plugin_hot_reload,
    bench_inter_plugin_communication,
    bench_plugin_state_serialization,
    bench_plugin_state_deserialization,
    bench_security_monitoring
);

criterion_main!(benches);
