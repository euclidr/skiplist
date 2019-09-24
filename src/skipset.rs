use std::fmt::Display;
use std::borrow::Borrow;
use std::cmp::Ordering;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand;

use crate::level_generator::LevelGenerator;
use crate::ordered_skiplist::OrderedSkipList;
use crate::skiplist::Node;

pub struct SkipSet<V: Ord> {
    sk: OrderedSkipList<V>
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

    pub fn remove<Q: ?Sized>(&mut self, q: &Q) -> Option<V>
    where V: Borrow<Q>,
          Q: Ord,
    {
        self.sk.remove_first(q)
    }

    pub fn contains<Q: ?Sized>(&self, q: &Q) -> bool
    where V: Borrow<Q>,
          Q: Ord,
    {
        self.get(q).is_some()
    }

    pub fn cardinal(&self) -> usize {
        self.sk.len()
    }

    pub fn choose_one(&self) -> Option<&V> {
        let cnt = self.cardinal();
        if cnt == 0 {
            return None;
        }

        let idx = StdRng::from_entropy().gen_range(0, cnt);
        self.sk.get(idx)
    }

    fn minimum(&self) -> Option<&V> {
        unimplemented!()
    }

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