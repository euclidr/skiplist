#[macro_use]
extern crate criterion;

use criterion::Criterion;
use skiplist::skipset::SkipSet;
use std::collections::{BTreeSet, HashSet};

fn skipset_cardinal_close_benchmark(c: &mut Criterion) {
    let mut sets1 = SkipSet::new();
    let mut sets2 = SkipSet::new();
    for i in 0..500 {
        sets1.add(i * 2);
        sets2.add(i * 3);
    }

    c.bench_function("skipset close cardinal difference traverse", |b| {
        b.iter(|| {
            let _: Vec<_> = sets1.difference_traverse(&sets2).collect();
        })
    });

    c.bench_function("skipset close cardinal difference search", |b| {
        b.iter(|| {
            let _: Vec<_> = sets1.difference_search(&sets2).collect();
        })
    });
}

fn hashset_cardinal_close_benchmark(c: &mut Criterion) {
    let mut sets1 = HashSet::new();
    let mut sets2 = HashSet::new();
    for i in 0..500 {
        sets1.insert(i * 2);
        sets2.insert(i * 3);
    }

    c.bench_function("hashset close cardinal difference", |b| {
        b.iter(|| {
            let _: Vec<_> = sets1.difference(&sets2).collect();
        })
    });
}

fn btreeset_cardinal_close_benchmark(c: &mut Criterion) {
    let mut sets1 = BTreeSet::new();
    let mut sets2 = BTreeSet::new();
    for i in 0..500 {
        sets1.insert(i * 2);
        sets2.insert(i * 3);
    }

    c.bench_function("btreeset close cardinal difference", |b| {
        b.iter(|| {
            let _: Vec<_> = sets1.difference(&sets2).collect();
        })
    });
}

criterion_group!(
    benches,
    skipset_cardinal_close_benchmark,
    hashset_cardinal_close_benchmark,
    btreeset_cardinal_close_benchmark
);
criterion_main!(benches);
