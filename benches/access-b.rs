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

fn lookup_ahash(length: usize, iterations: usize) {
    let mut map: HashMap<String, String> = HashMap::new();

    for _n in 0..iterations {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect();
        map.insert(rand_string, "".to_string());
    }
    for _n in 0..iterations {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect();
        map.get(&rand_string);
    }
}

fn lookup_rustc(length: usize, iterations: usize) {
    let mut map: FxHashMap<String, String> = FxHashMap::default();

    for _n in 0..iterations {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect();
        map.insert(rand_string, "".to_string());
    }
    for _n in 0..iterations {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect();
        map.get(&rand_string);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Lookup string 7B - aHash", |b| {
        b.iter(|| lookup_ahash(black_box(7), black_box(100000)))
    });
    c.bench_function("Lookup string 7B - rustc-hash", |b| {
        b.iter(|| lookup_rustc(black_box(7), black_box(100000)))
    });
    c.bench_function("Lookup string 8B - aHash", |b| {
        b.iter(|| lookup_ahash(black_box(8), black_box(100000)))
    });
    c.bench_function("Lookup string 8B - rustc-hash", |b| {
        b.iter(|| lookup_rustc(black_box(8), black_box(100000)))
    });
    c.bench_function("Lookup string 128B - aHash", |b| {
        b.iter(|| lookup_ahash(black_box(128), black_box(50000)))
    });
    c.bench_function("Lookup string 128B - rustc-hash", |b| {
        b.iter(|| lookup_rustc(black_box(128), black_box(50000)))
    });
    c.bench_function("Lookup string 1024B - aHash", |b| {
        b.iter(|| lookup_ahash(black_box(1024), black_box(5000)))
    });
    c.bench_function("Lookup string 1024B - rustc-hash", |b| {
        b.iter(|| lookup_rustc(black_box(1024), black_box(5000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
