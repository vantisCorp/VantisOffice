//! Integration tests for Vantis-Core-IO

use vantis_core_io::{init, FileHandle, OpenFlags};

#[test]
fn test_initialization() {
    let result = init();
    assert!(result.is_ok(), "Initialization should succeed");
}

#[test]
fn test_file_handle_creation() {
    let handle = FileHandle::open("test.txt", OpenFlags::Read);
    assert!(handle.is_ok(), "File handle creation should succeed");
    
    let handle = handle.unwrap();
    assert_eq!(handle.path(), "test.txt");
}

#[test]
fn test_read_all() {
    let handle = FileHandle::open("test.txt", OpenFlags::Read).unwrap();
    let result = handle.read_all();
    assert!(result.is_ok(), "Read operation should succeed");
}

#[test]
fn test_write() {
    let handle = FileHandle::open("test.txt", OpenFlags::Write).unwrap();
    let data = b"Hello, Vantis!";
    let result = handle.write(data);
    assert!(result.is_ok(), "Write operation should succeed");
}
