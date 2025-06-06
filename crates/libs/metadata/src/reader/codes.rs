use super::*;

pub trait Decode<'a> {
    fn decode(index: &'a TypeIndex, file: usize, code: usize) -> Self;
}

macro_rules! code {
    ($name:ident($size:literal) $(($table:ident, $code:literal))+) => {
        #[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
        pub enum $name<'a> {
            $($table($table<'a>),)*
        }
        impl<'a> Decode<'a> for $name<'a> {
            fn decode(index: &'a TypeIndex, file: usize, code: usize) -> Self {
                let (kind, row) = (code & ((1 << $size) - 1), (code >> $size) - 1);
                match kind {
                    $($code => Self::$table($table(Row::new(index, file, row))),)*
                    rest => panic!("{rest:?}"),
                }
            }
        }
        impl $name<'_> {
            #[allow(dead_code)]
            pub fn encode(&self) -> usize {
                match self {
                    $(Self::$table(row) => (row.pos() + 1) << $size | $code,)*
                }
            }
        }
        $(
            impl<'a> From<$table<'a>> for $name<'a> {
                fn from(from: $table<'a>) -> Self {
                    Self::$table(from)
                }
            }
        )*
    };
}

code! { AttributeType(3)
    (MethodDef, 2)
    (MemberRef, 3)
}

impl<'a> AttributeType<'a> {
    pub fn parent(&self) -> MemberRefParent<'a> {
        match self {
            Self::MethodDef(row) => row.parent(),
            Self::MemberRef(row) => row.parent(),
        }
    }

    pub fn signature(&self, generics: &[Type]) -> Signature {
        match self {
            Self::MethodDef(row) => row.signature(generics),
            Self::MemberRef(row) => row.signature(generics),
        }
    }
}

code! { HasAttribute(5)
    (MethodDef, 0)
    (Field, 1)
    (TypeRef, 2)
    (TypeDef, 3)
    (MethodParam, 4)
    (InterfaceImpl, 5)
    (MemberRef, 6)
    (TypeSpec, 13)
    (GenericParam, 19)
}

code! { HasConstant(2)
    (Field, 0)
}

code! { MemberForwarded(1)
    (MethodDef, 1)
}

code! { MemberRefParent(3)
    (TypeDef, 0)
    (TypeRef, 1)
}

impl MemberRefParent<'_> {
    pub fn namespace(&self) -> &str {
        match self {
            Self::TypeDef(row) => row.namespace(),
            Self::TypeRef(row) => row.namespace(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::TypeDef(row) => row.name(),
            Self::TypeRef(row) => row.name(),
        }
    }
}

code! { TypeDefOrRef(2)
    (TypeDef, 0)
    (TypeRef, 1)
    (TypeSpec, 2)
}

code! { TypeOrMethodDef(1)
    (TypeDef, 0)
}

impl TypeDefOrRef<'_> {
    pub fn namespace(&self) -> &str {
        match self {
            Self::TypeDef(row) => row.namespace(),
            Self::TypeRef(row) => row.namespace(),
            rest => panic!("{rest:?}"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::TypeDef(row) => row.name(),
            Self::TypeRef(row) => row.name(),
            rest => panic!("{rest:?}"),
        }
    }

    pub fn ty(&self, generics: &[Type]) -> Type {
        if let Self::TypeSpec(def) = self {
            return def.ty(generics);
        }

        Type::named(self.namespace(), self.name())
    }
}

code! { ResolutionScope(2)
    (Module, 0)
    (ModuleRef, 1)
    (AssemblyRef, 2)
    (TypeRef, 3)
}
