
use std::borrow::Borrow;
use std::cmp::Ordering;

struct Node<K> {
    next: Option<Box<Node<K>>>,
    nexts: Vec<*mut Node<K>>,
    prev: *mut Node<K>,
    key: Option<K>,
}

impl<K> Default for Node<K> {
    fn default() -> Self {
        Node {
            next: None,
            nexts: vec![],
            prev: std::ptr::null_mut(),
            key: None,
        }
    }
}

struct OrderedSkiplist<K>
{
    head: Box<Node<K>>,
    tail: *mut Node<K>,
    length: usize,
    duplicatable: bool,
    compare: Box<dyn Fn(&K, &K) -> Ordering>,
}

impl<K: Ord> OrderedSkiplist<K>
{
    fn new() -> Self {
        Self::with_config(|a, b| a.cmp(&b), false)
    }

    fn new_duplicatable() -> Self {
        Self::with_config(|a, b| a.cmp(&b), true)
    }
}

impl<K> OrderedSkiplist<K>
{
    fn with_config(f: impl Fn(&K, &K) -> Ordering + 'static, dup: bool) -> Self {
        Self {
            head: Box::new(Node::default()),
            tail: std::ptr::null_mut(),
            length: 0,
            duplicatable: dup,
            compare: Box::new(f),
        }
    }

    fn dedup(&mut self) { unimplemented!() }

    fn get(&self, index: usize) -> Option<&K> { unimplemented!() }

    fn insert(&mut self, key: K) { unimplemented!() }

    fn remove<Q: ?Sized>(&mut self, q: &Q) -> bool
    where K: Borrow<Q>,
    { unimplemented!() }

    fn remove_one<Q: ?Sized>(&mut self, q: &Q) -> bool
    where K: Borrow<Q>,
    { unimplemented!() }

    fn length(&self) -> usize { unimplemented!() }
}