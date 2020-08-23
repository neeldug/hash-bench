use criterion::{criterion_group, criterion_main, Criterion};
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

/// https://martin.ankerl.com/2019/04/01/hashmap-benchmarks-03-01-result-InsertHugeInt/
/// Benchmarking 1M int
/// Unable to follow complete benchmark as CSPRNG's state is kept secret.
fn random_insertion_map_ahash() {
    let mut small_rng = SmallRng::from_entropy();
    let mut map: HashMap<i64, i64> = HashMap::new();
    for _n in 0..1000000 {
        map.insert(small_rng.gen(), 0);
    }
    map.clear();
    assert!(map.is_empty());
}

fn random_insertion_map_rustc() {
    let mut small_rng = SmallRng::from_entropy();
    let mut map: FxHashMap<i64, i64> = FxHashMap::default();
    for _n in 0..1000000 {
        map.insert(small_rng.gen(), 0);
    }
    map.clear();
    assert!(map.is_empty());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Random insertion - aHash", |b| {
        b.iter(|| random_insertion_map_ahash())
    });
    c.bench_function("Random insertion - rustc-hash", |b| {
        b.iter(|| random_insertion_map_rustc())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
