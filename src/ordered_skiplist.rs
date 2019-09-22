use crate::skiplist::ReverseRange;
use crate::skiplist::Range;
use crate::skiplist::ReverseIter;
use crate::skiplist::Iter;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::ops::{Bound, RangeBounds};

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

    /// Removes duplicated items
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    ///
    /// let mut sk = OrderedSkipList::new_duplicatable();
    ///
    /// sk.insert(0);
    /// sk.insert(0);
    /// sk.insert(1);
    /// sk.insert(1);
    /// sk.insert(1);
    /// sk.insert(2);
    ///
    /// sk.dedup();
    ///
    /// let mut idx = 0;
    /// for value in sk.iter() {
    ///     assert_eq!(value, &idx);
    ///     idx += 1;
    /// }
    /// ```
    pub fn dedup(&mut self) {
        self.sk.dedup();
    }

    /// Returns length of the ordered_skiplist
    pub fn len(&self) -> usize {
        self.sk.len()
    }

    /// Returns an iterator for the ordered_skiplist
    /// 
    /// # Examples
    ///
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    ///
    /// let mut sk = OrderedSkipList::new();
    /// sk.insert(0);
    /// sk.insert(1);
    /// sk.insert(2);
    ///
    /// let mut i = 0;
    /// for value in sk.iter() {
    ///     assert_eq!(value, &i);
    ///     i += 1;
    /// }
    /// ```
    pub fn iter(&self) -> Iter<V> {
        self.sk.iter()
    }

    /// Returns a reverse iterator for the ordered_skiplist
    /// 
    /// # Examples
    ///
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    ///
    /// let mut sk = OrderedSkipList::new();
    /// sk.insert(0);
    /// sk.insert(1);
    /// sk.insert(2);
    ///
    /// let mut i = 2;
    /// for value in sk.reverse_iter() {
    ///     assert_eq!(value, &i);
    ///     i -= 1;
    /// }
    /// ```
    pub fn reverse_iter(&self) -> ReverseIter<V> {
        self.sk.reverse_iter()
    }


    /// Returns a range iterator for the ordered_skiplist
    /// 
    /// # Panics
    /// 
    /// Panics if start_bound is greater than end_bound
    /// 
    /// # Examples
    /// 
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    /// 
    /// let mut sk = OrderedSkipList::new();
    /// for i in 0..20 {
    ///     sk.insert(i);
    /// }
    /// 
    /// let mut i = 2;
    /// for value in sk.range(&2..&7) {
    ///     assert_eq!(value, &i);
    ///     i += 1;
    /// }
    /// assert_eq!(i, 7);
    /// ```
    pub fn range<'a, 'b, R, Q: 'b +  ?Sized>(&'a self, range: R) -> Range<'a, V>
    where
        R: RangeBounds<&'b Q>,
        V: Borrow<Q>,
        Q: Ord,
    {
        if self.len() == 0 {
            return self.sk.range(0..0);
        }

        let left = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(q) => self._index_not_less(q),
            Bound::Excluded(q) => self._index_not_less_or_equal(q),
        };

        let right = match range.end_bound() {
            Bound::Unbounded => self.len(),
            Bound::Included(q) => self._index_not_less_or_equal(q),
            Bound::Excluded(q) => self._index_not_less(q),
        };

        self.sk.range(left..right)
    }

    /// Returns a range iterator for the ordered_skiplist
    /// 
    /// # Panics
    /// 
    /// Panics if start_bound is greater than end_bound
    /// 
    /// # Examples
    /// 
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    /// 
    /// let mut sk = OrderedSkipList::new();
    /// for i in 0..20 {
    ///     sk.insert(i);
    /// }
    /// 
    /// let mut i = 6;
    /// for value in sk.reverse_range(&2..&7) {
    ///     assert_eq!(value, &i);
    ///     i -= 1;
    /// }
    /// assert_eq!(i, 1);
    /// ```
    pub fn reverse_range<'a, 'b, R, Q: 'b +  ?Sized>(&'a self, range: R) -> ReverseRange<'a, V>
    where
        R: RangeBounds<&'b Q>,
        V: Borrow<Q>,
        Q: Ord,
    {
        if self.len() == 0 {
            return self.sk.reverse_range(0..0);
        }

        let left = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(q) => self._index_not_less(q),
            Bound::Excluded(q) => self._index_not_less_or_equal(q),
        };

        let right = match range.end_bound() {
            Bound::Unbounded => self.len(),
            Bound::Included(q) => self._index_not_less_or_equal(q),
            Bound::Excluded(q) => self._index_not_less(q),
        };

        self.sk.reverse_range(left..right)
    }

    fn _index_not_less<Q: ?Sized>(&self, q: &Q) -> usize
    where
        V: Borrow<Q>,
        Q: Ord,
    {
        if self.len() == 0 {
            panic!("Can't get index from empty skiplist.");
        }
        let mut cur_index = 0;
        let mut cur_level = self.sk.head.links.len() - 1;
        let mut cur_ptr: *const _ = &*self.sk.head;

        loop {
            let next_ptr = unsafe{ (*cur_ptr).links[cur_level] };
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
                    cur_index += unsafe { (*cur_ptr).links_len[cur_level] };
                    cur_ptr = next_ptr;
                    continue;
                },
                _ => (),
            }
            if cur_level == 0 {
                break;
            }
            cur_level -= 1;
        }

        // cur_index is prev node index plus 1, so cur_index is index of item not less than q
        cur_index
    }

    fn _index_not_less_or_equal<Q: ?Sized>(&self, q: &Q) -> usize
    where
        V: Borrow<Q>,
        Q: Ord,
    {
        if self.len() == 0 {
            panic!("Can't get index from empty skiplist.");
        }
        let mut cur_index = 0;
        let mut cur_level = self.sk.head.links.len() - 1;
        let mut cur_ptr: *const _ = &*self.sk.head;

        loop {
            let next_ptr = unsafe{ (*cur_ptr).links[cur_level] };
            if next_ptr.is_null() {
                if cur_level == 0 {
                    break;
                }
                cur_level -= 1;
                continue;
            }

            let next_value = unsafe{ (*next_ptr).value.as_ref().unwrap() };
            match q.cmp(next_value.borrow()) {
                Ordering::Less => (),
                _ => {
                    cur_index += unsafe{ (*cur_ptr).links_len[cur_level] };
                    cur_ptr = next_ptr;
                    continue;
                },
            }
            if cur_level == 0 {
                break;
            }
            cur_level -= 1;
        }

        cur_index
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
    pub fn get_last<Q: ?Sized>(&self, q: &Q) -> Option<(usize, &V)>
    where V: Borrow<Q>,
          Q: Ord
    {
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
            match next_value.borrow().cmp(q) {
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
    pub fn get_first<Q: ?Sized>(&self, q: &Q) -> Option<(usize, &V)>
    where V: Borrow<Q>,
          Q: Ord
    {
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
            match next_value.borrow().cmp(q) {
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

    /// Remove item at the index
    /// 
    /// # Panics
    ///
    /// Panics if index is out of bounds
    /// 
    pub fn remove(&mut self, index: usize) -> V {
        self.sk.remove(index)
    }


    /// Remove the first item equals to q, returns the removed value
    pub fn remove_first<Q: ?Sized>(&mut self, q: &Q) -> Option<V>
    where
        V: Borrow<Q>,
        Q: Ord,
    {
        let first = self.get_first(q);
        match first {
            None => None,
            Some((index, _)) => {
                Some(self.remove(index))
            }
        }
    }

    /// Remove the last item equals to q, returns the removed value
    pub fn remove_last<Q: ?Sized>(&mut self, q: &Q) -> Option<V>
    where
        V: Borrow<Q>,
        Q: Ord,
    {
        let last = self.get_last(q);
        match last {
            None => None,
            Some((index, _)) => {
                Some(self.remove(index))
            }
        }
    }

    /// Remove the all items equals to q, returns number of items removed
    ///
    /// # Examples
    /// 
    /// ```
    /// use skiplist::ordered_skiplist::OrderedSkipList;
    /// 
    /// let mut sk = OrderedSkipList::new_duplicatable();
    /// sk.insert(0);
    /// sk.insert(0);
    /// sk.insert(0);
    /// 
    /// sk.remove_value(&0);
    /// assert_eq!(sk.len(), 0);
    /// ```
    pub fn remove_value<Q: ?Sized>(&mut self, q: &Q) -> usize
    where
        V: Borrow<Q>,
        Q: Ord,
    {
        let left = match self.get_first(q) {
            None => return 0,
            Some((index, _)) => index,
        };

        let right = match self.get_last(q) {
            None => unreachable!(),
            Some((index, _)) => index+1,
        };

        self.sk.remove_range(left..right)
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

    #[test]
    fn remove_value() {
        let mut sk = OrderedSkipList::new_duplicatable();
        sk.insert(5);
        sk.insert(5);
        for i in 0..10 {
            sk.insert(i);
        }

        assert_eq!(sk.len(), 12);
        assert_eq!(sk.get_first(&5), Some((5, &5)));
        assert_eq!(sk.get_last(&5), Some((7, &5)));

        sk.remove_value(&5);
        assert_eq!(sk.len(), 9);
        assert_eq!(sk.get_first(&5), None);
    }
}
