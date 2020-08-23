use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashbrown::HashMap;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rustc_hash::FxHashMap;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
#[cfg_attr(
    all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"),
    global_allocator
)]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn insert_erase_string_ahash(length: usize, iterations: usize) {
    let mut map: HashMap<String, String> = HashMap::new();
    for _n in 0..iterations {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect();
        map.insert(rand_string, "".to_string());
        let new_rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect();
        map.remove(&new_rand_string);
    }
}

fn insert_erase_string_rustc(length: usize, iterations: usize) {
    let mut map: FxHashMap<String, String> = FxHashMap::default();
    for _n in 0..iterations {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect();
        map.insert(rand_string, "".to_string());
        let new_rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect();
        map.remove(&new_rand_string);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Random insertion erasure string 7B - aHash", |b| {
        b.iter(|| insert_erase_string_ahash(black_box(7), black_box(200000)))
    });
    c.bench_function("Random insertion erasure string 7B - rustc-hash", |b| {
        b.iter(|| insert_erase_string_rustc(black_box(7), black_box(200000)))
    });

    c.bench_function("Random insertion erasure string 8B - aHash", |b| {
        b.iter(|| insert_erase_string_ahash(black_box(8), black_box(200000)))
    });
    c.bench_function("Random insertion erasure string 8B - rustc-hash", |b| {
        b.iter(|| insert_erase_string_rustc(black_box(8), black_box(200000)))
    });

    c.bench_function("Random insertion erasure string 13B - aHash", |b| {
        b.iter(|| insert_erase_string_ahash(black_box(13), black_box(200000)))
    });
    c.bench_function("Random insertion erasure string 13B - rustc-hash", |b| {
        b.iter(|| insert_erase_string_rustc(black_box(13), black_box(200000)))
    });

    c.bench_function("Random insertion erasure string 100B - aHash", |b| {
        b.iter(|| insert_erase_string_ahash(black_box(100), black_box(120000)))
    });
    c.bench_function("Random insertion erasure string 100B - rustc-hash", |b| {
        b.iter(|| insert_erase_string_rustc(black_box(100), black_box(120000)))
    });

    c.bench_function("Random insertion erasure string 1000B - aHash", |b| {
        b.iter(|| insert_erase_string_ahash(black_box(1000), black_box(60000)))
    });
    c.bench_function("Random insertion erasure string 1000B - rustc-hash", |b| {
        b.iter(|| insert_erase_string_rustc(black_box(1000), black_box(60000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
