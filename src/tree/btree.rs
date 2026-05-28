use crate::tree::node::Node;

#[derive(Debug, Clone)]
pub struct BTree {
    pub root: Node,
    pub degree: usize,
}

impl BTree {
    pub fn new(degree: usize) -> Self {
        BTree {
            degree,
            root: Node::new(true),
        }
    }

    pub fn insert(&mut self, key: String, offset: u64) {
        if self.root.entries.len() == (2 * self.degree) - 1 {
            let mut new_root = Node::new(false);
            new_root.children.push(self.root.clone());
            new_root.split_child(0, self.degree);

            let mut idx = 0;
            if key > new_root.entries[0].0 {
                idx += 1;
            }
            new_root.children[idx].insert_non_full(key, offset, self.degree);
            self.root = new_root;
        } else {
            self.root.insert_non_full(key, offset, self.degree);
        }
    }

    pub fn get(&self, key: &str) -> Option<u64> {
        self.root.search(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.root.remove(key);
    }

    pub fn traverse(&self) -> Vec<(String, u64)> {
        let mut result = Vec::new();
        self.root.traverse(&mut result);
        result
    }
}