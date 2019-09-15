#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use skiplist::skiplist::SkipList;

fn skiplist_get_benchmark(c: &mut Criterion) {
    let mut sk = SkipList::new();
    for i in 0..10000 {
        sk.push_front(i);
    }

    c.bench_function("skiplist get 7777", |b| b.iter(|| sk.get(black_box(7777))));
}

criterion_group!(benches, skiplist_get_benchmark);
criterion_main!(benches);
