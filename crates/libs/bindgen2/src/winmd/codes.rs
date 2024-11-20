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
                    rest => panic!("windows-bindgen: {rest:?}"),
                }
            }
        }
        impl $name {
            #[allow(dead_code)]
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

impl TypeDefOrRef {
    pub fn type_name(&self) -> TypeName<'static> {
        TypeName(self.namespace(), self.name())
    }

    fn name(&self) -> &'static str {
        match self {
            Self::TypeDef(row) => row.name(),
            Self::TypeRef(row) => row.name(),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    fn namespace(&self) -> &'static str {
        match self {
            Self::TypeDef(row) => row.namespace(),
            Self::TypeRef(row) => row.namespace(),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    pub fn reader(&self) -> &'static Reader {
        match self {
            Self::TypeDef(row) => row.reader(),
            Self::TypeRef(row) => row.reader(),
            Self::TypeSpec(row) => row.reader(),
        }
    }
}
