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

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0)
    }

    pub fn included(&self, filter: &Filter) -> bool {
        for (namespace, names) in self.iter() {
            if namespace.is_empty() {
                continue;
            }

            for name in names {
                if !filter.includes_type_name(namespace, name) {
                    return false;
                }
            }
        }

        true
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
    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        match self {
            Self::PtrMut(ty, _) => ty.dependencies(dependencies, config),
            Self::PtrConst(ty, _) => ty.dependencies(dependencies, config),
            Self::ArrayFixed(ty, _) => ty.dependencies(dependencies, config),
            Self::Array(ty) => ty.dependencies(dependencies, config),
            Self::ArrayRef(ty) => ty.dependencies(dependencies, config),
            Self::ConstRef(ty) => ty.dependencies(dependencies, config),
            Self::PrimitiveOrEnum(_, ty) => ty.dependencies(dependencies, config),
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
                item.dependencies(dependencies, config);
            }
            Self::Generic(item, generics) => {
                item.dependencies(dependencies, config);

                generics
                    .iter()
                    .for_each(|ty| ty.dependencies(dependencies, config));
            }
            _ => {}
        }
    }
}

// TODO: and how to deal with limiting dependencies from undesired interface methods?
impl Signature {
    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        self.return_type.0.dependencies(dependencies, config);
        self.params
            .iter()
            .for_each(|(ty, _)| ty.dependencies(dependencies, config));
    }
}

// tODO: this should be the pivot for short circuiting, not Type
impl Item {
    pub fn dependencies(&'static self, dependencies: &mut Dependencies, config: &Config) {
        match self {
            Item::Class(item) => item.dependencies(dependencies, config),
            Item::Delegate(item) => item.dependencies(dependencies, config),
            Item::Enum(item) => item.dependencies(dependencies, config),
            Item::Interface(item) => item.dependencies(dependencies, config),
            Item::Struct(item) => item.dependencies(dependencies, config),
            Item::CppConst(item) => item.dependencies(dependencies, config),
            Item::CppDelegate(item) => item.dependencies(dependencies, config),
            Item::CppFn(item) => item.dependencies(dependencies, config),
            Item::CppInterface(item) => item.dependencies(dependencies, config),
            Item::CppStruct(item) => item.dependencies(dependencies, config),
            Item::CppEnum(item) => item.dependencies(dependencies, config),
        }
    }
}

impl Class {
    pub fn dependencies(&self, dependencies: &mut Dependencies, _config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            // TODO: add dependencies
        }
    }
}

impl Delegate {
    pub fn dependencies(&self, dependencies: &mut Dependencies, _config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            // TODO: add dependencies
        }
    }
}

impl Interface {
    pub fn dependencies(&self, dependencies: &mut Dependencies, _config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            // TODO: add dependencies
        }
    }
}

impl Struct {
    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            for field in self.def.fields() {
                field.ty(None).dependencies(dependencies, config);
            }
        }
    }
}

impl CppConst {
    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        if dependencies.insert(self.def.namespace(), self.field.name()) {
            self.field.ty(None).dependencies(dependencies, config);
        }
    }
}

impl CppDelegate {
    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            self.method()
                .signature(&[])
                .dependencies(dependencies, config);
        }
    }
}

impl CppFn {
    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        if dependencies.insert(self.def.namespace(), self.method.name()) {
            self.method
                .signature(&[])
                .dependencies(dependencies, config);
        }
    }
}

impl CppInterface {
    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        if !config.sys && dependencies.insert(self.def.namespace(), self.def.name()) {
            // TODO: add dependencies
        }
    }
}

impl CppStruct {
    pub fn dependencies(&'static self, dependencies: &mut Dependencies, config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            for field in self.def.fields() {
                field.ty(Some(self)).dependencies(dependencies, config);
            }
        }
    }
}

impl CppEnum {
    pub fn dependencies(&self, dependencies: &mut Dependencies, _config: &Config) {
        dependencies.insert(self.def.namespace(), self.def.name());
    }
}

impl Enum {
    pub fn dependencies(&self, dependencies: &mut Dependencies, _config: &Config) {
        dependencies.insert(self.def.namespace(), self.def.name());
    }
}
