use std::collections::HashMap;

// LRU implementation with double ended linked list, making all operation O(1).
// Key points:
// * Nodes are stored in a Vec. The Vec length never exceeds capacity because
//   evicted slots are reused.
// * We keep track on pointers from both sides of the list (Least and most recently used).

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Entry>,
    head: Option<usize>, // least recently used
    tail: Option<usize>, // most recently used
}

struct Entry {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

impl Entry {
    pub fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        assert!(capacity > 0);

        Self {
            capacity: capacity as usize,
            map: HashMap::new(),
            nodes: Vec::new(),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let Some(&idx) = self.map.get(&key) else {
            return -1;
        };

        let value = self.nodes[idx].value;
        self.move_to_tail(idx);

        value
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].value = value;
            self.move_to_tail(idx);
            return;
        }

        if self.map.len() == self.capacity {
            let idx = self.evict_lru();

            self.nodes[idx] = Entry::new(key, value);
            self.map.insert(key, idx);
            self.push_back(idx);
        } else {
            let idx = self.nodes.len();

            let entry = Entry::new(key, value);
            self.nodes.push(entry);
            self.map.insert(key, idx);
            self.push_back(idx);
        }
    }

    /// Make element most recent by detaching it and reattaching it again on top.
    fn move_to_tail(&mut self, idx: usize) {
        if self.tail.is_some_and(|i| i == idx) {
            return;
        }

        self.detach(idx);
        self.push_back(idx);
    }

    /// Remove the least recent item from `head`, detach it, and remove it from the map.
    fn evict_lru(&mut self) -> usize {
        let idx = self.head.expect("Cache is full, so LRU node must exist");
        let key = self.nodes[idx].key;

        self.detach(idx);
        self.map.remove(&key);

        idx
    }

    /// Detach a node from the recency list by linking its previous and next nodes.
    /// The node stays in `nodes`, but its prev/next links are cleared.
    fn detach(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;

        if let Some(prev_idx) = prev {
            self.nodes[prev_idx].next = next;
        } else {
            self.head = next;
        }

        if let Some(next_idx) = next {
            self.nodes[next_idx].prev = prev;
        } else {
            self.tail = prev;
        }

        self.nodes[idx].prev = None;
        self.nodes[idx].next = None;
    }

    /// Attach node as most recent touched node by setting `tail` field to it.
    fn push_back(&mut self, idx: usize) {
        self.nodes[idx].prev = self.tail;
        self.nodes[idx].next = None;

        if let Some(tail_idx) = self.tail {
            self.nodes[tail_idx].next = Some(idx);
        } else {
            self.head = Some(idx);
        }

        self.tail = Some(idx);
    }
}
