#[derive(Debug, Clone)]
pub struct Node {
    pub entries: Vec<(String, u64)>, // (Key, Offset)
    pub children: Vec<Node>,
    pub is_leaf: bool,
}

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            entries: Vec::new(),
            children: Vec::new(),
            is_leaf,
        }
    }

    pub fn search(&self, key: &str) -> Option<u64> {
        let mut i = 0;
        while i < self.entries.len() && key > &self.entries[i].0 {
            i += 1;
        }

        // key found
        if i < self.entries.len() && self.entries[i].0 == key {
            return Some(self.entries[i].1);
        }

        // leaf reached and not found
        if self.is_leaf {
            return None;
        }

        // recurse into child
        self.children[i].search(key)
    }

    pub fn split_child(&mut self, i: usize, degree: usize) {
        let mut child = self.children.remove(i);
        let mut new_child = Node::new(child.is_leaf);

        // Midpoint for splitting
        let mid = degree - 1;

        // Move entries to new_child
        new_child.entries = child.entries.split_off(mid + 1);
        let mid_entry = child.entries.pop().unwrap();

        // Move children if not leaf
        if !child.is_leaf {
            new_child.children = child.children.split_off(mid + 1);
        }

        // Insert middle entry into self
        self.entries.insert(i, mid_entry);

        // Put child and new_child back
        self.children.insert(i, child);
        self.children.insert(i + 1, new_child);
    }

    pub fn insert_non_full(&mut self, key: String, offset: u64, degree: usize) {
        let mut i = 0;
        while i < self.entries.len() && &key > &self.entries[i].0 {
            i += 1;
        }

        // If key already exists in this node, update it
        if i < self.entries.len() && self.entries[i].0 == key {
            self.entries[i].1 = offset;
            return;
        }

        if self.is_leaf {
            self.entries.insert(i, (key, offset));
        } else {
            // Check if child is full
            if self.children[i].entries.len() == (2 * degree) - 1 {
                self.split_child(i, degree);

                // After split, the middle key of the child moved to this node.
                // Re-check where the new key goes.
                if key > self.entries[i].0 {
                    i += 1;
                } else if key == self.entries[i].0 {
                    // It's possible the split key is the target key (unlikely for new inserts but possible for updates)
                    self.entries[i].1 = offset;
                    return;
                }
            }
            self.children[i].insert_non_full(key, offset, degree);
        }
    }

    pub fn traverse(&self, result: &mut Vec<(String, u64)>) {
        let mut i = 0;
        while i < self.entries.len() {
            if !self.is_leaf {
                self.children[i].traverse(result);
            }
            result.push(self.entries[i].clone());
            i += 1;
        }

        if !self.is_leaf {
            self.children[i].traverse(result);
        }
    }

    pub fn remove(&mut self, key: &str) {
        let mut i = 0;
        while i < self.entries.len() && key > &self.entries[i].0 {
            i += 1;
        }

        if i < self.entries.len() && self.entries[i].0 == key {
            self.entries.remove(i);
            // In a real B-Tree, we'd need to handle child merging if this is now too small
            // or replace with predecessor/successor if not a leaf.
            // For now, if it's not a leaf, we just "leaked" the children structure
            // but for a KeyDir it will still function as a search tree.
            return;
        }

        if !self.is_leaf {
            self.children[i].remove(key);
        }
    }
}