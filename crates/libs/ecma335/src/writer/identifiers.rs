macro_rules! identifiers {
    ($($name:ident)+) => {
        $(
        #[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
        pub struct $name(pub(crate) u32);
    )*
    };
}

identifiers! {
    Attribute
    ClassLayout
    Constant
    Field
    GenericParam
    ImplMap
    InterfaceImpl
    MemberRef
    MethodDef
    ModuleRef
    NestedClass
    MethodParam
    TypeDef
    TypeRef
    TypeSpec
    Module
    AssemblyRef
    Param

    FieldSig
    MethodDefSig
    AttributeValue
    ConstantValue
}
