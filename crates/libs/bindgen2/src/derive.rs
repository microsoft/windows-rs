use super::*;

struct Derive(BTreeSet<String>);

impl Derive {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn insert(name: &str) {
        self.0.insert(name.to_string())
    }
}