mod private {
    pub trait Sealed {}
}

/// A typed ECMA-335 table marker.
pub trait Table: private::Sealed {
    /// The table identifier represented by this marker.
    const ID: TableId;
}

/// Describes one ECMA-335 table.
#[derive(Debug)]
pub struct TableSchema {
    id: TableId,
    name: &'static str,
    columns: &'static [Column],
    sorted_column: Option<usize>,
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

    pub(crate) const fn sorted_column(&self) -> Option<usize> {
        self.sorted_column
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
    List(TableId),
    Coded(CodedIndex),
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CodedTarget {
    pub(crate) tag: u32,
    pub(crate) table: TableId,
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

    pub(crate) const fn targets(self) -> &'static [CodedTarget] {
        use TableId::*;
        match self {
            Self::TypeDefOrRef => &[
                CodedTarget {
                    tag: 0,
                    table: TypeDef,
                },
                CodedTarget {
                    tag: 1,
                    table: TypeRef,
                },
                CodedTarget {
                    tag: 2,
                    table: TypeSpec,
                },
            ],
            Self::HasConstant => &[
                CodedTarget {
                    tag: 0,
                    table: Field,
                },
                CodedTarget {
                    tag: 1,
                    table: Param,
                },
                CodedTarget {
                    tag: 2,
                    table: Property,
                },
            ],
            Self::HasCustomAttribute => &[
                CodedTarget {
                    tag: 0,
                    table: MethodDef,
                },
                CodedTarget {
                    tag: 1,
                    table: Field,
                },
                CodedTarget {
                    tag: 2,
                    table: TypeRef,
                },
                CodedTarget {
                    tag: 3,
                    table: TypeDef,
                },
                CodedTarget {
                    tag: 4,
                    table: Param,
                },
                CodedTarget {
                    tag: 5,
                    table: InterfaceImpl,
                },
                CodedTarget {
                    tag: 6,
                    table: MemberRef,
                },
                CodedTarget {
                    tag: 7,
                    table: Module,
                },
                CodedTarget {
                    tag: 8,
                    table: DeclSecurity,
                },
                CodedTarget {
                    tag: 9,
                    table: Property,
                },
                CodedTarget {
                    tag: 10,
                    table: Event,
                },
                CodedTarget {
                    tag: 11,
                    table: StandAloneSig,
                },
                CodedTarget {
                    tag: 12,
                    table: ModuleRef,
                },
                CodedTarget {
                    tag: 13,
                    table: TypeSpec,
                },
                CodedTarget {
                    tag: 14,
                    table: Assembly,
                },
                CodedTarget {
                    tag: 15,
                    table: AssemblyRef,
                },
                CodedTarget {
                    tag: 16,
                    table: File,
                },
                CodedTarget {
                    tag: 17,
                    table: ExportedType,
                },
                CodedTarget {
                    tag: 18,
                    table: ManifestResource,
                },
                CodedTarget {
                    tag: 19,
                    table: GenericParam,
                },
                CodedTarget {
                    tag: 20,
                    table: GenericParamConstraint,
                },
                CodedTarget {
                    tag: 21,
                    table: MethodSpec,
                },
            ],
            Self::HasFieldMarshal => &[
                CodedTarget {
                    tag: 0,
                    table: Field,
                },
                CodedTarget {
                    tag: 1,
                    table: Param,
                },
            ],
            Self::HasDeclSecurity => &[
                CodedTarget {
                    tag: 0,
                    table: TypeDef,
                },
                CodedTarget {
                    tag: 1,
                    table: MethodDef,
                },
                CodedTarget {
                    tag: 2,
                    table: Assembly,
                },
            ],
            Self::MemberRefParent => &[
                CodedTarget {
                    tag: 0,
                    table: TypeDef,
                },
                CodedTarget {
                    tag: 1,
                    table: TypeRef,
                },
                CodedTarget {
                    tag: 2,
                    table: ModuleRef,
                },
                CodedTarget {
                    tag: 3,
                    table: MethodDef,
                },
                CodedTarget {
                    tag: 4,
                    table: TypeSpec,
                },
            ],
            Self::HasSemantics => &[
                CodedTarget {
                    tag: 0,
                    table: Event,
                },
                CodedTarget {
                    tag: 1,
                    table: Property,
                },
            ],
            Self::MethodDefOrRef => &[
                CodedTarget {
                    tag: 0,
                    table: MethodDef,
                },
                CodedTarget {
                    tag: 1,
                    table: MemberRef,
                },
            ],
            Self::MemberForwarded => &[
                CodedTarget {
                    tag: 0,
                    table: Field,
                },
                CodedTarget {
                    tag: 1,
                    table: MethodDef,
                },
            ],
            Self::Implementation => &[
                CodedTarget {
                    tag: 0,
                    table: File,
                },
                CodedTarget {
                    tag: 1,
                    table: AssemblyRef,
                },
                CodedTarget {
                    tag: 2,
                    table: ExportedType,
                },
            ],
            Self::CustomAttributeType => &[
                CodedTarget {
                    tag: 2,
                    table: MethodDef,
                },
                CodedTarget {
                    tag: 3,
                    table: MemberRef,
                },
            ],
            Self::ResolutionScope => &[
                CodedTarget {
                    tag: 0,
                    table: Module,
                },
                CodedTarget {
                    tag: 1,
                    table: ModuleRef,
                },
                CodedTarget {
                    tag: 2,
                    table: AssemblyRef,
                },
                CodedTarget {
                    tag: 3,
                    table: TypeRef,
                },
            ],
            Self::TypeOrMethodDef => &[
                CodedTarget {
                    tag: 0,
                    table: TypeDef,
                },
                CodedTarget {
                    tag: 1,
                    table: MethodDef,
                },
            ],
        }
    }

    pub(crate) fn encode(self, table: TableId, row: u32) -> Option<u32> {
        let target = self.targets().iter().find(|target| target.table == table)?;
        row.checked_shl(self.tag_bits())?.checked_add(target.tag)
    }

    pub(crate) fn target(self, tag: u32) -> Option<TableId> {
        self.targets()
            .iter()
            .find(|target| target.tag == tag)
            .map(|target| target.table)
    }
}

macro_rules! tables {
    (@sorted) => {
        None
    };
    (@sorted $column:literal) => {
        Some($column)
    };
    ($(
        $number:literal => $id:ident, $name:literal, [$($column:expr),* $(,)?]
        $(, sorted $sorted:literal)?;
    )+) => {
        /// Identifies one of the 45 standard ECMA-335 metadata tables.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[repr(u8)]
        pub enum TableId {
            $($id = $number,)+
        }

        impl TableId {
            /// The number of standard tables.
            pub const COUNT: usize = [$(Self::$id),+].len();

            /// Returns the table identifier for an ECMA table number.
            pub const fn from_u8(value: u8) -> Option<Self> {
                match value {
                    $($number => Some(Self::$id),)+
                    _ => None,
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

        /// Typed markers for the standard ECMA-335 metadata tables.
        pub mod tables {
            $(
                #[doc = concat!("Marks the `", $name, "` metadata table.")]
                #[derive(Debug)]
                pub enum $id {}

                impl super::private::Sealed for $id {}

                impl super::Table for $id {
                    const ID: super::TableId = super::TableId::$id;
                }
            )+
        }

        /// The complete standard ECMA-335 table schema.
        pub static TABLES: [TableSchema; TableId::COUNT] = [
            $(TableSchema {
                id: TableId::$id,
                name: $name,
                columns: &[$($column),*],
                sorted_column: tables!(@sorted $($sorted)?),
            },)+
        ];
    };
}

use CodedIndex as Code;
use Column::{Blob, Coded, Guid, List, String, Table as Ref, U16, U32};

tables! {
    0x00 => Module, "Module", [U16, String, Guid, Guid, Guid];
    0x01 => TypeRef, "TypeRef", [Coded(Code::ResolutionScope), String, String];
    0x02 => TypeDef, "TypeDef", [
        U32, String, String, Coded(Code::TypeDefOrRef), List(TableId::Field),
        List(TableId::MethodDef)
    ];
    0x03 => FieldPtr, "FieldPtr", [Ref(TableId::Field)];
    0x04 => Field, "Field", [U16, String, Blob];
    0x05 => MethodPtr, "MethodPtr", [Ref(TableId::MethodDef)];
    0x06 => MethodDef, "MethodDef", [
        U32, U16, U16, String, Blob, List(TableId::Param)
    ];
    0x07 => ParamPtr, "ParamPtr", [Ref(TableId::Param)];
    0x08 => Param, "Param", [U16, U16, String];
    0x09 => InterfaceImpl, "InterfaceImpl", [
        Ref(TableId::TypeDef), Coded(Code::TypeDefOrRef)
    ], sorted 0;
    0x0a => MemberRef, "MemberRef", [Coded(Code::MemberRefParent), String, Blob];
    0x0b => Constant, "Constant", [U16, Coded(Code::HasConstant), Blob], sorted 1;
    0x0c => CustomAttribute, "CustomAttribute", [
        Coded(Code::HasCustomAttribute), Coded(Code::CustomAttributeType), Blob
    ], sorted 0;
    0x0d => FieldMarshal, "FieldMarshal", [Coded(Code::HasFieldMarshal), Blob], sorted 0;
    0x0e => DeclSecurity, "DeclSecurity", [U16, Coded(Code::HasDeclSecurity), Blob], sorted 1;
    0x0f => ClassLayout, "ClassLayout", [U16, U32, Ref(TableId::TypeDef)], sorted 2;
    0x10 => FieldLayout, "FieldLayout", [U32, Ref(TableId::Field)], sorted 1;
    0x11 => StandAloneSig, "StandAloneSig", [Blob];
    0x12 => EventMap, "EventMap", [Ref(TableId::TypeDef), List(TableId::Event)], sorted 0;
    0x13 => EventPtr, "EventPtr", [Ref(TableId::Event)];
    0x14 => Event, "Event", [U16, String, Coded(Code::TypeDefOrRef)];
    0x15 => PropertyMap, "PropertyMap", [
        Ref(TableId::TypeDef), List(TableId::Property)
    ], sorted 0;
    0x16 => PropertyPtr, "PropertyPtr", [Ref(TableId::Property)];
    0x17 => Property, "Property", [U16, String, Blob];
    0x18 => MethodSemantics, "MethodSemantics", [
        U16, Ref(TableId::MethodDef), Coded(Code::HasSemantics)
    ], sorted 2;
    0x19 => MethodImpl, "MethodImpl", [
        Ref(TableId::TypeDef), Coded(Code::MethodDefOrRef), Coded(Code::MethodDefOrRef)
    ], sorted 0;
    0x1a => ModuleRef, "ModuleRef", [String];
    0x1b => TypeSpec, "TypeSpec", [Blob];
    0x1c => ImplMap, "ImplMap", [
        U16, Coded(Code::MemberForwarded), String, Ref(TableId::ModuleRef)
    ], sorted 1;
    0x1d => FieldRva, "FieldRVA", [U32, Ref(TableId::Field)], sorted 1;
    0x1e => EncLog, "ENCLog", [U32, U32];
    0x1f => EncMap, "ENCMap", [U32];
    0x20 => Assembly, "Assembly", [
        U32, U16, U16, U16, U16, U32, Blob, String, String
    ];
    0x21 => AssemblyProcessor, "AssemblyProcessor", [U32];
    0x22 => AssemblyOs, "AssemblyOS", [U32, U32, U32];
    0x23 => AssemblyRef, "AssemblyRef", [
        U16, U16, U16, U16, U32, Blob, String, String, Blob
    ];
    0x24 => AssemblyRefProcessor, "AssemblyRefProcessor", [
        U32, Ref(TableId::AssemblyRef)
    ];
    0x25 => AssemblyRefOs, "AssemblyRefOS", [
        U32, U32, U32, Ref(TableId::AssemblyRef)
    ];
    0x26 => File, "File", [U32, String, Blob];
    0x27 => ExportedType, "ExportedType", [
        U32, U32, String, String, Coded(Code::Implementation)
    ];
    0x28 => ManifestResource, "ManifestResource", [
        U32, U32, String, Coded(Code::Implementation)
    ];
    0x29 => NestedClass, "NestedClass", [
        Ref(TableId::TypeDef), Ref(TableId::TypeDef)
    ], sorted 0;
    0x2a => GenericParam, "GenericParam", [
        U16, U16, Coded(Code::TypeOrMethodDef), String
    ], sorted 2;
    0x2b => MethodSpec, "MethodSpec", [Coded(Code::MethodDefOrRef), Blob];
    0x2c => GenericParamConstraint, "GenericParamConstraint", [
        Ref(TableId::GenericParam), Coded(Code::TypeDefOrRef)
    ], sorted 0;
}

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

    #[test]
    fn custom_attribute_type_preserves_sparse_tags() {
        let code = CodedIndex::CustomAttributeType;
        assert_eq!(code.target(0), None);
        assert_eq!(code.target(1), None);
        assert_eq!(code.target(2), Some(TableId::MethodDef));
        assert_eq!(code.target(3), Some(TableId::MemberRef));
    }
}
