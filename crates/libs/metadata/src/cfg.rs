use super::*;
use std::collections::*;

#[derive(Clone)]
pub struct Cfg{
    types: BTreeMap<&'static str, BTreeSet<Row>>,
    arches: BTreeSet<&'static str>, 
}

impl Cfg {
    pub fn new() -> Self {
        Self{types: BTreeMap::new(), arches: BTreeSet::new()}
    }

    pub fn features(&self) -> Vec<&'static str> {
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

    pub fn arches(&self) -> &BTreeSet<&'static str> {
        &self.arches
    }

    pub(crate) fn add_type(&mut self, def: &TypeDef) -> bool {
        self.types.entry(def.namespace()).or_default().insert(def.row.clone())
    }

    pub fn add_feature(&mut self, feature: &'static str) {
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

    pub fn add_attributes(&mut self, attributes: impl Iterator<Item = Attribute>) {
        for attribute in attributes {
            match attribute.name() {
                "SupportedArchitectureAttribute" => {
                    if let Some((_, ConstantValue::I32(value))) = attribute.args().get(0) {
                        if value & 1 == 1 {
                            self.arches.insert("x86");
                        }
                        if value & 2 == 2 {
                            self.arches.insert("x86_64");
                        }
                        if value & 4 == 4 {
                            self.arches.insert("aarch64");
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
        self.arches.iter().for_each(|arch|{union.arches.insert(arch);});
        other.arches.iter().for_each(|arch|{union.arches.insert(arch);});
        union
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types() {
        let def = TypeReader::get().get_type(("Windows.Foundation", "IStringable")).unwrap();
        let namespaces = def.cfg().features();
        assert_eq!(namespaces.len(), 1);
        assert_eq!(namespaces[0], "Windows.Foundation");

        let def = TypeReader::get().get_type(("Windows.Devices.Display.Core", "DisplayPresentationRate")).unwrap();
        let namespaces = def.cfg().features();
        assert_eq!(namespaces.len(), 2);
        assert_eq!(namespaces[0], "Windows.Devices.Display.Core");
        assert_eq!(namespaces[1], "Windows.Foundation.Numerics");

        let def = TypeReader::get().get_type(("Windows.Graphics.DirectX.Direct3D11", "Direct3DSurfaceDescription")).unwrap();
        let namespaces = def.cfg().features();
        assert_eq!(namespaces.len(), 1);
        assert_eq!(namespaces[0], "Windows.Graphics.DirectX.Direct3D11");

        let def = TypeReader::get().get_type(("Windows.Win32.Security.Authorization.UI", "EFFPERM_RESULT_LIST")).unwrap();
        let namespaces = def.cfg().features();
        assert_eq!(namespaces.len(), 2);
        assert_eq!(namespaces[0], "Windows.Win32.Foundation");
        assert_eq!(namespaces[1], "Windows.Win32.Security.Authorization.UI");
    }

    #[test]
    fn type_defs() {
        let def = TypeReader::get().expect_type_def(("Windows.Foundation", "IStringable"));
        let namespaces = def.cfg().features();
        assert_eq!(namespaces.len(), 0);

        let def = TypeReader::get().expect_type_def(("Windows.Devices.Display.Core", "DisplayPresentationRate"));
        let namespaces = def.cfg().features();
        assert_eq!(namespaces.len(), 1);
        assert_eq!(namespaces[0], "Windows.Foundation.Numerics");

        let def = TypeReader::get().expect_type_def(("Windows.Graphics.DirectX.Direct3D11", "Direct3DSurfaceDescription"));
        let namespaces = def.cfg().features();
        assert_eq!(namespaces.len(), 0);

        let def = TypeReader::get().expect_type_def(("Windows.Win32.Security.Authorization.UI", "EFFPERM_RESULT_LIST"));
        let namespaces = def.cfg().features();
        assert_eq!(namespaces.len(), 1);
        assert_eq!(namespaces[0], "Windows.Win32.Foundation");
    }
}
