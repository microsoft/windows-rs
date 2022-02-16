use super::*;
use std::collections::*;

pub struct Features{
    types: BTreeMap<&'static str, BTreeSet<Row>>,
    arch: BTreeSet<&'static str>, 
}

impl Features {
    pub fn get(&self) -> Vec<&'static str> {
        let mut compact = Vec::<&'static str>::new();
        for feature in self.types.keys() {
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
        Self{types: BTreeMap::new(), arch: BTreeSet::new()}
    }

    pub(crate) fn add_type(&mut self, def: &TypeDef) -> bool {
        self.types.entry(def.namespace()).or_default().insert(def.row.clone())
    }

    pub(crate) fn add_feature(&mut self, feature: &'static str) {
        self.types.entry(feature).or_default();
    }

    pub(crate) fn remove_feature(&mut self, feature: &'static str) {
        let mut remove = Vec::<&'static str>::new();
        for existing in self.types.keys() {
            if feature.starts_with(existing) {
                remove.push(existing);
            }
        }
        for remove in remove {
            self.types.remove(remove);
        }
    }

    pub(crate) fn add_attributes(&mut self, attributes: impl Iterator<Item = Attribute>) {
        for attribute in attributes {
            match attribute.name() {
                "SupportedArchitectureAttribute" => {
                    if let Some((_, ConstantValue::I32(value))) = attribute.args().get(0) {
                        if value & 1 == 1 {
                            self.arch.insert("x86");
                        }
                        if value & 2 == 2 {
                            self.arch.insert("x86_64");
                        }
                        if value & 4 == 4 {
                            self.arch.insert("aarch64");
                        }
                    }
                }
                "DeprecatedAttribute" => {
                    self.add_feature("deprecated");
                }
                _ => {}
            }
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut union = Self::new();
        self.types.keys().for_each(|feature|union.add_feature(feature));
        other.types.keys().for_each(|feature|union.add_feature(feature));
        self.arch.iter().for_each(|arch|{union.arch.insert(arch);});
        other.arch.iter().for_each(|arch|{union.arch.insert(arch);});
        union
    }
}
