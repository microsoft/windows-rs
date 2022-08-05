/// A `ResolutionScope` is an index into a certain table indicating the scope in which a TypeRef can be resolved.
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
            Self::Module(row) => (row + 1) << 2,
            Self::ModuleRef(row) => ((row + 1) << 2) + 1,
            Self::AssemblyRef(row) => ((row + 1) << 2) + 2,
            Self::TypeRef(row) => ((row + 1) << 2) + 3,
            _ => 0,
        }
    }
}

impl Default for ResolutionScope {
    fn default() -> Self {
        Self::None
    }
}

/// A `TypeDefOrRef` is an index into a certain table used to locate a type definition.
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
            Self::TypeDef(row) => (row + 1) << 2,
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

/// A `HasConstant` is an index into a certain table used to identify the parent of a row in the `Constant` table.
#[derive(Clone, Copy)]
pub enum HasConstant {
    None,
    Field(usize),
    Param(usize),
    Property(usize),
}

impl HasConstant {
    pub fn encode(&self) -> usize {
        match self {
            Self::Field(row) => (row + 1) << 2,
            Self::Param(row) => ((row + 1) << 2) + 1,
            Self::Property(row) => ((row + 1) << 2) + 2,
            _ => 0,
        }
    }
}

impl Default for HasConstant {
    fn default() -> Self {
        Self::None
    }
}
