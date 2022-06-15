#[derive(Clone, Copy)]
pub enum ResolutionScope {
    None,
    Module(usize),
    ModuleRef(usize),
    AssemblyRef(usize),
    TypeRef(usize),
}

impl ResolutionScope {
    pub fn encode(&self) -> usize {
        match self {
            Self::Module(row) => ((row + 1) << 2),
            Self::ModuleRef(row) => ((row + 1) << 2) + 1,
            Self::AssemblyRef(row) => ((row + 1) << 2) + 2,
            Self::TypeRef(row) => ((row + 1) << 2) + 3,
            _ => unimplemented!(),
        }
    }
}

impl Default for ResolutionScope {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy)]
pub enum TypeDefOrRef {
    None,
    TypeDef(usize),
    TypeRef(usize),
    TypeSpec(usize),
}

impl TypeDefOrRef {
    pub fn encode(&self) -> usize {
        match self {
            Self::TypeDef(row) => ((row + 1) << 2),
            Self::TypeRef(row) => ((row + 1) << 2) + 1,
            Self::TypeSpec(row) => ((row + 1) << 2) + 2,
            _ => 0,
        }
    }
}

impl Default for TypeDefOrRef {
    fn default() -> Self {
        Self::None
    }
}
