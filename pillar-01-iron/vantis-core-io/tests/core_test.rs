//! Unit tests for Vantis-Core-IO

use vantis_core_io::crypto::sha3_hash;
use vantis_core_io::{init, FileHandle, OpenFlags, VirtualFileSystem};

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
fn test_open_flags_read() {
    let flags = OpenFlags::Read;
    assert!(matches!(flags, OpenFlags::Read));
}

#[test]
fn test_open_flags_write() {
    let flags = OpenFlags::Write;
    assert!(matches!(flags, OpenFlags::Write));
}

#[test]
fn test_open_flags_create() {
    let flags = OpenFlags::Create;
    assert!(matches!(flags, OpenFlags::Create));
}

#[test]
fn test_open_flags_truncate() {
    let flags = OpenFlags::Truncate;
    assert!(matches!(flags, OpenFlags::Truncate));
}

#[test]
fn test_open_flags_readwrite() {
    let flags = OpenFlags::ReadWrite;
    assert!(matches!(flags, OpenFlags::ReadWrite));
}

#[test]
fn test_virtual_filesystem() {
    let fs = VirtualFileSystem::new();
    assert!(fs.is_ok());

    let _vfs = fs.unwrap();
    // VirtualFileSystem is created successfully
}

#[test]
fn test_virtual_filesystem_mount() {
    let mut vfs = VirtualFileSystem::new().unwrap();
    let result = vfs.mount("/mnt");
    assert!(result.is_ok());
}

#[test]
fn test_virtual_filesystem_unmount() {
    let mut vfs = VirtualFileSystem::new().unwrap();
    vfs.mount("/mnt").unwrap();
    let result = vfs.unmount("/mnt");
    assert!(result.is_ok());
}

#[test]
fn test_sha3_hashing() {
    let data = b"Hello, VantisOffice!";
    let hash = sha3_hash(data);
    assert!(hash.is_ok());

    let hash = hash.unwrap();
    assert_eq!(hash.len(), 32); // SHA3-256 produces 32 bytes
    assert_ne!(hash, vec![0u8; 32]);
}

#[test]
fn test_hash_consistency() {
    let data = b"Test data";
    let hash1 = sha3_hash(data).unwrap();
    let hash2 = sha3_hash(data).unwrap();
    assert_eq!(hash1, hash2);
}

#[test]
fn test_hash_uniqueness() {
    let data1 = b"Data 1";
    let data2 = b"Data 2";
    let hash1 = sha3_hash(data1).unwrap();
    let hash2 = sha3_hash(data2).unwrap();
    assert_ne!(hash1, hash2);
}

#[test]
fn test_hash_empty_data() {
    let data = b"";
    let hash = sha3_hash(data);
    assert!(hash.is_ok());

    let hash = hash.unwrap();
    assert_eq!(hash.len(), 32);
}

#[test]
fn test_hash_large_data() {
    let data = vec![0u8; 10000];
    let hash = sha3_hash(&data);
    assert!(hash.is_ok());

    let hash = hash.unwrap();
    assert_eq!(hash.len(), 32);
}

#[test]
fn test_file_handle_path() {
    let handle = FileHandle::open("path/to/file.txt", OpenFlags::Read).unwrap();
    assert_eq!(handle.path(), "path/to/file.txt");
}

#[test]
fn test_file_handle_read_all() {
    let handle = FileHandle::open("test.txt", OpenFlags::Read).unwrap();
    let result = handle.read_all();
    assert!(result.is_ok());
}

#[test]
fn test_file_handle_write() {
    let handle = FileHandle::open("test.txt", OpenFlags::Write).unwrap();
    let data = b"Hello, Vantis!";
    let result = handle.write(data);
    assert!(result.is_ok());
}

#[test]
fn test_file_handle_readwrite() {
    let handle = FileHandle::open("test.txt", OpenFlags::ReadWrite).unwrap();
    let data = b"Hello, Vantis!";
    let result = handle.write(data);
    assert!(result.is_ok());

    let result = handle.read_all();
    assert!(result.is_ok());
}

#[test]
fn test_file_handle_create() {
    let handle = FileHandle::open("newfile.txt", OpenFlags::Create);
    assert!(handle.is_ok());
}

#[test]
fn test_file_handle_truncate() {
    let handle = FileHandle::open("existing.txt", OpenFlags::Truncate);
    assert!(handle.is_ok());
}
