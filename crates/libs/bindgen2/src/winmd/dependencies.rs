use super::*;

type Map = HashMap<&'static str, HashSet<&'static str>>;

#[derive(Debug)]
pub struct Dependencies(Map);

impl Dependencies {
    pub fn new() -> Self {
        Self(Map::new())
    }

    fn insert(&mut self, namespace: &'static str, name: &'static str) -> bool {
        self.entry(namespace).or_default().insert(name)
    }
}

impl std::ops::Deref for Dependencies {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Dependencies {
    fn deref_mut(&mut self) -> &mut Map {
        &mut self.0
    }
}

impl Type {
    pub fn dependencies(&self, dependencies: &mut Dependencies, minimal: bool) {
        // First get the underlying type.
        let ty = match self {
            Self::PtrMut(ty, _) => ty,
            Self::PtrConst(ty, _) => ty,
            Self::ArrayFixed(ty, _) => ty,
            Self::Array(ty) => ty,
            Self::ArrayRef(ty) => ty,
            Self::ConstRef(ty) => ty,
            Self::PrimitiveOrEnum(_, ty) => ty,
            _ => self,
        };

        // Then insert its name into the dependencies map.
        match ty {
            Self::String => {
                dependencies.insert("", "String");
            }
            Self::Object => {
                dependencies.insert("", "Object");
            }
            Self::PSTR => {
                dependencies.insert("", "PSTR");
            }
            Self::PCSTR => {
                dependencies.insert("", "PCSTR");
            }
            Self::PWSTR => {
                dependencies.insert("", "PWSTR");
            }
            Self::PCWSTR => {
                dependencies.insert("", "PCWSTR");
            }
            Self::GUID => {
                dependencies.insert("", "GUID");
            }
            Self::HRESULT => {
                dependencies.insert("", "HRESULT");
            }
            Self::Item(item) => {
                let namespace = item.namespace();

                if namespace.is_empty() || dependencies.insert(namespace, item.name()) {
                    item.dependencies(dependencies, minimal);
                }
            }
            Self::Generic(item, generics) => {
                // Only chase dependencies if it was not previously added.
                if dependencies.insert(item.namespace(), item.name()) {
                    item.dependencies(dependencies, minimal);
                }

                generics
                    .iter()
                    .for_each(|ty| ty.dependencies(dependencies, minimal));
            }
            _ => {}
        }
    }
}

// TODO: and how to deal with limiting dependencies from undesired interface methods?
impl Signature {
    pub fn dependencies(&self, dependencies: &mut Dependencies, minimal: bool) {
        self.return_type.0.dependencies(dependencies, minimal);
        self.params
            .iter()
            .for_each(|(ty, _)| ty.dependencies(dependencies, minimal));
    }
}

impl Item {
    pub fn dependencies(&'static self, dependencies: &mut Dependencies, minimal: bool) {
        match self {
            Item::Class(item) => item.dependencies(dependencies, minimal),
            Item::Delegate(item) => item.dependencies(dependencies, minimal),
            Item::Interface(item) => item.dependencies(dependencies, minimal),
            Item::Struct(item) => item.dependencies(dependencies, minimal),
            Item::CppConst(item) => item.dependencies(dependencies, minimal),
            Item::CppDelegate(item) => item.dependencies(dependencies, minimal),
            Item::CppFn(item) => item.dependencies(dependencies, minimal),
            Item::CppInterface(item) => item.dependencies(dependencies, minimal),
            Item::CppStruct(item) => item.dependencies(dependencies, minimal),
            _ => {}
        }
    }
}

impl Class {
    pub fn dependencies(&self, _dependencies: &mut Dependencies, _minimal: bool) {}
}

impl Delegate {
    pub fn dependencies(&self, _dependencies: &mut Dependencies, _minimal: bool) {}
}

impl Interface {
    pub fn dependencies(&self, _dependencies: &mut Dependencies, _minimal: bool) {}
}

impl Struct {
    pub fn dependencies(&self, dependencies: &mut Dependencies, minimal: bool) {
        for field in self.def.fields() {
            field.ty(None).dependencies(dependencies, minimal);
        }
    }
}

impl CppConst {
    pub fn dependencies(&self, dependencies: &mut Dependencies, minimal: bool) {
        self.field.ty(None).dependencies(dependencies, minimal);
    }
}

impl CppDelegate {
    pub fn dependencies(&self, _dependencies: &mut Dependencies, _minimal: bool) {}
}

impl CppFn {
    pub fn dependencies(&self, dependencies: &mut Dependencies, minimal: bool) {
        self.method
            .signature(&[])
            .dependencies(dependencies, minimal);
    }
}

impl CppInterface {
    pub fn dependencies(&self, _dependencies: &mut Dependencies, _minimal: bool) {}
}

impl CppStruct {
    pub fn dependencies(&'static self, dependencies: &mut Dependencies, minimal: bool) {
        for field in self.def.fields() {
            field.ty(Some(self)).dependencies(dependencies, minimal);
        }
    }
}
