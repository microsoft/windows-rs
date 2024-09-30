use super::*;

// TODO: for primitive dependencies (e.g. HSTRING, GUID, IUnknown) use the root namespace of ""
// so that standalone code generation can generate them as needed.

type Map = HashMap<&'static str, HashSet<&'static str>>;

pub struct Dependencies(Map);

impl Dependencies {
    pub fn new() -> Self {
        Self(Map::new())
    }

    pub fn insert(&mut self, namespace: &'static str, name: &'static str) -> bool {
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
    
    pub fn dependencies(&self, dependencies: &mut Dependencies) {
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
                // Only chase dependencies if it was not previously added.
                if dependencies.insert(item.namespace(), item.name()) {
                    item.dependencies(dependencies);
                }
            }
            Self::Generic(item, generics) => {
                // Only chase dependencies if it was not previously added.
                if dependencies.insert(item.namespace(), item.name()) {
                    item.dependencies(dependencies);
                }

                generics.iter().for_each(|ty| ty.dependencies(dependencies));
            }
            _ => {}
        }
    }
}

impl Signature {
    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        self.return_type.0.dependencies(dependencies);
        self.params.iter().for_each(|(ty, _)| ty.dependencies(dependencies));
    }
}

impl Item {
        // TODO: this really should be on the Iterm,TypeDef,MEthodDef, etc so that bindgen can use it for generating cfg attributes easily.
        pub fn dependencies(&'static self, dependencies: &mut Dependencies) {
            match self {
                winmd::Item::Class(item) => item.dependencies(dependencies),
                winmd::Item::Delegate(item) => item.dependencies(dependencies),
                winmd::Item::Enum(item) => item.dependencies(dependencies),
                winmd::Item::Interface(item) => item.dependencies(dependencies),
                winmd::Item::Struct(item) => item.dependencies(dependencies),
                winmd::Item::CppConst(item) => item.dependencies(dependencies),
                winmd::Item::CppDelegate(item) => item.dependencies(dependencies),
                winmd::Item::CppEnum(item) => item.dependencies(dependencies),
                winmd::Item::CppFn(item) => item.dependencies(dependencies),
                winmd::Item::CppInterface(item) => item.dependencies(dependencies),
                winmd::Item::CppStruct(item) => item.dependencies(dependencies),
            }
        }
}


impl Class {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {}
}

impl Delegate {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {}
}

impl Enum {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {}
}

impl Interface {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {}
}

impl Struct {
    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        for field in self.def.fields() {
            field.ty(None).dependencies(dependencies);
        }
    }
}

impl CppConst {
    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        self.field.ty(None).dependencies(dependencies);
    }
}

impl CppDelegate {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {}
}

impl CppEnum {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {}
}

impl CppFn {
    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        self.method.signature(&[]).dependencies(dependencies);
    }
}

impl CppInterface {
    pub fn dependencies(&self, _dependencies: &mut Dependencies) {}
}

impl CppStruct {
    pub fn dependencies(&'static self, dependencies: &mut Dependencies) {
        for field in self.def.fields() {
            field.ty(Some(self)).dependencies(dependencies);
        }
    }
}
