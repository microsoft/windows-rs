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
    FieldLayout
    GenericParam
    ImplMap
    InterfaceImpl
    MemberRef
    MethodDef
    ModuleRef
    NestedClass
    TypeDef
    TypeRef
    TypeSpec
    Module
    AssemblyRef
    Param

    Property
    PropertyMap
    Event
    EventMap
    MethodSemantics

    BlobId
    StringId
}

/// A writer row whose table position is unchanged by finalization.
pub trait RowHandle: Copy {
    const TABLE: crate::reader::TableId;
    fn position(self) -> usize;

    /// Returns the row identity the handle will have in a finalized metadata index.
    fn row_id(self, file: usize) -> crate::reader::RowId {
        crate::reader::RowId::new(file, Self::TABLE, self.position())
    }
}

macro_rules! row_handles {
    ($(($name:ident, $table:ident))+) => {
        $(
            impl RowHandle for $name {
                const TABLE: crate::reader::TableId = crate::reader::TableId::$table;

                fn position(self) -> usize {
                    self.0 as usize
                }
            }
        )+
    };
}

row_handles! {
    (TypeDef, TypeDef)
    (Field, Field)
    (MethodDef, MethodDef)
    (Param, Param)
    (Property, Property)
    (PropertyMap, PropertyMap)
    (Event, Event)
    (EventMap, EventMap)
    (ClassLayout, ClassLayout)
    (FieldLayout, FieldLayout)
}
