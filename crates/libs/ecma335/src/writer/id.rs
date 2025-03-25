macro_rules! identifiers {
    ($($name:ident)+) => {
        $(
        #[derive(Default,Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd, Debug)]
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

    BlobId
    StringId
}
