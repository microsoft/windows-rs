use super::*;

macro_rules! code {
    ($name:ident($size:literal) $(($table:ident, $code:literal))+) => {
        #[derive(Clone, Copy, Eq, PartialEq, Hash)]
        pub enum $name {
            $($table(id::$table),)*
        }
        impl $name {
            pub fn encode(&self) -> u32 {
                match self {
                    $(Self::$table(row) => (row.0.overflowing_add(1).0) << $size | $code,)*
                }
            }
        }
        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.encode().cmp(&other.encode())
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
    };
}

code! { TypeDefOrRef(2)
    (TypeDef, 0)
    (TypeRef, 1)
    (TypeSpec, 2)
}

impl Default for TypeDefOrRef {
    fn default() -> Self {
        // This results in an encoded value of zero.
        TypeDefOrRef::TypeDef(id::TypeDef(u32::MAX))
    }
}

code! { ResolutionScope(2)
    (Module, 0)
    (ModuleRef, 1)
    (AssemblyRef, 2)
    (TypeRef, 3)
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

code! { AttributeType(3)
    (MemberRef, 3)
}

code! { MemberRefParent(3)
    (TypeRef, 1)
}

code! { HasConstant(2)
    (Field, 0)
}

code! { MemberForwarded(1)
    (MethodDef, 1)
}

code! { TypeOrMethodDef(1)
    (TypeDef, 0)
}
