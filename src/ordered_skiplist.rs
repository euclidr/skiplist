use std::borrow::Borrow;
use std::cmp::Ordering;

use crate::level_generator::LevelGenerator;
use crate::skiplist::{SkipList, Node};


pub struct OrderedSkipList<V: Ord> {
    sk: SkipList<V>,
    duplicatable: bool,
}


impl<V: Ord + std::fmt::Debug> OrderedSkipList<V> {
    pub fn new() -> Self {
        Self::with_config(false, LevelGenerator::new())
    }

    pub fn new_duplicatable() -> Self {
        Self::with_config(true, LevelGenerator::new())
    }

    pub fn with_config(dup: bool, lg: LevelGenerator) -> Self {
        Self {
            sk: SkipList::with_level_generator(lg),
            duplicatable: dup,
        }
    }

    fn dedup(&mut self) {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        self.sk.len()
    }

    /// Returns value at the given index, or `None` if the index is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    ///
    /// let mut sk = OrderedSkipList::new();
    /// sk.insert(0);
    /// sk.insert(1);
    /// assert_eq!(sk.get(1), Some(&1));
    /// ```
    pub fn get(&self, index: usize) -> Option<&V> {
        self.sk.get(index)
    }

    /// Get the last element equals to q
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    ///
    /// let mut sk = OrderedSkipList::new_duplicatable();
    /// sk.insert(1);
    /// sk.insert(0);
    /// sk.insert(1);
    /// sk.insert(2);
    ///
    /// assert_eq!(sk.get_last(&2), Some((3, &2)));
    /// assert_eq!(sk.get_last(&1), Some((2, &1)));
    /// assert_eq!(sk.get_last(&3), None);
    /// ```
    pub fn get_last(&self, q: &V) -> Option<(usize, &V)> {
        if self.len() == 0 {
            return None;
        }
        let sk = &self.sk;
        let mut cur_level = sk.head.links.len() - 1;
        let mut cur_index = 0;
        let mut cur_ptr: *const _ = &*sk.head;
        let mut has_equal = false;

        loop {
            let cur_node = unsafe { &*cur_ptr };
            if cur_node.links[cur_level].is_null() {
                if cur_level == 0 {
                    break;
                }
                cur_level -= 1;
                continue;
            }

            let next_value = unsafe { (*cur_node.links[cur_level]).value.as_ref().unwrap() };
            match next_value.cmp(q) {
                Ordering::Less => {
                    cur_ptr = cur_node.links[cur_level];
                    cur_index += cur_node.links_len[cur_level];
                    continue;
                }
                Ordering::Equal => {
                    has_equal = true;
                    cur_ptr = cur_node.links[cur_level];
                    cur_index += cur_node.links_len[cur_level];
                    continue;
                }
                Ordering::Greater => (),
            }

            if cur_level == 0 {
                break;
            }
            cur_level -= 1;
        }

        if !has_equal {
            return None;
        }

        let v = unsafe { (*cur_ptr).value.as_ref() };

        // cur_index is node index added by 1
        v.map(|v| (cur_index - 1, v))
    }

    /// Get the first element equals to q
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    ///
    /// let mut sk = OrderedSkipList::new_duplicatable();
    /// sk.insert(1);
    /// sk.insert(0);
    /// sk.insert(1);
    ///
    /// assert_eq!(sk.get_first(&1), Some((1, &1)));
    /// assert_eq!(sk.get_first(&2), None);
    /// ```
    pub fn get_first(&self, q: &V) -> Option<(usize, &V)> {
        if self.len() == 0 {
            return None;
        }

        let sk = &self.sk;
        let mut cur_level = sk.head.links.len() - 1;
        let mut cur_index = 0;
        let mut cur_ptr: *const _ = &*sk.head;
        let mut has_equal = false;

        loop {
            let cur_node = unsafe { &*cur_ptr };
            if cur_node.links[cur_level].is_null() {
                if cur_level == 0 {
                    break;
                }
                cur_level -= 1;
                continue;
            }

            let next_value = unsafe { (*cur_node.links[cur_level]).value.as_ref().unwrap() };
            match next_value.cmp(q) {
                Ordering::Less => {
                    cur_ptr = cur_node.links[cur_level];
                    cur_index += cur_node.links_len[cur_level];
                    continue;
                }
                Ordering::Equal => {
                    has_equal = true;
                }
                Ordering::Greater => (),
            }

            if cur_level == 0 {
                break;
            }
            cur_level -= 1;
        }

        if !has_equal {
            return None;
        }

        let v = unsafe {
            (*cur_ptr)
                .next
                .as_ref()
                .and_then(|next| next.value.as_ref())
        };

        // cur_index is prev index added by 1
        // so the node index which is prev index plus one equals to cur_index
        v.map(|v| (cur_index, v))
    }

    /// Insert value, if the ordered skiplist if duplicatable return None after inserted
    /// if it's not duplicatable and the value is duplicated return the old one
    ///
    /// # Example
    ///
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    ///
    /// let mut sk = OrderedSkipList::new();
    /// sk.insert(0);
    /// sk.insert(1);
    /// sk.insert(0);
    /// assert_eq!(sk.get(1), Some(&1));
    /// ```
    pub fn insert(&mut self, value: V) -> Option<V> {
        // create a node
        let sk = &mut self.sk;
        let level = sk.level_generator.choose();
        let mut node = Box::new(Node::new(None, level + 1));
        let node_ptr: *mut _ = &mut *node;

        while level >= sk.head.links.len() {
            sk.head.increase_level();
        }

        // get previous nodes for later use
        let total_level = sk.head.links.len();
        let mut prev_ptrs = vec![std::ptr::null_mut(); total_level];
        let mut prev_indexs = vec![0; total_level];
        let mut cur_ptr: *mut _ = &mut *sk.head;
        let mut cur_index = 0;
        let mut cur_level = total_level - 1;
        let mut has_equal = false;
        loop {
            prev_ptrs[cur_level] = cur_ptr;
            prev_indexs[cur_level] = cur_index;

            let next_ptr = unsafe { (*cur_ptr).links[cur_level] };
            let cur_len = unsafe { (*cur_ptr).links_len[cur_level] };
            if next_ptr.is_null() {
                if cur_level == 0 {
                    break;
                }
                cur_level -= 1;
                continue;
            }

            let next_value = unsafe { (*next_ptr).value.as_ref().unwrap() };
            match next_value.cmp(&value) {
                Ordering::Less => {
                    cur_ptr = next_ptr;
                    cur_index += cur_len;
                    continue;
                }
                Ordering::Equal => {
                    has_equal = true;
                }
                _ => (),
            }

            if cur_level == 0 {
                break;
            }
            cur_level -= 1;
        }

        // if duplicated and not duplicatable, replace the old one
        if has_equal && !self.duplicatable {
            return unsafe {
                (*cur_ptr)
                    .next
                    .as_mut()
                    .and_then(|node| node.replace(value))
            };
        }

        node.value = Some(value);
        let node_index = prev_indexs[0] + 1;

        // modify links
        for i in 0..total_level {
            let prev = unsafe { &mut *prev_ptrs[i] };
            if prev.links[i].is_null() && i > level {
                continue;
            }

            if prev.links[i].is_null() {
                prev.links[i] = node_ptr;
                prev.links_len[i] = node_index - prev_indexs[i];
                continue;
            }

            if i > level {
                prev.links_len[i] += 1;
                continue;
            }

            node.links[i] = prev.links[i];
            node.links_len[i] = prev_indexs[i] + prev.links_len[i] + 1 - node_index;
            prev.links[i] = node_ptr;
            prev.links_len[i] = node_index - prev_indexs[i];
        }

        // insert the node
        let prev = unsafe { &mut *cur_ptr };
        node.next = prev.next.take().map(|mut next| {
            next.prev = node_ptr;
            next
        });
        node.prev = cur_ptr;
        prev.next = Some(node);

        self.sk.length += 1;

        None
    }

    fn remove_one<Q: ?Sized>(&mut self, q: &Q) -> bool
    where
        V: Borrow<Q>,
    {
        unimplemented!()
    }

    fn length(&self) -> usize {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ordered_skiplist_no_dup_insert() {
        let mut sk = OrderedSkipList::new();
        sk.insert(0);
        sk.insert(2);
        sk.insert(1);
        sk.insert(4);
        sk.insert(3);

        assert_eq!(sk.len(), 5);
        assert_eq!(sk.get(0), Some(&0));
        assert_eq!(sk.get(1), Some(&1));
        assert_eq!(sk.get(2), Some(&2));
        assert_eq!(sk.get(3), Some(&3));
        assert_eq!(sk.get(4), Some(&4));

        sk.insert(0);
        sk.insert(1);
        sk.insert(2);
        sk.insert(3);
        sk.insert(4);

        assert_eq!(sk.len(), 5);
        assert_eq!(sk.get(0), Some(&0));
        assert_eq!(sk.get(1), Some(&1));
        assert_eq!(sk.get(2), Some(&2));
        assert_eq!(sk.get(3), Some(&3));
        assert_eq!(sk.get(4), Some(&4));
    }

    #[test]
    fn ordered_skiplist_dup_insert() {
        let mut sk = OrderedSkipList::new_duplicatable();
        sk.insert(0);
        sk.insert(2);
        sk.insert(1);
        sk.insert(4);
        sk.insert(3);

        assert_eq!(sk.len(), 5);
        assert_eq!(sk.get(0), Some(&0));
        assert_eq!(sk.get(1), Some(&1));
        assert_eq!(sk.get(2), Some(&2));
        assert_eq!(sk.get(3), Some(&3));
        assert_eq!(sk.get(4), Some(&4));

        sk.insert(0);
        sk.insert(1);
        sk.insert(2);
        sk.insert(3);
        sk.insert(4);

        assert_eq!(sk.len(), 10);
        assert_eq!(sk.get(0), Some(&0));
        assert_eq!(sk.get(1), Some(&0));
        assert_eq!(sk.get(2), Some(&1));
        assert_eq!(sk.get(3), Some(&1));
        assert_eq!(sk.get(4), Some(&2));
    }
}
