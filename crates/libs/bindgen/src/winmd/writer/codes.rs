#![allow(dead_code, clippy::enum_variant_names)]

macro_rules! code {
    ($name:ident($size:literal) $(($table:ident, $code:literal))+) => {
        #[derive(Clone, Copy)]
        pub enum $name {
            $($table(u32),)*
        }
        impl $name {
            pub fn encode(&self) -> u32 {
                match self {
                    $(Self::$table(row) => (row.overflowing_add(1).0) << $size | $code,)*
                }
            }
        }
    };
}

code! { AttributeType(3)
    (MemberRef, 3)
}

// TODO: needs to be called HasCustomAttribute
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

impl TypeDefOrRef {
    pub fn none() -> Self {
        TypeDefOrRef::TypeDef(u32::MAX)
    }
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
