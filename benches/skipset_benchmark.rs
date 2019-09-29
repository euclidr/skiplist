#[macro_use]
extern crate criterion;

use criterion::{Criterion, BenchmarkId};
use skiplist::skipset::SkipSet;
use std::collections::{BTreeSet, HashSet};

fn sets_difference(c: &mut Criterion) {

    let mut group = c.benchmark_group("sets_difference_close_cardinal_skipset_traverse");
    for size in [50, 500, 5000, 50000].iter() {
        if size > &1000 {
            group.sample_size(30);
        }
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut sets1 = SkipSet::new();
            let mut sets2 = SkipSet::new();
            for i in 0..size {
                sets1.add(i * 2);
                sets2.add(i * 3);
            }
            b.iter(|| {
                let _: Vec<_> = sets1.difference_traverse(&sets2).collect();
            })
        });
    }

    group.finish();

    let mut group = c.benchmark_group("sets_difference_close_cardinal_skipset_search");
    for size in [50, 500, 5000, 50000].iter() {
        if size > &1000 {
            group.sample_size(10);
        }
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut sets1 = SkipSet::new();
            let mut sets2 = SkipSet::new();
            for i in 0..size {
                sets1.add(i * 2);
                sets2.add(i * 3);
            }
            b.iter(|| {
                let _: Vec<_> = sets1.difference_search(&sets2).collect();
            })
        });
    }

    group.finish();

    let mut group = c.benchmark_group("sets_difference_close_cardinal_hashset");
    for size in [50, 500, 5000, 50000].iter() {
        if size > &1000 {
            group.sample_size(10);
        }
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut sets1 = HashSet::new();
            let mut sets2 = HashSet::new();
            for i in 0..size {
                sets1.insert(i * 2);
                sets2.insert(i * 3);
            }
            b.iter(|| {
                let _: Vec<_> = sets1.difference(&sets2).collect();
            })
        });
    }

    group.finish();

    let mut group = c.benchmark_group("sets_difference_close_cardinal_btreeset");
    for size in [50, 500, 5000, 50000].iter() {
        if size > &1000 {
            group.sample_size(30);
        }
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut sets1 = BTreeSet::new();
            let mut sets2 = BTreeSet::new();
            for i in 0..size {
                sets1.insert(i * 2);
                sets2.insert(i * 3);
            }
            b.iter(|| {
                let _: Vec<_> = sets1.difference(&sets2).collect();
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    sets_difference,
);
criterion_main!(benches);
