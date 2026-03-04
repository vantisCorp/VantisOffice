// Performance benchmarks for Vantis Core-IO
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::io::Write;
use vantis_core_io::{crypto, FileHandle, OpenFlags, VirtualFileSystem};

fn benchmark_file_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_operations");

    // Benchmark file creation
    group.bench_function("create_file", |b| {
        b.iter(|| {
            let _handle = FileHandle::open(black_box("test.txt"), OpenFlags::Create).unwrap();
        })
    });

    // Benchmark file opening
    group.bench_function("open_file", |b| {
        b.iter(|| {
            let _handle = FileHandle::open(black_box("test.txt"), OpenFlags::Read).unwrap();
        })
    });

    // Benchmark file path access
    group.bench_function("file_path", |b| {
        let handle = FileHandle::open("test.txt", OpenFlags::Create).unwrap();
        b.iter(|| black_box(handle.path()))
    });

    group.finish();
}

fn benchmark_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashing");

    let sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024]; // 1KB, 10KB, 100KB, 1MB

    for size in sizes {
        let data = vec![b'x'; size];

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| black_box(crypto::sha3_hash(black_box(&data)).unwrap()))
        });
    }

    group.finish();
}

fn benchmark_integrity_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("integrity_verification");

    let sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024]; // 1KB, 10KB, 100KB, 1MB

    for size in sizes {
        let data = vec![b'x'; size];
        let hash = crypto::sha3_hash(&data).unwrap();

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| black_box(crypto::verify_integrity(black_box(&data), &hash)))
        });
    }

    group.finish();
}

fn benchmark_filesystem_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("filesystem_operations");

    // Benchmark filesystem creation
    group.bench_function("create_filesystem", |b| {
        b.iter(|| black_box(VirtualFileSystem::new().unwrap()))
    });

    // Benchmark mount operation
    group.bench_function("mount", |b| {
        let mut fs = VirtualFileSystem::new().unwrap();
        b.iter(|| fs.mount(black_box("/mnt")).unwrap())
    });

    // Benchmark unmount operation
    group.bench_function("unmount", |b| {
        let mut fs = VirtualFileSystem::new().unwrap();
        fs.mount("/mnt").unwrap();
        b.iter(|| fs.unmount(black_box("/mnt")).unwrap())
    });

    group.finish();
}

fn benchmark_combined_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("combined_operations");

    // Benchmark creating and hashing multiple files
    group.bench_function("create_and_hash_100_files", |b| {
        b.iter(|| {
            for i in 0..100 {
                let handle =
                    FileHandle::open(&format!("test_{}.txt", i), OpenFlags::Create).unwrap();
                let _path = handle.path();
                let data = vec![b'x'; 1024];
                let _hash = crypto::sha3_hash(&data).unwrap();
            }
        })
    });

    // Benchmark filesystem with multiple operations
    group.bench_function("filesystem_workflow", |b| {
        b.iter(|| {
            let mut fs = VirtualFileSystem::new().unwrap();
            fs.mount("/mnt").unwrap();
            let handle = FileHandle::open("test.txt", OpenFlags::Create).unwrap();
            let _path = handle.path();
            fs.unmount("/mnt").unwrap();
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_file_operations,
    benchmark_hashing,
    benchmark_integrity_verification,
    benchmark_filesystem_operations,
    benchmark_combined_operations
);
criterion_main!(benches);
