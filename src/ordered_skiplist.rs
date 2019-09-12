
struct Node<K: Ord> {
    next: Option<Box<Node<K>>>,
    nexts: Vec<*mut Node<K>>,
    prev: *mut Node<K>,
    key: Option<K>,
}

struct OrderedSkiplist<K, F>
where F: Fn(&K, &K) -> Ordering
{
    head: Box<Node<K>>,
    tail: *mut Node<K>,
    length: usize,
    duplicatable: bool,
    compare: F,
}

impl<K: Ord, F> OrderedSkiplist<K, F> {
    fn new() -> Self { unimplemented!() }

    fn new_duplicatable() -> Self { unimplemented!() }
}

impl<K, F> OrderedSkiplist<K, F> {
    fn new_with_compare(f: F) -> Self { unimplemented!() }

    fn dedup(&mut self) { unimplemented!() }

    fn get(&self, index usize) -> Option<&K> { unimplemented!() }

    fn insert(&mut self, key: K) { unimplemented!() }

    fn remove<Q: ?Sized>(&mut self, q: &Q) -> bool
    where K: Borrowed<Q>,
    { unimplemented!() }

    fn remove_one<Q: ?Sized>(&mut self, q: &Q) -> bool
    where K: Borrowed<Q>,
    { unimplemented!() }

    fn length(&self) -> usize { unimplemented!() }
}