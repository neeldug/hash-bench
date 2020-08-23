use criterion::{criterion_group, criterion_main, Criterion};
use hashbrown::HashMap;
use rustc_hash::FxHashMap;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
#[cfg_attr(
    all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"),
    global_allocator
)]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn construction_map_ahash() {
    for n in 0..500000 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(n, 0);
    }
}

fn construction_map_rustc() {
    for n in 0..500000 {
        let mut map: FxHashMap<i32, i32> = FxHashMap::default();
        map.insert(n, 0);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Construction - b - aHash", |b| {
        b.iter(|| construction_map_ahash())
    });
    c.bench_function("Construction - b - rustc-hash", |b| {
        b.iter(|| construction_map_rustc())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
