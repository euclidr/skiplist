
use std::borrow::Borrow;
use std::cmp::Ordering;

use crate::level_generator::LevelGenerator;

struct Node<K> {
    next: Option<Box<Node<K>>>,
    links: Vec<*mut Node<K>>,
    links_len: Vec<i32>,
    prev: *mut Node<K>,
    key: Option<K>,
}

impl<K> Default for Node<K> {
    fn default() -> Self {
        Node {
            next: None,
            links: vec![],
            links_len: vec![]
            prev: std::ptr::null_mut(),
            key: None,
        }
    }

    fn increase_level(&mut self) {
        self.links.push(std::ptr::null_mut());
        self.links_len.push(0);
    }
}

struct OrderedSkiplist<K>
{
    head: Box<Node<K>>,
    tail: *mut Node<K>,
    length: usize,
    duplicatable: bool,
    compare: Box<dyn Fn(&K, &K) -> Ordering>,
    level_generator: LevelGenerator,
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
            level_generator: LevelGenerator::new(), // TODO make it configable
        }
    }

    fn dedup(&mut self) { unimplemented!() }

    fn get(&self, index: usize) -> Option<&K> { unimplemented!() }

    fn insert(&mut self, key: K) -> Option<K> {
        let level = self.level_generator.choose();
        let total_level = self.head.links.len();

        let mut node = Box::new(Node::new(Some(key), level + 1));
        let node_ptr: *mut _ = &mut *node;
        let mut prev_ptrs = vec![std::ptr::null_mut():total_level];

        while level >= total_level {
            self.head.increase_level();
        }

        let cur_ptr = &mut *self.head;
        let mut cur_level = total_level - 1;
        let has_equal = false;
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

            match self.compare(unsafe{ &(*next_ptr).value }, &key) {
                Ordering::Less => {
                    cur_ptr = next_ptr;
                    continue;
                },
                Ordering::Equal => {
                    has_equal = true;
                    break;
                }
                _ => ()
            }

            if cur_level == 0 {
                break;
            }
            cur_level -= 1;
        }

        if has_equal {
            return Some(key);
        }

        for i in (0..total_level).rev() {
            let prev = unsafe{ &mut *prev_ptrs[i] };
            if prev.links[i].is_null() || i > level {
                continue
            }

            if prev.links[i].is_null() {
                prev.links[i] = node_ptr;
                prev.links_len[i] = 1;
                continue;
            }

            if i > level {
                prev.links_len[i] += 1;
                continue;
            }

            node.links[i] = prev.links[i];
            node.links_len[i] = prev.links_len[i];
            prev.links[i] = node_ptr
            prev.links_len[i] = 1;
        }

        unimplemented!()

    }

    fn remove<Q: ?Sized>(&mut self, q: &Q) -> bool
    where K: Borrow<Q>,
    { unimplemented!() }

    fn remove_one<Q: ?Sized>(&mut self, q: &Q) -> bool
    where K: Borrow<Q>,
    { unimplemented!() }

    fn length(&self) -> usize { unimplemented!() }
}