
struct Node<K: Ord, V> {
    next: Option<Box<Node<K, V>>>,
    nexts: Vec<*mut Node<K, V>>,
    prev: *mut Node<K, V>,
    key: Option<K>,
    value: Option<V>,
}

struct SkipMap<K: Ord, V> {
    head: Box<Node<K, V>>,
    tail: *mut Node<K, V>,
    length: usize,
}

struct Iter<'a, K, V> {}

struct IterMut<'a, K, V> {}

struct IntoIter<K, V> {}

struct Range<'a, K, V> {}

struct RangeMut<'a, K, V> {}

impl<K: Ord, V> SkipMap<K, V> {

    fn new() -> Self { unimplemented!() }

    fn insert(&mut self, key: K, value: V) -> Option<(K, V)> { unimplemented!() }

    fn remove<Q: ?Sized>(&mut self, q: &Q) -> Option<(K, V)>
    where K: Borrowed<Q>,
          Q: Ord
    { unimplemented!() }

    fn get<Q: ?Sized>(&self, q: &Q) -> Option<&V>
    where K: Borrowed<Q>,
          Q: Ord
    { unimplemented!() }

    fn get_mut<Q: ?Sized>(&mut self, q: &Q) -> Option<&mut V>
    where K: Borrowed<Q>,
          Q: Ord
    { unimplemented!() }

    fn get_kv<Q: ?Sized>(&self, q: &Q) -> Option<&K, &V>
    where K: Borrowed<Q>,
          Q: Ord
    { unimplemented!() }

    fn get_kv_mut<Q: ?Sized>(&mut self, q: &Q) -> Option<&K, &mut V>
    where K: Borrowed<Q>,
          Q: Ord
    { unimplemented!() }

    fn contains<Q: ?Sized>(&self, q: &Q) -> bool
    where K: Borrowed<Q>,
          Q: Ord
    { unimplemented!() }

    fn len(&self) -> usize { unimplemented!() }

    fn first(&self) -> Option<&K, &V> { unimplemented!() }

    fn first_mut(&mut self) -> Option<&K, &mut V> { unimplemented!() }

    fn remove_first(&mut self) -> Option<K, V> { unimplemented!() }

    fn last(&self) -> Option<&K, &V> { unimplemented!() }

    fn last_mut(&mut self) -> Option<&K, &mut V> { unimplemented!() }

    fn remove_last(&mut self) -> Option<K, V> { unimplemented!() }

    fn iter(&self) -> Iter<'_, K, V> { unimplemented!() }

    fn into_iter(self) -> IntoIter<K, V> { unimplemented!() }

    fn iter_mut(&mut self) -> IterMut<'_, K, V> { unimplemented!() }

    fn range(&self) -> Range<'_, K, V> { unimplemented!() }

    fn range_mut(&mut self) -> RangeMut<'_, K, V> { unimplemented!() }
}