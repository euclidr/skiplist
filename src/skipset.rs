use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::RangeBounds;

use rand;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::level_generator::LevelGenerator;
use crate::ordered_skiplist::OrderedSkipList;
use crate::skiplist::{IntoIter, Iter, Node, Range};

pub struct SkipSet<V: Ord> {
    sk: OrderedSkipList<V>,
}

impl<V: Ord + Display> SkipSet<V> {
    pub fn new() -> Self {
        Self::with_level_generator(LevelGenerator::new())
    }

    pub fn with_level_generator(lg: LevelGenerator) -> Self {
        SkipSet {
            sk: OrderedSkipList::with_config(false, lg),
        }
    }

    /// Add a value, returns the old value if it exists.
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
        self.sk.insert(value)
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
        self.sk.get_first(q).map(|(_, v)| v)
    }

    /// Remove the value that equals q, returns the value if an element is removed
    /// returns None if the element do not exist.
    pub fn remove<Q: ?Sized>(&mut self, q: &Q) -> Option<V>
    where
        V: Borrow<Q>,
        Q: Ord,
    {
        self.sk.remove_first(q)
    }

    /// Check if the set contains the value.
    pub fn contains<Q: ?Sized>(&self, q: &Q) -> bool
    where
        V: Borrow<Q>,
        Q: Ord,
    {
        self.get(q).is_some()
    }

    /// Returns cardinal of the set.
    pub fn cardinal(&self) -> usize {
        self.sk.len()
    }

    /// Return a random value from the set, returns None if it's empty.
    pub fn choose_one(&self) -> Option<&V> {
        let cnt = self.cardinal();
        if cnt == 0 {
            return None;
        }

        let idx = StdRng::from_entropy().gen_range(0, cnt);
        self.sk.get(idx)
    }

    /// Returns the minimum value in the set
    pub fn min(&self) -> Option<&V> {
        self.sk.front()
    }

    /// Returns the maximum value in the set
    pub fn max(&self) -> Option<&V> {
        self.sk.back()
    }

    /// Remove the minimum value in the set
    pub fn remove_min(&mut self) -> Option<V> {
        self.sk.pop_front()
    }

    /// Remove the maximum value in the set
    pub fn remove_max(&mut self) -> Option<V> {
        self.sk.pop_back()
    }

    /// Returns graph that contains a range of elements of the skipset
    /// same as [`SkipList::explain`]: trait.SkipList.html#method.explain
    pub fn explain<R>(&self, range: R) -> Result<String, &'static str>
    where
        V: std::fmt::Display,
        R: RangeBounds<usize>,
    {
        self.sk.explain(range)
    }

    /// Returns an iterator for the set
    pub fn iter(&self) -> Iter<'_, V> {
        self.sk.iter()
    }

    /// Returns a range iterator for the set
    ///
    /// # Panics
    ///
    /// The method will panic if the start_bounds is less than the end_bounds
    ///
    pub fn range<'a, 'b, R, Q: 'b + ?Sized>(&'a self, range: R) -> Range<'a, V>
    where
        R: RangeBounds<&'b Q>,
        V: Borrow<Q>,
        Q: Ord,
    {
        self.sk.range(range)
    }

    /// Return a difference sets
    ///
    /// # Example
    ///
    /// ```
    /// use skiplist::skipset::SkipSet;
    ///
    /// let mut ss1 = SkipSet::new();
    /// let mut ss2 = SkipSet::new();
    /// for i in 0..10 {
    ///     ss1.add(i);
    ///     ss2.add(i+1);
    /// }
    ///
    /// let ss = ss1.into_symmetric_difference(ss2);
    /// assert_eq!(ss.cardinal(), 2);
    /// assert_eq!(ss.contains(&0), true);
    /// assert_eq!(ss.contains(&10), true);
    /// ```
    pub fn into_symmetric_difference(self, other: Self) -> Self {
        let lg = self.sk.sk.level_generator.clone();
        let mut iter_a = self.into_iter();
        let mut iter_b = other.into_iter();
        let mut a = iter_a.next();
        let mut b = iter_b.next();
        let mut arr = vec![];
        while a.is_some() || b.is_some() {
            if a.is_none() {
                arr.push(b.take().expect("b must not be none"));
                b = iter_b.next();
                continue;
            }

            if b.is_none() {
                arr.push(a.take().expect("a must not be none"));
                a = iter_a.next();
                continue;
            }

            match a.cmp(&b) {
                Ordering::Equal => {
                    a = iter_a.next();
                    b = iter_b.next();
                }
                Ordering::Less => {
                    arr.push(a.take().expect("a must not be none"));
                    a = iter_a.next();
                }
                Ordering::Greater => {
                    arr.push(b.take().expect("b must not be none"));
                    b = iter_b.next();
                }
            }
        }

        let mut result = Self::with_level_generator(lg);
        while !arr.is_empty() {
            result.add(arr.pop().expect("value must exist"));
        }

        result
    }

    fn into_difference(self, other: Self) -> Self {
        unimplemented!()
    }

    fn into_intersection(self, other: Self) -> Self {
        unimplemented!()
    }

    fn into_union(self, other: Self) -> Self {
        unimplemented!()
    }

    fn is_subset(&self, other: &Self) -> bool {
        unimplemented!()
    }

    fn is_superset(&self, other: &Self) -> bool {
        unimplemented!()
    }

}

impl<V: Ord> IntoIterator for SkipSet<V> {
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
        self.sk.into_iter()
    }
}

impl<K: Ord + Copy> SkipSet<K> {
    fn symmetric_difference(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn difference(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn intersection(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn union(&self, other: &Self) -> Self {
        unimplemented!()
    }
}
