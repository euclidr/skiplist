
struct Node<K: Ord> {
    next: Option<Box<Node<K>>>,
    nexts: Vec<*mut Node<K>>,
    prev: *mut Node<K>,
    key: Option<K>,
}

struct SkipSet<K: Ord> {
    head: Box<Node<K>>,
    tail: *mut Node<K>,
    length: usize,
}

struct Iter<'_, K> {}

struct IntoIter<K> {}

struct Range<'_, K> {}

impl<K: Ord> SkipSet<K> {

    fn new() -> Self { unimplemented!() }

    fn add(&mut self, K) -> bool { unimplemented!() }

    fn remove<Q: ?Sized>(&mut self, q: &Q) -> Option<K>
    where K: Borrowed<Q>,
          Q: Ord,
    { unimplemented!() }

    fn contains<Q: ?Size>(&self, q: &Q) -> bool
    where K: Borrowed<Q>,
          Q: Ord,
    { unimplemented!() }

    fn cardinal(&self) -> usize { unimplemented!() }

    fn choose_one(&self) -> Option<&K> { unimplemented!() }

    fn minimum(&self) -> Option<&K> { unimplemented!() }

    fn maximum(&self) -> Option<&K> { unimplemented!() }

    fn remove_min(&mut self) -> Option<K> { unimplemented!() }

    fn remove_max(&mut self) -> Option<K> { unimplemented!() }

    fn iter(&self) -> Iter<'_, K> { unimplemented!() }

    fn into_iter(&mut self) -> IntoIter<K> { unimplemented!() }

    fn into_diff(self, other: Self) -> Self { unimplemented!() }

    fn into_inter(self, other: Self) -> Self { unimplemented!() }

    fn into_union(self, other: Self) -> Self { unimplemented!() }
}

impl<K: Ord + Copy> SkipSet<K> {
    fn diff(&self, other: &Self) -> Self { unimplemented!() }

    fn inter(&self, other: &Self) -> Self { unimplemented!() }

    fn union(&self, other: &Self) -> Self { unimplemented!() }
}