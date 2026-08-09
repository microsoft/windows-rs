/// Identifies one of the 45 standard ECMA-335 metadata tables.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum TableId {
    Module,
    TypeRef,
    TypeDef,
    FieldPtr,
    Field,
    MethodPtr,
    MethodDef,
    ParamPtr,
    Param,
    InterfaceImpl,
    MemberRef,
    Constant,
    CustomAttribute,
    FieldMarshal,
    DeclSecurity,
    ClassLayout,
    FieldLayout,
    StandAloneSig,
    EventMap,
    EventPtr,
    Event,
    PropertyMap,
    PropertyPtr,
    Property,
    MethodSemantics,
    MethodImpl,
    ModuleRef,
    TypeSpec,
    ImplMap,
    FieldRva,
    EncLog,
    EncMap,
    Assembly,
    AssemblyProcessor,
    AssemblyOs,
    AssemblyRef,
    AssemblyRefProcessor,
    AssemblyRefOs,
    File,
    ExportedType,
    ManifestResource,
    NestedClass,
    GenericParam,
    MethodSpec,
    GenericParamConstraint,
}

impl TableId {
    /// The number of standard tables.
    pub const COUNT: usize = 45;

    const ALL: [Self; Self::COUNT] = [
        Self::Module,
        Self::TypeRef,
        Self::TypeDef,
        Self::FieldPtr,
        Self::Field,
        Self::MethodPtr,
        Self::MethodDef,
        Self::ParamPtr,
        Self::Param,
        Self::InterfaceImpl,
        Self::MemberRef,
        Self::Constant,
        Self::CustomAttribute,
        Self::FieldMarshal,
        Self::DeclSecurity,
        Self::ClassLayout,
        Self::FieldLayout,
        Self::StandAloneSig,
        Self::EventMap,
        Self::EventPtr,
        Self::Event,
        Self::PropertyMap,
        Self::PropertyPtr,
        Self::Property,
        Self::MethodSemantics,
        Self::MethodImpl,
        Self::ModuleRef,
        Self::TypeSpec,
        Self::ImplMap,
        Self::FieldRva,
        Self::EncLog,
        Self::EncMap,
        Self::Assembly,
        Self::AssemblyProcessor,
        Self::AssemblyOs,
        Self::AssemblyRef,
        Self::AssemblyRefProcessor,
        Self::AssemblyRefOs,
        Self::File,
        Self::ExportedType,
        Self::ManifestResource,
        Self::NestedClass,
        Self::GenericParam,
        Self::MethodSpec,
        Self::GenericParamConstraint,
    ];

    /// Returns the table identifier for an ECMA table number.
    pub const fn from_u8(value: u8) -> Option<Self> {
        if value < Self::COUNT as u8 {
            Some(Self::ALL[value as usize])
        } else {
            None
        }
    }

    /// Returns the ECMA table number.
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns this table's schema.
    pub const fn schema(self) -> &'static TableSchema {
        &TABLES[self as usize]
    }
}

/// Describes one ECMA-335 table.
#[derive(Debug)]
pub struct TableSchema {
    id: TableId,
    name: &'static str,
    columns: &'static [Column],
}

impl TableSchema {
    /// Returns the table identifier.
    pub const fn id(&self) -> TableId {
        self.id
    }

    /// Returns the ECMA table name.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) const fn columns(&self) -> &'static [Column] {
        self.columns
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Column {
    U16,
    U32,
    String,
    Guid,
    Blob,
    Table(TableId),
    Coded(CodedIndex),
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum CodedIndex {
    TypeDefOrRef,
    HasConstant,
    HasCustomAttribute,
    HasFieldMarshal,
    HasDeclSecurity,
    MemberRefParent,
    HasSemantics,
    MethodDefOrRef,
    MemberForwarded,
    Implementation,
    CustomAttributeType,
    ResolutionScope,
    TypeOrMethodDef,
}

impl CodedIndex {
    pub(crate) const fn tag_bits(self) -> u32 {
        match self {
            Self::TypeDefOrRef
            | Self::HasConstant
            | Self::HasDeclSecurity
            | Self::Implementation
            | Self::ResolutionScope => 2,
            Self::HasCustomAttribute => 5,
            Self::MemberRefParent | Self::CustomAttributeType => 3,
            Self::HasFieldMarshal
            | Self::HasSemantics
            | Self::MethodDefOrRef
            | Self::MemberForwarded
            | Self::TypeOrMethodDef => 1,
        }
    }

    pub(crate) const fn tables(self) -> &'static [TableId] {
        use TableId::*;
        match self {
            Self::TypeDefOrRef => &[TypeDef, TypeRef, TypeSpec],
            Self::HasConstant => &[Field, Param, Property],
            Self::HasCustomAttribute => &[
                MethodDef,
                Field,
                TypeRef,
                TypeDef,
                Param,
                InterfaceImpl,
                MemberRef,
                Module,
                DeclSecurity,
                Property,
                Event,
                StandAloneSig,
                ModuleRef,
                TypeSpec,
                Assembly,
                AssemblyRef,
                File,
                ExportedType,
                ManifestResource,
                GenericParam,
                GenericParamConstraint,
                MethodSpec,
            ],
            Self::HasFieldMarshal => &[Field, Param],
            Self::HasDeclSecurity => &[TypeDef, MethodDef, Assembly],
            Self::MemberRefParent => &[TypeDef, TypeRef, ModuleRef, MethodDef, TypeSpec],
            Self::HasSemantics => &[Event, Property],
            Self::MethodDefOrRef => &[MethodDef, MemberRef],
            Self::MemberForwarded => &[Field, MethodDef],
            Self::Implementation => &[File, AssemblyRef, ExportedType],
            Self::CustomAttributeType => &[MethodDef, MemberRef],
            Self::ResolutionScope => &[Module, ModuleRef, AssemblyRef, TypeRef],
            Self::TypeOrMethodDef => &[TypeDef, MethodDef],
        }
    }
}

use Column::*;
use TableId::*;

macro_rules! table {
    ($id:ident, $name:literal, [$($column:expr),* $(,)?]) => {
        TableSchema {
            id: $id,
            name: $name,
            columns: &[$($column),*],
        }
    };
}

/// The complete standard ECMA-335 table schema.
pub static TABLES: [TableSchema; TableId::COUNT] = [
    table!(Module, "Module", [U16, String, Guid, Guid, Guid]),
    table!(
        TypeRef,
        "TypeRef",
        [Coded(CodedIndex::ResolutionScope), String, String]
    ),
    table!(
        TypeDef,
        "TypeDef",
        [
            U32,
            String,
            String,
            Coded(CodedIndex::TypeDefOrRef),
            Table(Field),
            Table(MethodDef),
        ]
    ),
    table!(FieldPtr, "FieldPtr", [Table(Field)]),
    table!(Field, "Field", [U16, String, Blob]),
    table!(MethodPtr, "MethodPtr", [Table(MethodDef)]),
    table!(
        MethodDef,
        "MethodDef",
        [U32, U16, U16, String, Blob, Table(Param)]
    ),
    table!(ParamPtr, "ParamPtr", [Table(Param)]),
    table!(Param, "Param", [U16, U16, String]),
    table!(
        InterfaceImpl,
        "InterfaceImpl",
        [Table(TypeDef), Coded(CodedIndex::TypeDefOrRef)]
    ),
    table!(
        MemberRef,
        "MemberRef",
        [Coded(CodedIndex::MemberRefParent), String, Blob]
    ),
    table!(
        Constant,
        "Constant",
        [U16, Coded(CodedIndex::HasConstant), Blob]
    ),
    table!(
        CustomAttribute,
        "CustomAttribute",
        [
            Coded(CodedIndex::HasCustomAttribute),
            Coded(CodedIndex::CustomAttributeType),
            Blob,
        ]
    ),
    table!(
        FieldMarshal,
        "FieldMarshal",
        [Coded(CodedIndex::HasFieldMarshal), Blob]
    ),
    table!(
        DeclSecurity,
        "DeclSecurity",
        [U16, Coded(CodedIndex::HasDeclSecurity), Blob]
    ),
    table!(ClassLayout, "ClassLayout", [U16, U32, Table(TypeDef)]),
    table!(FieldLayout, "FieldLayout", [U32, Table(Field)]),
    table!(StandAloneSig, "StandAloneSig", [Blob]),
    table!(EventMap, "EventMap", [Table(TypeDef), Table(Event)]),
    table!(EventPtr, "EventPtr", [Table(Event)]),
    table!(
        Event,
        "Event",
        [U16, String, Coded(CodedIndex::TypeDefOrRef)]
    ),
    table!(
        PropertyMap,
        "PropertyMap",
        [Table(TypeDef), Table(Property)]
    ),
    table!(PropertyPtr, "PropertyPtr", [Table(Property)]),
    table!(Property, "Property", [U16, String, Blob]),
    table!(
        MethodSemantics,
        "MethodSemantics",
        [U16, Table(MethodDef), Coded(CodedIndex::HasSemantics),]
    ),
    table!(
        MethodImpl,
        "MethodImpl",
        [
            Table(TypeDef),
            Coded(CodedIndex::MethodDefOrRef),
            Coded(CodedIndex::MethodDefOrRef),
        ]
    ),
    table!(ModuleRef, "ModuleRef", [String]),
    table!(TypeSpec, "TypeSpec", [Blob]),
    table!(
        ImplMap,
        "ImplMap",
        [
            U16,
            Coded(CodedIndex::MemberForwarded),
            String,
            Table(ModuleRef),
        ]
    ),
    table!(FieldRva, "FieldRVA", [U32, Table(Field)]),
    table!(EncLog, "ENCLog", [U32, U32]),
    table!(EncMap, "ENCMap", [U32]),
    table!(
        Assembly,
        "Assembly",
        [U32, U16, U16, U16, U16, U32, Blob, String, String]
    ),
    table!(AssemblyProcessor, "AssemblyProcessor", [U32]),
    table!(AssemblyOs, "AssemblyOS", [U32, U32, U32]),
    table!(
        AssemblyRef,
        "AssemblyRef",
        [U16, U16, U16, U16, U32, Blob, String, String, Blob,]
    ),
    table!(
        AssemblyRefProcessor,
        "AssemblyRefProcessor",
        [U32, Table(AssemblyRef)]
    ),
    table!(
        AssemblyRefOs,
        "AssemblyRefOS",
        [U32, U32, U32, Table(AssemblyRef)]
    ),
    table!(File, "File", [U32, String, Blob]),
    table!(
        ExportedType,
        "ExportedType",
        [U32, U32, String, String, Coded(CodedIndex::Implementation),]
    ),
    table!(
        ManifestResource,
        "ManifestResource",
        [U32, U32, String, Coded(CodedIndex::Implementation),]
    ),
    table!(NestedClass, "NestedClass", [Table(TypeDef), Table(TypeDef)]),
    table!(
        GenericParam,
        "GenericParam",
        [U16, U16, Coded(CodedIndex::TypeOrMethodDef), String,]
    ),
    table!(
        MethodSpec,
        "MethodSpec",
        [Coded(CodedIndex::MethodDefOrRef), Blob]
    ),
    table!(
        GenericParamConstraint,
        "GenericParamConstraint",
        [Table(GenericParam), Coded(CodedIndex::TypeDefOrRef),]
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_covers_every_standard_table() {
        assert_eq!(TABLES.len(), TableId::COUNT);
        for (number, schema) in TABLES.iter().enumerate() {
            assert_eq!(schema.id().as_u8() as usize, number);
            assert_eq!(TableId::from_u8(number as u8), Some(schema.id()));
            assert!(!schema.name().is_empty());
            assert!(!schema.columns().is_empty());
        }
        assert_eq!(TableId::from_u8(TableId::COUNT as u8), None);
    }
}
