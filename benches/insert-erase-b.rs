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

fn random_insertion_map_ahash(bitmask: u64) {
    let mut small_rng = SmallRng::from_entropy();
    let mut map: HashMap<u64, u64> = HashMap::new();
    for _n in 0..10000000 {
        map.insert(small_rng.gen::<u64>() & bitmask, 0);
        map.remove(&(small_rng.gen::<u64>() & bitmask));
    }
}

fn random_insertion_map_rustc(bitmask: u64) {
    let mut small_rng = SmallRng::from_entropy();
    let mut map: FxHashMap<u64, u64> = FxHashMap::default();
    for _n in 0..10000000 {
        map.insert(small_rng.gen::<u64>() & bitmask, 0);
        map.remove(&(small_rng.gen::<u64>() & bitmask));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    // Bitmask Small ~8 entries
    c.bench_function("Random insertion deletion - aHash - Small", |b| {
        b.iter(|| random_insertion_map_ahash(black_box(10376293541462671368)))
    });
    c.bench_function("Random insertion deletion - rustc-hash - Small", |b| {
        b.iter(|| random_insertion_map_rustc(10376293541462671368))
    });
    // Bitmask Medium ~128 entries
    c.bench_function("Random insertion deletion - aHash - Medium", |b| {
        b.iter(|| random_insertion_map_ahash(black_box(10376909267974488072)))
    });
    c.bench_function("Random insertion deletion - rustc-hash - Medium", |b| {
        b.iter(|| random_insertion_map_rustc(black_box(10376909267974488072)))
    });
    // Bitmask Large ~2048 entries
    c.bench_function("Random insertion deletion - aHash - Large", |b| {
        b.iter(|| random_insertion_map_ahash(black_box(10378035168149897225)))
    });
    c.bench_function("Random insertion deletion - rustc-hash - Large", |b| {
        b.iter(|| random_insertion_map_rustc(black_box(10378035168149897225)))
    });
    // Bitmask XL ~32.8k entries
    c.bench_function("Random insertion deletion - aHash - XL", |b| {
        b.iter(|| random_insertion_map_ahash(black_box(10378035236869439785)))
    });
    c.bench_function("Random insertion deletion - rustc-hash - XL", |b| {
        b.iter(|| random_insertion_map_rustc(black_box(10378035236869439785)))
    });
    // Bitmask XXL ~524k entries
    c.bench_function("Random insertion deletion - aHash - XXL", |b| {
        b.iter(|| random_insertion_map_ahash(black_box(15566182557356069161)))
    });
    c.bench_function("Random insertion deletion - rustc-hash - XXL", |b| {
        b.iter(|| random_insertion_map_rustc(black_box(15566182557356069161)))
    });
    // Bitmask XXXL ~8.4M entries
    c.bench_function("Random insertion deletion - aHash - XXL", |b| {
        b.iter(|| random_insertion_map_ahash(black_box(15568434365768077611)))
    });
    c.bench_function("Random insertion deletion - rustc-hash - XXL", |b| {
        b.iter(|| random_insertion_map_rustc(black_box(15568434365768077611)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
