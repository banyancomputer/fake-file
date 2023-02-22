use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use lazy_static::lazy_static;
use std::{env, fs, path::Path, time};

use fake_file::{
    utils::fs_utils::{ensure_path_exists_and_is_dir, ensure_path_exists_and_is_empty_dir},
    Structure,
};

// Configure the Benching Framework from the Environment
lazy_static! {
    // Path we will use to hold our benchmarking data
    static ref BENCH_PATH: String = env::var("BENCH_PATH").unwrap_or_else(|_| "bench".to_string());
}

/// Ready a directory for benchmarking
#[doc(hidden)]
fn prep_generate(path: &Path) {
    // If the path exists, remove it, regardless of whether it is a file or directory
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

/// Bench the generation of a file structure
#[doc(hidden)]
fn structure(c: &mut Criterion) {
    // Get the Bench path and make sure it exists
    let bench_scratch_space = Path::new(BENCH_PATH.as_str());
    ensure_path_exists_and_is_dir(&bench_scratch_space).unwrap();
    // Get the input path and make sure it exists and is empty
    let balanced_path = bench_scratch_space.join("balanced");
    ensure_path_exists_and_is_empty_dir(&balanced_path, true).unwrap();
    // Declare a balanced file structure
    let structure = Structure::new(
        4,                  // width
        4,                  // depth
        1024 * 1024 * 1024, // target size in bytes (1Gb)
    );
    // Get a path to the file structure
    let structure_path = structure.to_path_string();
    // Get the full path to the file structure
    let full_path = balanced_path.join(structure_path);
    // Create the benchmark group
    let mut group = c.benchmark_group("balanced");
    // Add a throughput benchmark
    group.throughput(criterion::Throughput::Bytes(structure.target_size as u64));
    group.bench_function("balanced", |b| {
        b.iter_batched(
            || {
                // Prep the input path
                prep_generate(&full_path);
            },
            |_| {
                // Generate the file structure
                structure.generate(black_box(&full_path)).unwrap();
            },
            BatchSize::PerIteration,
        )
    });
    // Finish the benchmark group
    group.finish()
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(time::Duration::from_secs(30));
    targets = structure
}
criterion_main!(benches);
