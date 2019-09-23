use std::fmt::{Debug, Display};
use std::borrow::Borrow;
use std::cmp::Ordering;
use crate::level_generator::LevelGenerator;

struct Node<V: Ord> {
    next: Option<Box<Node<V>>>,
    links: Vec<*mut Node<V>>,
    prev: *mut Node<V>,
    value: Option<V>,
}

impl<V: Ord> Default for Node<V> {
    fn default() -> Self {
        Self {
            next: None,
            links: vec![],
            prev: std::ptr::null_mut(),
            value: None,
        }
    }
}

impl<V: Ord> Node<V> {
    fn new(value: Option<V>, level: usize) -> Self {
        Self {
            next: None,
            links: vec![std::ptr::null_mut(); level],
            prev: std::ptr::null_mut(),
            value: value,
        }
    }

    fn increase_level(&mut self) {
        self.links.push(std::ptr::null_mut());
    }
}

pub struct SkipSet<V: Ord> {
    head: Box<Node<V>>,
    length: usize,
    level_generator: LevelGenerator,
}

// struct Iter<'a, K> {}

// struct IntoIter<K> {}

// struct Range<'a, K> {}

impl<V: Ord + Display> SkipSet<V> {

    pub fn new() -> Self {
        Self::with_level_generator(LevelGenerator::new())
    }

    pub fn with_level_generator(lg: LevelGenerator) -> Self {
        SkipSet {
            head: Box::new(Node::new(None, 0)),
            length: 0,
            level_generator: lg,
        }
    }

    /// Add a value, returns the new value if it exists.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use skiplist::skipset::SkipSet;
    /// 
    /// let mut ss = SkipSet::new();
    /// ss.add(1);
    /// ss.add(2);
    /// ss.add(0);
    /// assert_eq!(ss.cardinal(), 3);
    /// assert_eq!(ss.get(&1), Some(&1));
    /// assert_eq!(ss.get(&3), None);
    /// ```
    /// 
    pub fn add(&mut self, value: V) -> Option<V> {
        let level = self.level_generator.choose();
        let mut node = Box::new(Node::new(None, level+1));
        let node_ptr: *mut _ = &mut *node;

        while self.head.links.len() <= level {
            self.head.increase_level();
        }

        let total_level = self.head.links.len();
        let mut cur_level = total_level - 1;
        let mut cur_ptr: *mut _ = &mut *self.head;
        let mut prev_ptrs = vec![std::ptr::null_mut();total_level];
        loop {
            prev_ptrs[cur_level] = cur_ptr;
            let next_ptr = unsafe{ (*cur_ptr).links[cur_level] };
            if next_ptr.is_null() {
                if cur_level == 0 {
                    break;
                }
                cur_level -= 1;
                continue;
            }

            let next_value = unsafe{ (*next_ptr).value.as_ref().unwrap() };
            match value.cmp(next_value) {
                Ordering::Equal => return Some(value),
                Ordering::Greater => {
                    cur_ptr = next_ptr;
                    continue;
                },
                Ordering::Less => (),
            }

            if cur_level == 0 {
                break;
            }
            cur_level -= 1;
            continue;
        }

        for i in 0..level+1 {
            let prev = unsafe{ &mut *prev_ptrs[i] };
            node.links[i] = prev.links[i];
            prev.links[i] = node_ptr;
        }

        node.prev = prev_ptrs[0];
        node.value = Some(value);
        node.next = unsafe { (&mut *prev_ptrs[0]).next.take() };
        unsafe{ (&mut *prev_ptrs[0]).next = Some(node) };
        self.length += 1;
        None
    }

    /// Get the value that match q
    /// 
    /// # Examples
    /// 
    /// ```
    /// use skiplist::skipset::SkipSet;
    /// 
    /// let mut ss = SkipSet::new();
    /// ss.add(1);
    /// ss.add(2);
    /// ss.add(0);
    /// assert_eq!(ss.cardinal(), 3);
    /// assert_eq!(ss.get(&1), Some(&1));
    /// assert_eq!(ss.get(&3), None);
    /// ```
    /// 
    pub fn get<Q: ?Sized>(&self, q: &Q) -> Option<&V>
    where
        V: Borrow<Q>,
        Q: Ord,
    {
        if self.length == 0 {
            return None;
        }

        let mut cur_ptr: *const _ = &*self.head;
        let mut cur_level = self.head.links.len() - 1;
        loop {
            let next_ptr = unsafe{ (*cur_ptr).links[cur_level] as *const Node<V>};
            if next_ptr.is_null() {
                if cur_level == 0 {
                    break;
                }
                cur_level -= 1;
                continue;
            }

            let next_value = unsafe{ (*next_ptr).value.as_ref().unwrap() };
            match q.cmp(next_value.borrow()) {
                Ordering::Greater => {
                    cur_ptr = next_ptr;
                    continue;
                },
                Ordering::Equal => return Some(next_value),
                Ordering::Less => (),
            }
            if cur_level == 0 {
                break;
            }
            cur_level -= 1;
            continue;
        }

        None
    }

    fn remove<Q: ?Sized>(&mut self, q: &Q) -> Option<V>
    where V: Borrow<Q>,
          Q: Ord,
    { unimplemented!() }

    pub fn contains<Q: ?Sized>(&self, q: &Q) -> bool
    where V: Borrow<Q>,
          Q: Ord,
    {
        self.get(q).is_some()
    }

    pub fn cardinal(&self) -> usize {
        self.length
    }

    fn choose_one(&self) -> Option<&V> { unimplemented!() }

    fn minimum(&self) -> Option<&V> { unimplemented!() }

    fn maximum(&self) -> Option<&V> { unimplemented!() }

    fn remove_min(&mut self) -> Option<V> { unimplemented!() }

    fn remove_max(&mut self) -> Option<V> { unimplemented!() }

    // fn iter(&self) -> Iter<'_, K> { unimplemented!() }

    // fn into_iter(&mut self) -> IntoIter<K> { unimplemented!() }

    fn into_diff(self, other: Self) -> Self { unimplemented!() }

    fn into_inter(self, other: Self) -> Self { unimplemented!() }

    fn into_union(self, other: Self) -> Self { unimplemented!() }
}

impl<K: Ord + Copy> SkipSet<K> {
    fn diff(&self, other: &Self) -> Self { unimplemented!() }

    fn inter(&self, other: &Self) -> Self { unimplemented!() }

    fn union(&self, other: &Self) -> Self { unimplemented!() }
}