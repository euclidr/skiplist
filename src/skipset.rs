use std::borrow::Borrow;
use std::cmp::Ordering;
// use std::fmt::Display;
use std::ops::RangeBounds;

use rand;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::level_generator::LevelGenerator;
use crate::ordered_skiplist::OrderedSkipList;
use crate::skiplist::{IntoIter, Iter, Range};

pub struct SkipSet<V: Ord> {
    sk: OrderedSkipList<V>,
}

impl<V: Ord> SkipSet<V> {
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
    /// same as [`SkipList::explain`]: struct.SkipList.html#method.explain
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

    /// Returns a lazy iterator producing elements in the symmetric difference of `SkipSet`s.
    ///
    /// # Examples
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
    /// let arr: Vec<_> = ss1.symmetric_difference(&ss2).cloned().collect();
    /// assert_eq!(arr.len(), 2);
    /// assert_eq!(arr, vec![0, 10]);
    /// ```
    pub fn symmetric_difference<'a>(&'a self, rhs: &'a SkipSet<V>) -> SymmetricDifference<'a, V> {
        let mut lhs_iter = self.iter();
        let mut rhs_iter = rhs.iter();
        SymmetricDifference {
            lhs_value: lhs_iter.next(),
            rhs_value: rhs_iter.next(),
            lhs_iter: lhs_iter,
            rhs_iter: rhs_iter,
        }
    }

    /// Returns a lazy iterator producing elements in the difference of `SkipSet`s.
    ///
    /// # Examples
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
    /// let arr: Vec<_> = ss1.difference(&ss2).cloned().collect();
    /// assert_eq!(arr.len(), 1);
    /// assert_eq!(arr, vec![0]);
    /// ```
    pub fn difference<'a>(&'a self, rhs: &'a SkipSet<V>) -> Difference<'a, V> {
        // Use the search method if lhs's cardinal is much smaller than rhs's
        if self.cardinal() * rhs.levels() < rhs.cardinal() {
            return self.difference_search(rhs);
        }
        // else use the traverse method
        self.difference_traverse(rhs)
    }

    /// Returns a lazy iterator producing elements in the difference of `SkipSet`s.
    ///
    /// It's suitable when the cardinals of `self` and `rhs` is relatively close, but you should
    /// use [`SkipSet::difference`]: #method.difference most of the time, because it has been chosen
    /// for you.
    ///
    /// # Examples
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
    /// let arr: Vec<_> = ss1.difference_traverse(&ss2).cloned().collect();
    /// assert_eq!(arr.len(), 1);
    /// assert_eq!(arr, vec![0]);
    /// ```
    pub fn difference_traverse<'a>(&'a self, rhs: &'a SkipSet<V>) -> Difference<'a, V> {
        let mut lhs_iter = self.iter();
        let mut rhs_iter = rhs.iter();

        Difference::Traverse(DifferenceTraverse {
            lhs_value: lhs_iter.next(),
            rhs_value: rhs_iter.next(),
            lhs_iter: lhs_iter,
            rhs_iter: rhs_iter,
        })
    }

    /// Returns a lazy iterator producing elements in the difference of `SkipSet`s.
    ///
    /// It's suitable when the cardinals of `rhs` is much larger than `self`'s, but you should
    /// use [`SkipSet::difference`]: #method.difference most of the time, because it has been chosen
    /// for you.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::skipset::SkipSet;
    ///
    /// let mut ss1 = SkipSet::new();
    /// let mut ss2 = SkipSet::new();
    /// for i in 0..3 {
    ///     ss1.add(i);
    /// }
    /// for i in 3..30 {
    ///     ss2.add(i);
    /// }
    ///
    /// let arr: Vec<_> = ss1.difference_search(&ss2).cloned().collect();
    /// assert_eq!(arr.len(), 3);
    /// assert_eq!(arr, (0..3).collect::<Vec<i32>>());
    /// ```
    pub fn difference_search<'a>(&'a self, rhs: &'a SkipSet<V>) -> Difference<'a, V> {
        Difference::Search(DifferenceSearch {
            lhs_iter: self.iter(),
            rhs: rhs,
        })
    }

    /// Returns a lazy iterator producing elements in the intersection of `SkipSet`s.
    ///
    /// # Examples
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
    /// let arr: Vec<i32> = ss1.intersection(&ss2).cloned().collect();
    /// assert_eq!(arr.len(), 9);
    /// assert_eq!(arr, (1..10).collect::<Vec<i32>>());
    /// ```
    pub fn intersection<'a>(&'a self, rhs: &'a SkipSet<V>) -> Intersection<'a, V> {
        let (mut lhs, mut rhs) = (self, rhs);
        if rhs.cardinal() < lhs.cardinal() {
            std::mem::swap(&mut lhs, &mut rhs);
        }

        if lhs.cardinal() * rhs.levels() < rhs.cardinal() {
            return lhs.intersection_search(rhs);
        }

        lhs.intersection_traverse(rhs)
    }

    /// Returns a lazy iterator producing elements in the intersection of `SkipSet`s.
    ///
    /// It's suitable when the cardinals of `self` and `rhs` is relatively close, but you should
    /// use [`SkipSet::intersection`]: #method.intersection most of the time, because it has been chosen
    /// for you.
    ///
    ///
    /// # Examples
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
    /// let arr: Vec<i32> = ss1.intersection_traverse(&ss2).cloned().collect();
    /// assert_eq!(arr.len(), 9);
    /// assert_eq!(arr, (1..10).collect::<Vec<i32>>());
    /// ```
    pub fn intersection_traverse<'a>(&'a self, rhs: &'a SkipSet<V>) -> Intersection<'a, V> {
        let mut lhs_iter = self.iter();
        let mut rhs_iter = rhs.iter();
        Intersection::Traverse(IntersectionTraverse {
            lhs_value: lhs_iter.next(),
            rhs_value: rhs_iter.next(),
            lhs_iter: lhs_iter,
            rhs_iter: rhs_iter,
        })
    }

    /// Returns a lazy iterator producing elements in the intersection of `SkipSet`s.
    ///
    /// It's suitable when the cardinals of `rhs` is much larger than `self`'s, but you should
    /// use [`SkipSet::intersection`]: #method.intersection most of the time, because it has been chosen
    /// for you.
    ///
    /// # Examples
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
    /// let arr: Vec<i32> = ss1.intersection_search(&ss2).cloned().collect();
    /// assert_eq!(arr.len(), 9);
    /// assert_eq!(arr, (1..10).collect::<Vec<i32>>());
    /// ```
    pub fn intersection_search<'a>(&'a self, rhs: &'a SkipSet<V>) -> Intersection<'a, V> {
        Intersection::Search(IntersectionSearch {
            lhs_iter: self.iter(),
            rhs: rhs,
        })
    }

    /// Returns a lazy iterator producing elements in the union of `SkipSet`'s.
    ///
    /// # Examples
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
    /// let arr: Vec<i32> = ss1.union(&ss2).cloned().collect();
    /// assert_eq!(arr.len(), 11);
    /// assert_eq!(arr, (0..11).collect::<Vec<i32>>());
    /// ```
    pub fn union<'a>(&'a self, rhs: &'a SkipSet<V>) -> Union<'a, V> {
        let mut lhs_iter = self.iter();
        let mut rhs_iter = rhs.iter();
        Union {
            lhs_value: lhs_iter.next(),
            rhs_value: rhs_iter.next(),
            lhs_iter: lhs_iter,
            rhs_iter: rhs_iter,
        }
    }

    /// Check if `self` is subset of `rhs`
    ///
    /// # Examples
    /// ```
    /// use skiplist::skipset::SkipSet;
    ///
    /// let mut ss1 = SkipSet::new();
    /// let mut ss2 = SkipSet::new();
    /// for i in 0..10 {
    ///     ss1.add(i);
    /// }
    /// for i in 0..20 {
    ///     ss2.add(i);
    /// }
    /// assert!(ss1.is_subset(&ss2));
    /// ```
    pub fn is_subset(&self, rhs: &Self) -> bool {
        let mut cnt = 0;
        for _ in self.intersection(rhs) {
            cnt += 1;
        }
        cnt == self.cardinal()
    }

    /// Check if `self` is super of `rhs`
    ///
    /// # Examples
    /// ```
    /// use skiplist::skipset::SkipSet;
    ///
    /// let mut ss1 = SkipSet::new();
    /// let mut ss2 = SkipSet::new();
    /// for i in 0..10 {
    ///     ss1.add(i);
    /// }
    /// for i in 0..20 {
    ///     ss2.add(i);
    /// }
    /// assert!(ss2.is_superset(&ss1));
    /// ```
    pub fn is_superset(&self, rhs: &Self) -> bool {
        let mut cnt = 0;
        for _ in self.intersection(rhs) {
            cnt += 1;
        }
        cnt == rhs.cardinal()
    }

    fn levels(&self) -> usize {
        self.sk.sk.head.links.len()
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

/// A lazy iterator producing elements in the symmetric difference of `SkipSet`'s.
///
/// This `struct` is created by the [`symmetric_difference`] method on
/// [`SkipSet`]. See its documentation for more.
///
/// [`SkipSet`]: struct.SkipSet.html
/// [`symmetric_difference`]: struct.SkipSet.html#method.symmetric_difference
pub struct SymmetricDifference<'a, V: Ord> {
    lhs_iter: Iter<'a, V>,
    rhs_iter: Iter<'a, V>,
    lhs_value: Option<&'a V>,
    rhs_value: Option<&'a V>,
}

impl<'a, V: Ord> Iterator for SymmetricDifference<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.lhs_value.is_none() && self.rhs_value.is_none() {
                break;
            }

            if self.lhs_value.is_none() {
                let result = self.rhs_value.take();
                self.rhs_value = self.rhs_iter.next();
                return result;
            }

            if self.rhs_value.is_none() {
                let result = self.lhs_value.take();
                self.lhs_value = self.lhs_iter.next();
                return result;
            }

            match self.lhs_value.cmp(&self.rhs_value) {
                Ordering::Equal => {
                    self.lhs_value = self.lhs_iter.next();
                    self.rhs_value = self.rhs_iter.next();
                }
                Ordering::Greater => {
                    let result = self.rhs_value.take();
                    self.rhs_value = self.rhs_iter.next();
                    return result;
                }
                Ordering::Less => {
                    let result = self.lhs_value.take();
                    self.lhs_value = self.lhs_iter.next();
                    return result;
                }
            };
        }

        None
    }
}

#[doc(hidden)]
pub struct DifferenceTraverse<'a, V: Ord> {
    lhs_iter: Iter<'a, V>,
    rhs_iter: Iter<'a, V>,
    lhs_value: Option<&'a V>,
    rhs_value: Option<&'a V>,
}

impl<'a, V: Ord> Iterator for DifferenceTraverse<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.lhs_value.is_none() {
                break;
            }

            if self.rhs_value.is_none() {
                return std::mem::replace(&mut self.lhs_value, self.lhs_iter.next())
            }

            match self.lhs_value.cmp(&self.rhs_value) {
                Ordering::Equal => {
                    self.lhs_value = self.lhs_iter.next();
                    self.rhs_value = self.rhs_iter.next();
                }
                Ordering::Greater => {
                    self.rhs_value = self.rhs_iter.next();
                }
                Ordering::Less => {
                    return std::mem::replace(&mut self.lhs_value, self.lhs_iter.next());
                }
            }
        }
        None
    }
}

#[doc(hidden)]
pub struct DifferenceSearch<'a, V: Ord> {
    lhs_iter: Iter<'a, V>,
    rhs: &'a SkipSet<V>,
}

impl<'a, V: Ord> Iterator for DifferenceSearch<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.lhs_iter.next() {
                None => break,
                Some(value) => {
                    if !self.rhs.contains(value) {
                        return Some(value);
                    }
                }
            }
        }

        None
    }
}

/// A lazy iterator producing elements in the difference of `SkipSet`'s.
///
/// This `struct` is created by the [`difference`] method on
/// [`SkipSet`]. See its documentation for more.
///
/// [`SkipSet`]: struct.SkipSet.html
/// [`difference`]: struct.SkipSet.html#method.difference
pub enum Difference<'a, V: Ord> {
    Traverse(DifferenceTraverse<'a, V>),
    Search(DifferenceSearch<'a, V>),
}

impl<'a, V: Ord> Iterator for Difference<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Difference::Traverse(d) => d.next(),
            Difference::Search(d) => d.next(),
        }
    }
}

#[doc(hidden)]
pub struct IntersectionTraverse<'a, V: Ord> {
    lhs_iter: Iter<'a, V>,
    rhs_iter: Iter<'a, V>,
    lhs_value: Option<&'a V>,
    rhs_value: Option<&'a V>,
}

impl<'a, V: Ord> Iterator for IntersectionTraverse<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.lhs_value.is_none() || self.rhs_value.is_none() {
                break;
            }

            match self.lhs_value.cmp(&self.rhs_value) {
                Ordering::Equal => {
                    let result = self.lhs_value.take();
                    self.lhs_value = self.lhs_iter.next();
                    self.rhs_value = self.rhs_iter.next();
                    return result;
                }
                Ordering::Greater => {
                    self.rhs_value = self.rhs_iter.next();
                }
                Ordering::Less => {
                    self.lhs_value = self.lhs_iter.next();
                }
            }
        }
        None
    }
}

#[doc(hidden)]
pub struct IntersectionSearch<'a, V: Ord> {
    lhs_iter: Iter<'a, V>,
    rhs: &'a SkipSet<V>,
}

impl<'a, V: Ord> Iterator for IntersectionSearch<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.lhs_iter.next() {
                None => break,
                Some(value) => {
                    if self.rhs.contains(value) {
                        return Some(value);
                    }
                }
            }
        }

        None
    }
}

/// A lazy iterator producing elements in the intersection of `SkipSet`'s.
///
/// This `struct` is created by the [`intersection`] method on
/// [`SkipSet`]. See its documentation for more.
///
/// [`SkipSet`]: struct.SkipSet.html
/// [`intersection`]: struct.SkipSet.html#method.intersection
pub enum Intersection<'a, V: Ord> {
    Traverse(IntersectionTraverse<'a, V>),
    Search(IntersectionSearch<'a, V>),
}

impl<'a, V: Ord> Iterator for Intersection<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Intersection::Traverse(d) => d.next(),
            Intersection::Search(d) => d.next(),
        }
    }
}

/// A lazy iterator producing elements in the union of `SkipSet`'s.
///
/// This `struct` is created by the [`union`] method on
/// [`SkipSet`]. See its documentation for more.
///
/// [`SkipSet`]: struct.SkipSet.html
/// [`union`]: struct.SkipSet.html#method.union
pub struct Union<'a, V: Ord> {
    lhs_iter: Iter<'a, V>,
    rhs_iter: Iter<'a, V>,
    lhs_value: Option<&'a V>,
    rhs_value: Option<&'a V>,
}

impl<'a, V: Ord> Iterator for Union<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.lhs_value.is_none() && self.rhs_value.is_none() {
                break;
            }

            if self.lhs_value.is_none() {
                let result = self.rhs_value.take();
                self.rhs_value = self.rhs_iter.next();
                return result;
            }

            if self.rhs_value.is_none() {
                let result = self.lhs_value.take();
                self.lhs_value = self.lhs_iter.next();
                return result;
            }

            match self.lhs_value.cmp(&self.rhs_value) {
                Ordering::Equal => {
                    let result = self.lhs_value.take();
                    self.lhs_value = self.lhs_iter.next();
                    self.rhs_value = self.rhs_iter.next();
                    return result;
                }
                Ordering::Greater => {
                    let result = self.rhs_value.take();
                    self.rhs_value = self.rhs_iter.next();
                    return result;
                }
                Ordering::Less => {
                    let result = self.lhs_value.take();
                    self.lhs_value = self.lhs_iter.next();
                    return result;
                }
            }
        }
        None
    }
}
