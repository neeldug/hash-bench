use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashbrown::HashMap;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rustc_hash::FxHashMap;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
#[cfg_attr(
    all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"),
    global_allocator
)]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn lookup_ahash(iterations: usize) {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut small_rng = SmallRng::from_entropy();

    for _n in 0..iterations {
        map.insert(small_rng.gen(), 0);
    }
    for _n in 0..iterations {
        map.get(&(small_rng.gen()));
    }
}

fn lookup_rustc(iterations: usize) {
    let mut map: FxHashMap<usize, usize> = FxHashMap::default();
    let mut small_rng = SmallRng::from_entropy();

    for _n in 0..iterations {
        map.insert(small_rng.gen(), 0);
    }
    for _n in 0..iterations {
        map.get(&(small_rng.gen()));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Lookup 256 iter - aHash", |b| {
        b.iter(|| lookup_ahash(black_box(256)))
    });
    c.bench_function("Lookup 256 iter - rustc-hash", |b| {
        b.iter(|| lookup_rustc(black_box(256)))
    });
    c.bench_function("Lookup 1024 iter - aHash", |b| {
        b.iter(|| lookup_ahash(black_box(1024)))
    });
    c.bench_function("Lookup 1024 iter - rustc-hash", |b| {
        b.iter(|| lookup_rustc(black_box(1024)))
    });
    c.bench_function("Lookup ~1.05M iter - aHash", |b| {
        b.iter(|| lookup_ahash(black_box(1048576)))
    });
    c.bench_function("Lookup ~1.05M iter - rustc-hash", |b| {
        b.iter(|| lookup_rustc(black_box(1048576)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
