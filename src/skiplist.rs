
struct Node<K> {
    next: Option<Box<Node<K>>>,
    nexts: Vec<*mut Node<K>>,
    next_lens: Vec<usize>,
    prev: Option<*mut Node<K>>,
    key: Option<K>,
}

struct SkipList<K> {
    head: Node<K>,
    tail: *mut Node<K>,
    length: usize,
}

struct Iter<'a, K> {}

struct ReverseIter<'a, K> {}

struct IterMut<'a, K> {}

struct ReverseIterMut<'a, K> {}

struct Range<'a, K> {}

struct ReverseRange<'a, K> {}

struct RangeMut<'a, K> {}

struct ReverseRangeMut<'a, K> {}

impl<K> SkipList<K> {
    pub fn new() Self {
        unimplemented!()
    }

    pub fn insert(&mut self, index: usize, key: K) {
        unimplemented!()
    }

    pub fn remove(&mut self, index: usize) -> K {
        unimplemented!()
    }

    pub fn get(&self, index: usize) -> Option<&K> {
        unimplemented!()
    }

    pub fn get(&mut self, index: usize) -> Option<&mut K> {
        unimplemented!()
    }

    pub fn push_front(&mut self, K) {
        unimplemented!()
    }

    pub fn pop_front(&mut self) -> Option<K> {
        unimplemented!()
    }

    pub fn push_back(&mut self, K) {
        unimplemented!()
    }

    pub fn pop_back(&mut self) -> Option<K> {
        unimplemented!()
    }

    pub fn iter(&self) -> Iter<'_, K> {
        unimplemented!()
    }

    pub fn reverse_iter(&self) -> ReverseIter<'_, K> {
        unimplemented!()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K> {
        unimplemented!()
    }
    
    pub fn reverse_iter_mut(&mut self) -> ReverseIterMut<'_, K> {
        unimplemented!()
    }

    pub fn range<R>(&self, range R) -> Range<'_, K>
    where R: RangeBounds<usize> {
        unimplemented!()
    }

    pub fn reverse_range<R>(&self, range R) -> ReverseRange<'_, K>
    where R: RangeBounds<usize> {
        unimplemented!()
    }

    pub fn range_mut<R>(&mut self, range R) -> RangeMut<'_, K>
    where R: RangeBounds<usize> {
        unimplemented!()
    }

    pub fn reverse_range_mut<R>(&mut self, range R) -> ReverseRangeMut<'_, K>
    where R: RangeBounds<usize> {
        unimplemented!()
    }

    pub fn dedup(&mut self) {
        unimplemented!()
    }
}