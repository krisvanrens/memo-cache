use criterion::{criterion_group, criterion_main, Criterion};
use memo_cache::MemoCache;
use rand::Rng;
use std::{collections::HashMap, thread, time};

fn fake_expensive_calculation() {
    thread::sleep(time::Duration::from_millis(1));
}

// Use a hash map cache (memory size: 201 * (32 + 32) bits = 1608 bytes).
fn bench_hash_map(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut cache = HashMap::new();
    c.bench_function("HashMap (control)", |b| {
        b.iter(|| {
            let input = rng.gen_range(-100..=100);
            if cache.get(&input).is_none() {
                fake_expensive_calculation();
                cache.insert(input, 42);
            }
        })
    });
}

// Use a 64-element fixed-size cache (memory size: 64 * (32 + 32) bits = 512 bytes).
fn bench_memo_cache64(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut cache = MemoCache::<_, _, 64>::new();
    c.bench_function("MemoCache (size: 64)", |b| {
        b.iter(|| {
            let input = rng.gen_range(-100..=100);
            if cache.get(&input).is_none() {
                fake_expensive_calculation();
                cache.insert(input, 42);
            }
        })
    });
}

criterion_group!(benches, bench_hash_map, bench_memo_cache64,);
criterion_main!(benches);
