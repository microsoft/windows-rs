use super::*;
use std::collections::*;

pub struct Features(BTreeMap<&'static str, BTreeSet<Row>>);

impl Features {
    pub fn get(&self) -> Vec<&'static str> {
        let mut compact = Vec::<&'static str>::new();
        for feature in self.0.keys() {
            for pos in 0..compact.len() {
                if feature.starts_with(unsafe { compact.get_unchecked(pos) }) {
                    compact.remove(pos);
                    break;
                }
            }
            compact.push(feature);
        }
        compact
    }

    pub(crate) fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub(crate) fn add_type(&mut self, def: &TypeDef) -> bool {
        self.0.entry(def.namespace()).or_default().insert(def.row.clone())
    }

    pub(crate) fn add_feature(&mut self, feature: &'static str) {
        self.0.entry(feature).or_default();
    }
}
