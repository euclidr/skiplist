use skiplist::skipset::SkipSet;
use std::time::Instant;
use std::collections::BTreeSet;

// Results 2019-09-29, MacBook Pro (Retina, 13-inch, Early 2015)
// add in order ellapse: 24.623865618s
// add in reverse order ellapse: 2.728420022s
// btreeset add in order ellapse: 2.437528442s
// btreeset add in reverse order ellapse: 1.383035462s
fn main() {
    let start = Instant::now();
    let mut ss = SkipSet::new();
    for i in 0..500000 {
        ss.add(i);
    }
    let duration = start.elapsed();
    println!("add in order ellapse: {:?}", duration);

    let start = Instant::now();
    let mut ss = SkipSet::new();
    for i in (0..500000).rev() {
        ss.add(i);
    }
    let duration = start.elapsed();
    println!("add in reverse order ellapse: {:?}", duration);

    let start = Instant::now();
    let mut ss = BTreeSet::new();
    for i in 0..500000 {
        ss.insert(i);
    }
    let duration = start.elapsed();
    println!("btreeset add in order ellapse: {:?}", duration);

    let start = Instant::now();
    let mut ss = BTreeSet::new();
    for i in (0..500000).rev() {
        ss.insert(i);
    }
    let duration = start.elapsed();
    println!("btreeset add in reverse order ellapse: {:?}", duration);

    let mut a = 0;
    for i in ss.iter() {
        a = *i;
    }

    println!("done {}", a)
}