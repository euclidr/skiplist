use crate::level_generator::LevelGenerator;
// use std::fmt::Debug;

use std::marker::PhantomData;
use std::ops::{Bound, RangeBounds};

pub(crate) struct Node<V> {
    pub(crate) value: Option<V>,
    pub(crate) next: Option<Box<Node<V>>>,
    pub(crate) prev: *mut Node<V>,
    pub(crate) links: Vec<*mut Node<V>>,
    pub(crate) links_len: Vec<usize>,
}

impl<V> Default for Node<V> {
    fn default() -> Self {
        Self {
            value: None,
            next: None,
            prev: std::ptr::null_mut(),
            links: vec![],
            links_len: vec![],
        }
    }
}

impl<V> Node<V> {
    pub(crate) fn new(value: Option<V>, levels: usize) -> Self {
        Self {
            value,
            next: None,
            prev: std::ptr::null_mut(),
            links: vec![std::ptr::null_mut(); levels],
            links_len: vec![0; levels],
        }
    }

    pub(crate) fn increase_level(&mut self) {
        self.links.push(std::ptr::null_mut());
        self.links_len.push(0);
    }

    pub(crate) fn replace(&mut self, value: V) -> Option<V> {
        let result = self.value.take();
        self.value = Some(value);
        result
    }
}

pub struct SkipList<V> {
    pub(crate) head: Box<Node<V>>,
    pub(crate) length: usize,
    pub(crate) level_generator: LevelGenerator,
}

impl<V> SkipList<V> {
    /// Create a skiplist with default LevelGenerator that
    /// each level's propability is 1/2 of its previous level,
    /// and less than 32 levels
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut list: SkipList<i64> = SkipList::new();
    /// ```
    pub fn new() -> Self {
        Self::with_level_generator(LevelGenerator::new())
    }

    pub fn with_level_generator(lg: LevelGenerator) -> Self {
        SkipList {
            head: Box::new(Node::new(None, 0)),
            length: 0,
            level_generator: lg,
        }
    }

    /// Insert value at specific index
    ///
    /// # Panics
    ///
    /// Panics if index exceeds the length of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.insert(0, 2);
    /// sk.insert(1, 1);
    /// sk.insert(2, 0);
    /// ```
    pub fn insert(&mut self, index: usize, value: V) {
        if index > self.length {
            panic!("Index out of bounds.");
        }

        let level = self.level_generator.choose();
        let mut node = Box::new(Node::new(Some(value), level + 1));
        let node_ptr: *mut _ = &mut *node;
        while level >= self.head.links.len() {
            self.head.increase_level();
        }

        let mut cur_index = 0;
        let mut cur_level = self.head.links.len() - 1;
        let mut cur_ptr: *mut _ = &mut *self.head;
        // Outsider doesn't know the existence of head, but we should consider head
        // as the first node while inserting, so the index should be added by 1.
        let actual_index = index + 1;

        let pre_node = unsafe {
            loop {
                let next_ptr = (*cur_ptr).links[cur_level];
                if next_ptr.is_null() {
                    if cur_level <= level {
                        (*cur_ptr).links[cur_level] = node_ptr;
                        (*cur_ptr).links_len[cur_level] = actual_index - cur_index;
                    }
                    if cur_level == 0 {
                        break;
                    }
                    cur_level -= 1;
                    continue;
                }

                let next_index = cur_index + (*cur_ptr).links_len[cur_level];
                if next_index < actual_index {
                    // move forward in the same level
                    cur_ptr = (*cur_ptr).links[cur_level];
                    cur_index = next_index;
                    continue;
                }

                if cur_level <= level {
                    // insert link between current node and the next node
                    node.links_len[cur_level] = next_index + 1 - actual_index;
                    (*cur_ptr).links_len[cur_level] = actual_index - cur_index;
                    node.links[cur_level] = (*cur_ptr).links[cur_level];
                    (*cur_ptr).links[cur_level] = node_ptr;
                } else {
                    // increase link_len between current node and the next node
                    (*cur_ptr).links_len[cur_level] += 1;
                }

                if cur_level == 0 {
                    break;
                }

                cur_level -= 1;
            }

            &mut *cur_ptr
        };

        node.prev = cur_ptr;

        match pre_node.next.take() {
            None => pre_node.next = Some(node),
            Some(mut next) => {
                next.prev = node_ptr;
                node.next = Some(next);
                pre_node.next = Some(node);
            }
        };

        self.length += 1;
    }

    /// Remove item at specific index
    ///
    /// # Panics
    ///
    /// Panics if index exceeds the length of the skiplist
    ///
    /// # Example
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.insert(0, 0);
    /// sk.insert(1, 1);
    /// assert_eq!(sk.remove(0), 0);
    /// assert_eq!(sk.remove(0), 1);
    /// ```
    ///
    pub fn remove(&mut self, index: usize) -> V {
        if index > self.length {
            panic!("Index out of bounds.");
        }

        let actual_index = index + 1;
        let mut cur_index = 0;
        let mut cur_level = self.head.links.len() - 1;
        let mut cur_ptr: *mut _ = &mut *self.head;

        let pre_node = unsafe {
            loop {
                let next_ptr = (*cur_ptr).links[cur_level];
                if next_ptr.is_null() {
                    if cur_level == 0 {
                        unreachable!()
                    }
                    cur_level -= 1;
                    continue;
                }

                let next_index = cur_index + (*cur_ptr).links_len[cur_level];
                let next_links_len = (*next_ptr).links_len[cur_level];

                if next_index < actual_index {
                    // move forward in the same level
                    cur_ptr = (*cur_ptr).links[cur_level];
                    cur_index = next_index;
                    continue;
                }

                if next_index == actual_index {
                    // remove next link
                    (*cur_ptr).links[cur_level] = (*next_ptr).links[cur_level];
                    if next_links_len == 0 {
                        (*cur_ptr).links_len[cur_level] = 0;
                    } else {
                        (*cur_ptr).links_len[cur_level] += next_links_len - 1;
                    }
                }

                if next_index > actual_index {
                    // decrease link_len between current node and the next node
                    (*cur_ptr).links_len[cur_level] -= 1;
                }

                if cur_level == 0 {
                    break;
                }

                cur_level -= 1;
            }

            &mut *cur_ptr
        };

        let mut the_node = pre_node.next.take().unwrap();
        match the_node.next.take() {
            None => (),
            Some(mut next_node) => {
                next_node.prev = cur_ptr;
                pre_node.next = Some(next_node);
            }
        };

        self.length -= 1;

        the_node.value.unwrap()
    }

    /// Remove items in a range of indexes
    /// 
    /// # Panics
    ///
    /// Panics if start_bounds is greater than end_bounds
    ///
    /// # Example
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// for i in 0..20 {
    ///     sk.insert(i, i);
    /// }
    /// sk.remove_range(1..19);
    /// assert_eq!(sk.get(0), Some(&0));
    /// assert_eq!(sk.get(1), Some(&19));
    /// ```
    ///
    pub fn remove_range<R>(&mut self, range: R) -> usize
    where
        R: RangeBounds<usize>,
    {
        let (left, right) = self._normalize_range(range);
        if left == right {
            return 0;
        }

        let (left, right) = (left+1, right+1);

        let total_level = self.head.links.len();

        let mut prev_ptrs = vec![std::ptr::null_mut();total_level];
        let mut prev_indexes = vec![0;total_level];
        let mut cur_level = total_level - 1;
        let mut cur_ptr: *mut _ = &mut *self.head;
        let mut cur_index = 0;

        loop {
            prev_ptrs[cur_level] = cur_ptr;
            prev_indexes[cur_level] = cur_index;

            let next_ptr = unsafe{ (*cur_ptr).links[cur_level] };
            if next_ptr.is_null() {
                if cur_level == 0 {
                    break
                }
                cur_level -= 1;
                continue
            }

            let cur_len = unsafe{ (*cur_ptr).links_len[cur_level] };
            if cur_index + cur_len < left {
                cur_ptr = next_ptr;
                cur_index += cur_len;
                continue
            }

            if cur_level == 0 {
                break
            }
            cur_level -= 1;
        }

        for i in 0..total_level {
            let prev_node = unsafe{ &mut *prev_ptrs[i] };
            let mut next_index = prev_indexes[i] + prev_node.links_len[i];
            let mut next_ptr = prev_node.links[i];
            while !next_ptr.is_null() && next_index < right {
                let node = unsafe{ &mut *next_ptr };
                next_index += node.links_len[i];
                next_ptr = node.links[i];
            }

            if next_ptr.is_null() {
                prev_node.links[i] = std::ptr::null_mut();
                prev_node.links_len[i] = 0;
                continue
            }

            prev_node.links[i] = next_ptr;
            prev_node.links_len[i] = (next_index - prev_indexes[i]) - (right - left);
        }

        let prev_node = unsafe{ &mut *prev_ptrs[0] };
        let mut next_node = prev_node.next.take();
        for _ in left..right {
            next_node = next_node.and_then(|mut node| {
                node.next.take()
            });
        }

        prev_node.next = next_node;
        match prev_node.next.as_mut() {
            None => (),
            Some(next) => next.prev = prev_ptrs[0],
        }

        self.length -= right - left;
        right - left
    }

    /// Returns pointer to the given index
    ///
    /// Panics
    ///
    /// Panics if the index exceeds the length of the skiplist
    ///
    fn _get_ptr(&self, index: usize) -> *const Node<V> {
        if self.length <= index {
            panic!("Index out of bounds.");
        }

        let actual_index = index + 1;
        let mut cur_level = self.head.links.len() - 1;
        let mut cur_node: *const _ = &*self.head;
        let mut cur_index = 0;

        unsafe {
            while actual_index != cur_index {
                let next_index = cur_index + (*cur_node).links_len[cur_level];
                // if current node don't have next, cur_index equals next_index
                if next_index <= actual_index && cur_index != next_index {
                    cur_node = (*cur_node).links[cur_level];
                    cur_index = next_index;
                    continue;
                }
                cur_level -= 1;
            }
        };

        cur_node
    }

    /// Returns value at the given index, or `None` if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.insert(0, 0);
    /// sk.insert(1, 1);
    /// assert_eq!(sk.get(0), Some(&0));
    /// assert_eq!(sk.get(1), Some(&1));
    /// assert_eq!(sk.get(2), None);
    /// ```
    ///
    pub fn get(&self, index: usize) -> Option<&V> {
        if self.length <= index {
            return None;
        }

        let node = unsafe { &*self._get_ptr(index) };
        node.value.as_ref()
    }

    /// Returns mutable value at the given index, or `None` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.insert(0, 0);
    /// sk.insert(1, 1);
    /// *sk.get_mut(0).unwrap() = 10;
    /// assert_eq!(sk.get(0), Some(&10));
    /// ```
    ///
    pub fn get_mut(&mut self, index: usize) -> Option<&mut V> {
        if self.length <= index {
            return None;
        }

        let the_node = unsafe { &mut *(self._get_ptr(index) as *mut Node<V>) };
        Some(the_node.value.as_mut().unwrap())
    }

    /// Push a value at the front of skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_front(0);
    /// sk.push_front(1);
    /// sk.push_front(2);
    /// assert_eq!(sk.get(0), Some(&2));
    /// ```
    pub fn push_front(&mut self, value: V) {
        self.insert(0, value)
    }

    /// Remove the element at the front of skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_front(0);
    /// sk.push_front(1);
    /// sk.pop_front();
    /// assert_eq!(sk.get(0), Some(&0));
    /// ```
    pub fn pop_front(&mut self) -> Option<V> {
        if self.length == 0 {
            return None;
        }

        Some(self.remove(0))
    }

    /// Push a value at the end of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// assert_eq!(sk.get(1), Some(&1));
    /// ```
    pub fn push_back(&mut self, value: V) {
        self.insert(self.length, value)
    }

    /// Get the first value of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// assert_eq!(sk.front(), Some(&0));
    /// ```
    pub fn front(&self) -> Option<&V> {
        self.head.next.as_ref().and_then(|node| {
            node.value.as_ref()
        })
    }

    /// Get the last value of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// assert_eq!(sk.back(), Some(&1));
    /// ```
    pub fn back(&self) -> Option<&V> {
        if self.length == 0 {
            return None;
        }
        self.get(self.length-1)
    }

    /// Get the first mutable value of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// match sk.front_mut() {
    ///     Some(v) => *v = 10,
    ///     None => ()
    /// };
    /// assert_eq!(sk.front(), Some(&10));
    /// ```
    pub fn front_mut(&mut self) -> Option<&mut V> {
        self.head.next.as_mut().and_then(|node| {
            node.value.as_mut()
        })
    }

    /// Get the last mutable value of the skiplist
    /// 
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// match sk.back_mut() {
    ///     Some(v) => *v = 10,
    ///     None => ()
    /// };
    /// assert_eq!(sk.back(), Some(&10));
    /// ```
    pub fn back_mut(&mut self) -> Option<&mut V> {
        if self.length == 0 {
            return None;
        }
        self.get_mut(self.length-1)
    }

    /// Remove the element at the end of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// assert_eq!(sk.pop_back(), Some(1));
    /// assert_eq!(sk.pop_back(), Some(0));
    /// assert_eq!(sk.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<V> {
        if self.length == 0 {
            return None;
        }

        Some(self.remove(self.length - 1))
    }

    /// Returns an iterator of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// sk.push_back(2);
    ///
    /// let mut i = 0;
    /// for value in sk.iter() {
    ///     assert_eq!(value, &i);
    ///     i += 1;
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, V> {
        Iter {
            current: self.head.next.as_ref().map(|node| &**node),
        }
    }

    /// Returns an reverse iterator of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_front(0);
    /// sk.push_front(1);
    /// sk.push_front(2);
    ///
    /// let mut i = 0;
    /// for value in sk.reverse_iter() {
    ///     assert_eq!(value, &i);
    ///     i += 1;
    /// }
    /// ```
    pub fn reverse_iter(&self) -> ReverseIter<'_, V> {
        if self.length == 0 {
            return ReverseIter {
                current: std::ptr::null(),
                phantom: PhantomData,
            };
        }

        ReverseIter {
            current: self._get_ptr(self.length - 1),
            phantom: PhantomData,
        }
    }

    /// Returns a mutable iterator of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// sk.push_back(2);
    ///
    /// for value in sk.iter_mut() {
    ///     *value *= 2;
    /// }
    ///
    /// let mut i = 0;
    /// for value in sk.iter() {
    ///     assert_eq!(value, &i);
    ///     i += 2;
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, V> {
        IterMut {
            current: self.head.next.as_mut().map(|node| &mut **node),
        }
    }

    /// Returns a mutable reverse iterator of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// sk.push_back(2);
    ///
    /// let mut i = 0;
    /// for value in sk.reverse_iter_mut() {
    ///     *value += i;
    ///     i += 1;
    /// }
    ///
    /// for value in sk.iter() {
    ///     assert_eq!(value, &2);
    /// }
    /// ```
    pub fn reverse_iter_mut(&mut self) -> ReverseIterMut<'_, V> {
        if self.length == 0 {
            return ReverseIterMut {
                current: std::ptr::null_mut(),
                phantom: PhantomData,
            };
        }

        ReverseIterMut {
            current: self._get_ptr(self.length - 1) as *mut Node<V>,
            phantom: PhantomData,
        }
    }

    fn _normalize_range<R>(&self, range: R) -> (usize, usize)
    where
        R: RangeBounds<usize>,
    {
        let left = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1,
        };

        let mut right = match range.end_bound() {
            Bound::Unbounded => self.length,
            Bound::Included(i) => *i + 1,
            Bound::Excluded(i) => *i,
        };

        if right > self.length {
            right = self.length;
        }

        if left > right {
            panic!("Invalid range.")
        }

        (left, right)
    }

    /// Returns a range iterator of the skiplist
    ///
    /// # Panics
    ///
    /// Panics if start_bound is greater than end_bound
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// for i in 0..10 {
    ///     sk.push_back(i);
    /// }
    ///
    /// let mut idx = 2;
    /// for value in sk.range(2..7) {
    ///     assert_eq!(value, &idx);
    ///     idx += 1;
    /// }
    /// assert_eq!(idx, 7);
    /// ```
    pub fn range<R>(&self, range: R) -> Range<'_, V>
    where
        R: RangeBounds<usize>,
    {
        if self.length == 0 {
            return Range {
                current: None,
                left: 0,
            };
        }

        let (left, right) = self._normalize_range(range);
        if left == right {
            return Range {
                current: None,
                left: 0,
            };
        }

        let first = unsafe { &*self._get_ptr(left) };
        Range {
            current: Some(first),
            left: right - left,
        }
    }

    /// Returns a reverse range of the skiplist
    ///
    /// # Panics
    ///
    /// Panics if start_bound is greater than end_bound
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// for i in 0..10 {
    ///     sk.push_back(i);
    /// }
    ///
    /// let mut idx = 7;
    /// for value in sk.reverse_range(..7) {
    ///     idx -= 1;
    ///     assert_eq!(value, &idx);
    /// }
    /// ```
    pub fn reverse_range<R>(&self, range: R) -> ReverseRange<'_, V>
    where
        R: RangeBounds<usize>,
    {
        if self.length == 0 {
            return ReverseRange {
                current: std::ptr::null(),
                left: 0,
                phantom: PhantomData,
            };
        }

        let (left, right) = self._normalize_range(range);
        if left == right {
            return ReverseRange {
                current: std::ptr::null(),
                left: 0,
                phantom: PhantomData,
            };
        }

        // now right is surely greater than 0
        let last = self._get_ptr(right - 1);
        ReverseRange {
            current: last,
            left: right - left,
            phantom: PhantomData,
        }
    }

    /// Returns a range iterator of the skiplist, in which elements is mutable
    ///
    /// # Panics
    ///
    /// Panics if start_bound is greater than end_bound
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    ///
    /// for i in 0..10 {
    ///     sk.push_back(i);
    /// }
    ///
    /// for value in sk.range_mut(..) {
    ///     *value *= 2;
    /// }
    ///
    /// for value in sk.range(1..7) {
    ///     assert_eq!(*value % 2, 0);
    /// }
    /// ```
    pub fn range_mut<R>(&mut self, range: R) -> RangeMut<'_, V>
    where
        R: RangeBounds<usize>,
    {
        if self.length == 0 {
            return RangeMut {
                current: None,
                left: 0,
            };
        }

        let (left, right) = self._normalize_range(range);
        if left == right {
            return RangeMut {
                current: None,
                left: 0,
            };
        }

        let first = unsafe { &mut *(self._get_ptr(left) as *mut _) };
        RangeMut {
            current: Some(first),
            left: right - left,
        }
    }

    /// Returns a reverse range of the skiplist
    ///
    /// # Panics
    ///
    /// Panics if start_bound is greater than end_bound
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// for i in 0..10 {
    ///     sk.push_back(i);
    /// }
    ///
    /// let mut a = 0;
    /// for value in sk.reverse_range_mut(..8) {
    ///     *value += a;
    ///     a += 1;
    /// }
    ///
    /// for value in sk.range(..8) {
    ///     assert_eq!(value, &7);
    /// }
    /// ```
    pub fn reverse_range_mut<R>(&mut self, range: R) -> ReverseRangeMut<'_, V>
    where
        R: RangeBounds<usize>,
    {
        if self.length == 0 {
            return ReverseRangeMut {
                current: std::ptr::null_mut(),
                left: 0,
                phantom: PhantomData,
            };
        }

        let (left, right) = self._normalize_range(range);
        if left == right {
            return ReverseRangeMut {
                current: std::ptr::null_mut(),
                left: 0,
                phantom: PhantomData,
            };
        }

        // now right is surely greater than 0
        let last = self._get_ptr(right - 1) as *mut _;
        ReverseRangeMut {
            current: last,
            left: right - left,
            phantom: PhantomData,
        }
    }

    /// Remove consecutive duplicated items
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    ///
    /// sk.push_back(0);
    /// sk.push_back(0);
    /// sk.push_back(1);
    /// sk.push_back(1);
    /// sk.push_back(1);
    /// sk.push_back(2);
    ///
    /// sk.dedup();
    ///
    /// let mut idx = 0;
    /// for value in sk.iter() {
    ///     assert_eq!(value, &idx);
    ///     idx += 1;
    /// }
    /// ```
    pub fn dedup(&mut self)
    where
        V: Ord,
    {
        if self.length == 0 {
            return;
        }

        let mut index = 0;
        unsafe {
            let node = self.head.next.as_ref().unwrap();
            let mut current = &**node as *const Node<V>;
            while !current.is_null() {
                match (*current).next.as_ref() {
                    None => current = std::ptr::null(),
                    Some(next) => match next.value.cmp(&(*current).value) {
                        std::cmp::Ordering::Equal => {
                            self.remove(index + 1);
                        }
                        _ => {
                            current = &**next as *const Node<V>;
                            index += 1;
                        }
                    },
                }
            }
        };
    }

    /// Returns the length of the skiplist
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns graph that contains a range of elements of the skiplist
    /// 
    /// The graph is something like:
    /// ```ignore
    /// start: 1234, levels: 3, show_len: 4, total_len: 2000
    /// ----------------> [+2] -------------------->
    /// -------> [+1] --> [+2] -----------> [+4] -->
    /// [+0] --> [+1] --> [+2] --> [+3] --> [+4] -->
    /// values:
    /// [+0]: aaa
    /// [+1]: bbb
    /// [+2]: ccc
    /// [+3]: ddd
    /// ```
    pub fn explain<R>(&self, range: R) -> Result<String, &'static str>
    where
        V: std::fmt::Display,
        R: RangeBounds<usize>,
    {
        const ELEMENT_EMPTY_PART1_1: &str = "-----";
        const ELEMENT_EMPTY_PART1_2: &str = "------";
        const ELEMENT_PART2_1: &str = "--> ";
        const ELEMENT_PART2_2: &str = "----";
        const MAX_SPAN: usize = 20;

        let (left, right) = self._normalize_range(range);
        let span = right - left;
        if span > MAX_SPAN {
            return Err("Range span is too big, the span should be smaller than 20");
        }

        let levels = self.head.links.len();
        let mut result = format!("start: {}, levels: {}, show_len: {}, total_len: {}",
                             left, levels, right-left, self.len());
        let mut l_lines = vec![String::from("");levels];
        if span > 0 {
            let mut cur = unsafe{ &*self._get_ptr(left) };
            for idx in 0..span {
                let next = cur.next.as_ref();
                for level in 0..levels {
                    if cur.links.len() > level {
                        l_lines[level].push_str(&format!("[+{}] ", idx));
                    } else {
                        if idx < 10 {
                            l_lines[level].push_str(ELEMENT_EMPTY_PART1_1);
                        } else {
                            l_lines[level].push_str(ELEMENT_EMPTY_PART1_2);
                        }
                    }
                    match next {
                        None => l_lines[level].push_str(ELEMENT_PART2_1),
                        Some(node) => {
                            if node.links.len() > level {
                                l_lines[level].push_str(ELEMENT_PART2_1);
                            } else {
                                l_lines[level].push_str(ELEMENT_PART2_2);
                            }
                        }
                    }
                }
                match next {
                    None => (),
                    Some(next) => cur = &**next,
                }
            }
        }

        for level in (0..levels).rev() {
            result.push_str("\n");
            result.push_str(&l_lines[level]);
        }

        result.push_str("\nvalues:\n");

        if span > 0 {
            let mut cur = unsafe{ &*self._get_ptr(left) };
            for idx in 0..span {
                result.push_str(&format!("[+{}]: {}", idx, cur.value.as_ref().unwrap()));
                result.push_str("\n");
                match cur.next.as_ref() {
                    None => (),
                    Some(next) => cur = &**next,
                }
            }
        }

        Ok(result)
    }
}

impl<V: std::fmt::Debug> std::fmt::Debug for SkipList<V> {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", value)?;
        }
        write!(f, "]")
    }
}

impl<V: std::fmt::Display> std::fmt::Display for SkipList<V> {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}

impl<V> IntoIterator for SkipList<V> {
    type Item = V;
    type IntoIter = IntoIter<V>;

    /// Returns a moved iterator of the skiplist
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skiplist::SkipList;
    ///
    /// let mut sk = SkipList::new();
    /// for i in 0..10 {
    ///     sk.push_back(i);
    /// }
    /// let mut idx = 0;
    /// for value in sk.into_iter() {
    ///     assert_eq!(value, idx);
    ///     idx += 1;
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct Iter<'a, V> {
    current: Option<&'a Node<V>>,
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_ref().map(|node| &**node);
            node.value.as_ref().unwrap()
        })
    }
}

pub struct IntoIter<V>(SkipList<V>);

impl<V> Iterator for IntoIter<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct ReverseIter<'a, V> {
    current: *const Node<V>,
    phantom: PhantomData<&'a V>,
}

impl<'a, V> Iterator for ReverseIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        unsafe {
            let result = (*self.current).value.as_ref();
            let pre_ptr = (*self.current).prev as *const Node<V>;
            // The head node don't have a value, it can be a mark for iteration ending
            match (*pre_ptr).value.as_ref() {
                None => self.current = std::ptr::null(),
                Some(_) => self.current = pre_ptr,
            }
            result
        }
    }
}

pub struct IterMut<'a, V> {
    current: Option<&'a mut Node<V>>,
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_mut().map(|node| &mut **node);
            node.value.as_mut().unwrap()
        })
    }
}

pub struct ReverseIterMut<'a, V> {
    current: *mut Node<V>,
    phantom: PhantomData<&'a V>,
}

impl<'a, V> Iterator for ReverseIterMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        unsafe {
            let result = (*self.current).value.as_mut();
            let pre_ptr = (*self.current).prev;
            // The head node don't have a value, it can be a mark for iteration ending
            match (*pre_ptr).value.as_ref() {
                None => self.current = std::ptr::null_mut(),
                Some(_) => self.current = pre_ptr,
            }
            result
        }
    }
}

pub struct Range<'a, V> {
    current: Option<&'a Node<V>>,
    left: usize,
}

impl<'a, V> Iterator for Range<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().and_then(|node| {
            self.left -= 1;
            if self.left > 0 {
                self.current = node.next.as_ref().map(|node| &**node);
            }
            node.value.as_ref()
        })
    }
}

pub struct ReverseRange<'a, V> {
    current: *const Node<V>,
    left: usize,
    phantom: PhantomData<&'a V>,
}

impl<'a, V> Iterator for ReverseRange<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        self.left -= 1;

        unsafe {
            let result = (*self.current).value.as_ref();
            let pre_ptr = (*self.current).prev;
            match (*pre_ptr).value.as_ref() {
                None => self.current = std::ptr::null(),
                Some(_) => {
                    if self.left == 0 {
                        self.current = std::ptr::null();
                    } else {
                        self.current = pre_ptr;
                    }
                }
            }
            result
        }
    }
}

pub struct RangeMut<'a, V> {
    current: Option<&'a mut Node<V>>,
    left: usize,
}

impl<'a, V> Iterator for RangeMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().and_then(|node| {
            self.left -= 1;
            if self.left > 0 {
                self.current = node.next.as_mut().map(|node| &mut **node);
            }
            node.value.as_mut()
        })
    }
}

pub struct ReverseRangeMut<'a, V> {
    current: *mut Node<V>,
    left: usize,
    phantom: PhantomData<&'a V>,
}

impl<'a, V> Iterator for ReverseRangeMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        self.left -= 1;

        unsafe {
            let result = (*self.current).value.as_mut();
            let pre_ptr = (*self.current).prev;
            match (*pre_ptr).value.as_ref() {
                None => self.current = std::ptr::null_mut(),
                Some(_) => {
                    if self.left == 0 {
                        self.current = std::ptr::null_mut();
                    } else {
                        self.current = pre_ptr;
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn skiplist_insert() {
        let mut sk = SkipList::new();
        sk.insert(0, "0-0");
        sk.insert(1, "1-0");
        sk.insert(2, "2-0");
        sk.insert(3, "3-0");

        assert_eq!(sk.get(0), Some(&"0-0"));
        assert_eq!(sk.get(1), Some(&"1-0"));
        assert_eq!(sk.get(2), Some(&"2-0"));
        assert_eq!(sk.get(3), Some(&"3-0"));

        sk.insert(3, "3-1");
        assert_eq!(sk.get(0), Some(&"0-0"));
        assert_eq!(sk.get(1), Some(&"1-0"));
        assert_eq!(sk.get(2), Some(&"2-0"));
        assert_eq!(sk.get(3), Some(&"3-1"));
        assert_eq!(sk.get(4), Some(&"3-0"));

        sk.insert(0, "0-1");
        assert_eq!(sk.get(0), Some(&"0-1"));
        assert_eq!(sk.get(1), Some(&"0-0"));
        assert_eq!(sk.get(2), Some(&"1-0"));
        assert_eq!(sk.get(3), Some(&"2-0"));
        assert_eq!(sk.get(4), Some(&"3-1"));
        assert_eq!(sk.get(5), Some(&"3-0"));

        sk.insert(3, "3-2");
        assert_eq!(sk.get(0), Some(&"0-1"));
        assert_eq!(sk.get(1), Some(&"0-0"));
        assert_eq!(sk.get(2), Some(&"1-0"));
        assert_eq!(sk.get(3), Some(&"3-2"));
        assert_eq!(sk.get(4), Some(&"2-0"));
        assert_eq!(sk.get(5), Some(&"3-1"));
        assert_eq!(sk.get(6), Some(&"3-0"));
    }

    #[test]
    fn skiplist_remove() {
        let mut sk = SkipList::new();
        sk.insert(0, "0");
        sk.insert(1, "1");
        sk.insert(2, "2");
        sk.insert(3, "3");
        sk.insert(4, "4");
        sk.insert(5, "5");

        assert_eq!(sk.get(0), Some(&"0"));
        assert_eq!(sk.get(1), Some(&"1"));
        assert_eq!(sk.get(2), Some(&"2"));
        assert_eq!(sk.get(3), Some(&"3"));
        assert_eq!(sk.get(4), Some(&"4"));
        assert_eq!(sk.get(5), Some(&"5"));

        assert_eq!(sk.remove(4), "4");
        assert_eq!(sk.get(0), Some(&"0"));
        assert_eq!(sk.get(1), Some(&"1"));
        assert_eq!(sk.get(2), Some(&"2"));
        assert_eq!(sk.get(3), Some(&"3"));
        assert_eq!(sk.get(4), Some(&"5"));

        assert_eq!(sk.remove(1), "1");
        assert_eq!(sk.get(0), Some(&"0"));
        assert_eq!(sk.get(1), Some(&"2"));
        assert_eq!(sk.get(2), Some(&"3"));
        assert_eq!(sk.get(3), Some(&"5"));

        assert_eq!(sk.remove(3), "5");
        assert_eq!(sk.get(0), Some(&"0"));
        assert_eq!(sk.get(1), Some(&"2"));
        assert_eq!(sk.get(2), Some(&"3"));

        assert_eq!(sk.remove(0), "0");
        assert_eq!(sk.get(0), Some(&"2"));
        assert_eq!(sk.get(1), Some(&"3"));

        assert_eq!(sk.remove(0), "2");
        assert_eq!(sk.get(0), Some(&"3"));

        assert_eq!(sk.remove(0), "3");
        assert_eq!(sk.get(0), None);
    }

    #[test]
    fn nomalize_range() {
        let mut sk = SkipList::new();

        for i in 0..10 {
            sk.push_back(i);
        }

        let range = sk._normalize_range(1..4);
        assert_eq!(range, (1, 4));

        let range = sk._normalize_range(1..=4);
        assert_eq!(range, (1, 5));

        let range = sk._normalize_range(1..);
        assert_eq!(range, (1, 10));

        let range = sk._normalize_range(1..15);
        assert_eq!(range, (1, 10));

        let range = sk._normalize_range(..4);
        assert_eq!(range, (0, 4));

        let range = sk._normalize_range(4..4);
        assert_eq!(range, (4, 4));

        let range = sk._normalize_range(..);
        assert_eq!(range, (0, 10));

        let range = sk._normalize_range(10..15);
        assert_eq!(range, (10, 10));
    }

    #[test]
    fn remove_range() {
        let mut sk = SkipList::new();

        for i in 0..20 {
            sk.push_back(i);
        }

        let n = sk.remove_range(7..7);
        assert_eq!(n, 0);
        assert_eq!(sk.len(), 20);

        let n = sk.remove_range(7..8);
        assert_eq!(n, 1);
        assert_eq!(sk.len(), 19);
        assert_eq!(sk.get(7), Some(&8));

        let n = sk.remove_range(7..10);
        assert_eq!(n, 3);
        assert_eq!(sk.len(), 16);
        assert_eq!(sk.get(7), Some(&11));

        let n = sk.remove_range(7..);
        assert_eq!(n, 9);
        assert_eq!(sk.len(), 7);
        assert_eq!(sk.get(7), None);
        assert_eq!(sk.get(6), Some(&6));

        let n = sk.remove_range(..2);
        assert_eq!(n, 2);
        assert_eq!(sk.len(), 5);
        assert_eq!(sk.get(0), Some(&2));
    }

    #[test]
    fn explain() {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        use rand;

        let mut sk = SkipList::<i32>::new();
        let mut rng = StdRng::from_entropy();
        for i in 0..500 {
            sk.insert(rng.gen_range(0, i+1), rng.gen())
        }

        match sk.explain(0..10) {
            Ok(text) => print!("{}", text),
            Err(err) => print!("{}", err),
        };

        println!("");

        match sk.explain(485..) {
            Ok(text) => print!("{}", text),
            Err(err) => print!("{}", err),
        };

        println!("");

        match sk.explain(470..) {
            Ok(text) => print!("{}", text),
            Err(err) => print!("{}", err),
        };
    }
}
