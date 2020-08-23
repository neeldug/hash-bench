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
    let mut result = 0;
    for _n in 0..100000000 {
        let map: HashMap<i32, i32> = HashMap::new();
        result += map.capacity();
    }
    assert_eq!(result, 0);
}

fn construction_map_rustc() {
    let mut result = 0;
    for _n in 0..100000000 {
        let map: FxHashMap<i32, i32> = FxHashMap::default();
        result += map.capacity();
    }
    assert_eq!(result, 0);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Construction - a - aHash", |b| {
        b.iter(|| construction_map_ahash())
    });
    c.bench_function("Construction - a - rustc-hash", |b| {
        b.iter(|| construction_map_rustc())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
