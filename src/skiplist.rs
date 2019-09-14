use crate::level_generator::LevelGenerator;
use std::fmt::Debug;

use std::marker::PhantomData;
use std::ops::RangeBounds;

struct Node<V> {
    value: Option<V>,
    next: Option<Box<Node<V>>>,
    prev: Option<*mut Node<V>>,
    links: Vec<*mut Node<V>>,
    links_len: Vec<usize>,
}

impl<V> Node<V> {
    fn new(value: Option<V>, levels: usize) -> Self {
        Self {
            value,
            next: None,
            prev: None,
            links: vec![std::ptr::null_mut(); levels],
            links_len: vec![0; levels],
        }
    }

    fn increase_level(&mut self) {
        self.links.push(std::ptr::null_mut());
        self.links_len.push(0);
    }
}

pub struct SkipList<V> {
    head: Box<Node<V>>,
    length: usize,
    level_generator: LevelGenerator,
}

pub struct Iter<'a, V> {
    phantom: PhantomData<&'a V>,
}

pub struct ReverseIter<'a, V> {
    phantom: PhantomData<&'a V>,
}

pub struct IterMut<'a, V> {
    phantom: PhantomData<&'a V>,
}

pub struct ReverseIterMut<'a, V> {
    phantom: PhantomData<&'a V>,
}

pub struct Range<'a, V> {
    phantom: PhantomData<&'a V>,
}

pub struct ReverseRange<'a, V> {
    phantom: PhantomData<&'a V>,
}

pub struct RangeMut<'a, V> {
    phantom: PhantomData<&'a V>,
}

pub struct ReverseRangeMut<'a, V> {
    phantom: PhantomData<&'a V>,
}

impl<V: Debug> SkipList<V> {
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
        if level >= self.head.links.len() {
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

        node.prev = Some(cur_ptr);

        match pre_node.next.take() {
            None => pre_node.next = Some(node),
            Some(mut next) => {
                next.prev = Some(node_ptr);
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
                next_node.prev = Some(cur_ptr);
                pre_node.next = Some(next_node);
            }
        };

        self.length -= 1;

        the_node.value.unwrap()
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

        let actual_index = index + 1;
        let mut cur_level = self.head.links.len() - 1;
        let mut cur_node: *const _ = &*self.head;
        let mut cur_index = 0;

        let the_node = unsafe {
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

            &*cur_node
        };

        Some(the_node.value.as_ref().unwrap())
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut V> {
        unimplemented!()
    }

    pub fn push_front(&mut self, value: V) {
        unimplemented!()
    }

    pub fn pop_front(&mut self) -> Option<V> {
        unimplemented!()
    }

    pub fn push_back(&mut self, value: V) {
        unimplemented!()
    }

    pub fn pop_back(&mut self) -> Option<V> {
        unimplemented!()
    }

    pub fn iter(&self) -> Iter<'_, V> {
        unimplemented!()
    }

    pub fn reverse_iter(&self) -> ReverseIter<'_, V> {
        unimplemented!()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, V> {
        unimplemented!()
    }
    pub fn reverse_iter_mut(&mut self) -> ReverseIterMut<'_, V> {
        unimplemented!()
    }

    pub fn range<R>(&self, range: R) -> Range<'_, V>
    where
        R: RangeBounds<usize>,
    {
        unimplemented!()
    }

    pub fn reverse_range<R>(&self, range: R) -> ReverseRange<'_, V>
    where
        R: RangeBounds<usize>,
    {
        unimplemented!()
    }

    pub fn range_mut<R>(&mut self, range: R) -> RangeMut<'_, V>
    where
        R: RangeBounds<usize>,
    {
        unimplemented!()
    }

    pub fn reverse_range_mut<R>(&mut self, range: R) -> ReverseRangeMut<'_, V>
    where
        R: RangeBounds<usize>,
    {
        unimplemented!()
    }

    pub fn dedup(&mut self) {
        unimplemented!()
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
}
