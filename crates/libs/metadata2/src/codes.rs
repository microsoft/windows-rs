use super::*;

pub trait Decode {
    fn decode(file: &'static File, code: usize) -> Self;
}

macro_rules! code {
    ($name:ident($size:literal) $(($table:ident, $code:literal))+) => {
        #[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
        pub enum $name {
            $($table($table),)*
        }
        impl Decode for $name {
            fn decode(file: &'static File, code: usize) -> Self {
                let (kind, row) = (code & ((1 << $size) - 1), (code >> $size) - 1);
                match kind {
                    $($code => Self::$table($table(Row::new(file, row))),)*
                    rest => unimplemented!("{rest:?}"),
                }
            }
        }
        impl $name {
            pub fn encode(&self) -> usize {
                match self {
                    $(Self::$table(row) => (row.index() + 1) << $size | $code,)*
                }
            }
        }
        $(
            impl From<$table> for $name {
                fn from(from: $table) -> Self {
                    Self::$table(from)
                }
            }
        )*
    };
}

code! { AttributeType(3)
    (MemberRef, 3)
}

code! { HasAttribute(5)
    (MethodDef, 0)
    (Field, 1)
    (TypeRef, 2)
    (TypeDef, 3)
    (Param, 4)
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
    (TypeRef, 1)
}

code! { TypeDefOrRef(2)
    (TypeDef, 0)
    (TypeRef, 1)
    (TypeSpec, 2)
}

code! { TypeOrMethodDef(1)
    (TypeDef, 0)
}

code! { ResolutionScope(2)
    (Module, 0)
    (ModuleRef, 1)
    (AssemblyRef, 2)
    (TypeRef, 3)
}

impl TypeDefOrRef {
    pub fn type_name(&self) -> TypeName {
        match self {
            Self::TypeDef(row) => row.type_name(),
            Self::TypeRef(row) => row.type_name(),
            rest => unimplemented!("{rest:?}"),
        }
    }
}
