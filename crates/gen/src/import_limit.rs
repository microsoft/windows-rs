use std::collections::BTreeSet;

#[derive(Debug)]
pub enum ImportLimit {
    All,
    Some(BTreeSet<&'static str>),
}

impl ImportLimit {
    pub fn none() -> Self {
        Self::Some(BTreeSet::new())
    }
}
