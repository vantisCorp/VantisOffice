//! Integration tests for WASM Sandbox

use wasm_sandbox::{init};
use wasm_sandbox::security::Capability;
use wasm_sandbox::runtime::Sandbox;

#[test]
fn test_sandbox_initialization() {
    let result = init();
    assert!(result.is_ok(), "Sandbox initialization should succeed");
}

#[test]
fn test_sandbox_creation() {
    let sandbox = Sandbox::new();
    assert!(sandbox.is_ok(), "Sandbox creation should succeed");
}

#[test]
fn test_capability_minimal() {
    let capability = Capability::minimal();
    assert!(
        !capability.permissions.file_read,
        "Minimal capability should not allow file read"
    );
    assert!(
        !capability.permissions.file_write,
        "Minimal capability should not allow file write"
    );
    assert!(
        !capability.permissions.network_access,
        "Minimal capability should not allow network access"
    );
}

#[test]
fn test_capability_with_file_read() {
    let capability = Capability::minimal().with_file_read();
    assert!(
        capability.permissions.file_read,
        "Capability should allow file read"
    );
    assert!(
        !capability.permissions.file_write,
        "Capability should not allow file write"
    );
}

#[test]
fn test_capability_with_memory_limit() {
    let capability = Capability::minimal().with_memory_limit(128 * 1024 * 1024);
    assert_eq!(
        capability.quotas.max_memory,
        128 * 1024 * 1024,
        "Memory limit should be set"
    );
}
