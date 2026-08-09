#![doc = include_str!("../readme.md")]

use windows_metadata2::{BuildError, BuildType, BuildTypeIdentity, ConstantValue, MetadataBuilder};

const TYPE_PUBLIC: u32 = 0x0000_0001;
const TYPE_SEQUENTIAL: u32 = 0x0000_0008;
const TYPE_SEALED: u32 = 0x0000_0100;
const TYPE_WINDOWS_RUNTIME: u32 = 0x0000_4000;
const FIELD_PRIVATE: u16 = 0x0001;
const FIELD_PUBLIC: u16 = 0x0006;
const FIELD_STATIC: u16 = 0x0010;
const FIELD_LITERAL: u16 = 0x0040;
const FIELD_SPECIAL_NAME: u16 = 0x0200;
const FIELD_RT_SPECIAL_NAME: u16 = 0x0400;

/// A primitive type supported by the first authoring checkpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primitive {
    Boolean,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISize,
    USize,
}

/// One enum member.
pub struct Variant {
    pub name: String,
    pub value: Value,
}

/// One struct field.
pub struct Field {
    pub name: String,
    pub ty: Primitive,
}

/// An integer enum value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Value {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
}

enum Definition {
    Enum {
        name: String,
        underlying: Primitive,
        variants: Vec<Variant>,
    },
    Struct {
        name: String,
        fields: Vec<Field>,
    },
}

/// A namespace containing RDL definitions.
pub struct Module {
    namespace: String,
    definitions: Vec<Definition>,
}

/// A programmatic RDL document.
pub struct Document {
    assembly: String,
    modules: Vec<Module>,
}

impl Document {
    /// Creates an empty document.
    pub fn new(assembly: impl Into<String>) -> Self {
        Self {
            assembly: assembly.into(),
            modules: Vec::new(),
        }
    }

    /// Adds a namespace module.
    pub fn module(&mut self, module: Module) {
        self.modules.push(module);
    }

    /// Emits a PE/CLI metadata image.
    pub fn compile(&self) -> Result<Vec<u8>, BuildError> {
        let mut output = MetadataBuilder::new(&self.assembly);
        for module in &self.modules {
            for definition in &module.definitions {
                module.compile(definition, &mut output)?;
            }
        }
        output.finish()
    }
}

impl Module {
    /// Creates an empty namespace module.
    pub fn new(namespace: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            definitions: Vec::new(),
        }
    }

    /// Adds a primitive integer enum.
    pub fn enum_type(
        &mut self,
        name: impl Into<String>,
        underlying: Primitive,
        variants: Vec<Variant>,
    ) {
        self.definitions.push(Definition::Enum {
            name: name.into(),
            underlying,
            variants,
        });
    }

    /// Adds a primitive-field struct.
    pub fn struct_type(&mut self, name: impl Into<String>, fields: Vec<Field>) {
        self.definitions.push(Definition::Struct {
            name: name.into(),
            fields,
        });
    }

    fn compile(
        &self,
        definition: &Definition,
        output: &mut MetadataBuilder,
    ) -> Result<(), BuildError> {
        match definition {
            Definition::Enum {
                name,
                underlying,
                variants,
            } => {
                if !underlying.is_enum_integer() {
                    return Err(BuildError::new(
                        "enum underlying type is not an ECMA integer",
                    ));
                }
                let extends = output.type_reference("System", "Enum");
                output.type_definition(
                    &self.namespace,
                    name,
                    Some(BuildTypeIdentity::Reference(extends)),
                    TYPE_PUBLIC | TYPE_SEALED | TYPE_WINDOWS_RUNTIME,
                    |fields| {
                        fields.field(
                            "value__",
                            (*underlying).into(),
                            FIELD_PRIVATE | FIELD_SPECIAL_NAME | FIELD_RT_SPECIAL_NAME,
                        )?;
                        let ty =
                            BuildType::Value(BuildTypeIdentity::Definition(fields.definition()));
                        for variant in variants {
                            if !underlying.matches(variant.value) {
                                return Err(BuildError::new(
                                    "enum value does not match its underlying type",
                                ));
                            }
                            let field = fields.field(
                                &variant.name,
                                ty,
                                FIELD_PUBLIC | FIELD_STATIC | FIELD_LITERAL,
                            )?;
                            fields.constant(field, variant.value.into())?;
                        }
                        Ok(())
                    },
                )?;
            }
            Definition::Struct { name, fields } => {
                let extends = output.type_reference("System", "ValueType");
                output.type_definition(
                    &self.namespace,
                    name,
                    Some(BuildTypeIdentity::Reference(extends)),
                    TYPE_PUBLIC | TYPE_SEQUENTIAL | TYPE_SEALED | TYPE_WINDOWS_RUNTIME,
                    |output| {
                        for field in fields {
                            output.field(&field.name, field.ty.into(), FIELD_PUBLIC)?;
                        }
                        Ok(())
                    },
                )?;
            }
        }
        Ok(())
    }
}

impl Primitive {
    fn is_enum_integer(self) -> bool {
        matches!(
            self,
            Self::I8
                | Self::U8
                | Self::I16
                | Self::U16
                | Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
        )
    }

    fn matches(self, value: Value) -> bool {
        matches!(
            (self, value),
            (Self::I8, Value::I8(_))
                | (Self::U8, Value::U8(_))
                | (Self::I16, Value::I16(_))
                | (Self::U16, Value::U16(_))
                | (Self::I32, Value::I32(_))
                | (Self::U32, Value::U32(_))
                | (Self::I64, Value::I64(_))
                | (Self::U64, Value::U64(_))
        )
    }
}

impl From<Value> for ConstantValue {
    fn from(value: Value) -> Self {
        match value {
            Value::I8(value) => Self::I8(value),
            Value::U8(value) => Self::U8(value),
            Value::I16(value) => Self::I16(value),
            Value::U16(value) => Self::U16(value),
            Value::I32(value) => Self::I32(value),
            Value::U32(value) => Self::U32(value),
            Value::I64(value) => Self::I64(value),
            Value::U64(value) => Self::U64(value),
        }
    }
}

impl From<Primitive> for BuildType {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::Boolean => Self::Boolean,
            Primitive::Char => Self::Char,
            Primitive::I8 => Self::I8,
            Primitive::U8 => Self::U8,
            Primitive::I16 => Self::I16,
            Primitive::U16 => Self::U16,
            Primitive::I32 => Self::I32,
            Primitive::U32 => Self::U32,
            Primitive::I64 => Self::I64,
            Primitive::U64 => Self::U64,
            Primitive::F32 => Self::F32,
            Primitive::F64 => Self::F64,
            Primitive::ISize => Self::ISize,
            Primitive::USize => Self::USize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_metadata2::{Database, Image, TypeCategory};

    #[test]
    fn primitive_enum_and_struct_are_readable_by_both_readers() {
        let mut document = Document::new("authoring");
        let mut module = Module::new("Test");
        module.enum_type(
            "Color",
            Primitive::I32,
            vec![
                Variant {
                    name: "Red".to_string(),
                    value: Value::I32(1),
                },
                Variant {
                    name: "Blue".to_string(),
                    value: Value::I32(2),
                },
            ],
        );
        module.struct_type(
            "Point",
            vec![
                Field {
                    name: "x".to_string(),
                    ty: Primitive::I32,
                },
                Field {
                    name: "y".to_string(),
                    ty: Primitive::I32,
                },
            ],
        );
        document.module(module);

        let bytes = document.compile().unwrap();
        let path = std::env::temp_dir().join(format!("windows_rdl2_{}.winmd", std::process::id()));
        windows_rdl::reader()
            .input_text(
                r#"
                    #[winrt]
                    mod Test {
                        #[repr(i32)]
                        enum Color {
                            Red = 1,
                            Blue = 2,
                        }
                        struct Point {
                            x: i32,
                            y: i32,
                        }
                    }
                "#,
            )
            .output(&path)
            .write()
            .unwrap();
        let expected = std::fs::read(&path).unwrap();
        std::fs::remove_file(path).unwrap();
        assert_eq!(summary(&bytes), summary(&expected));

        let database = Database::new([Image::new(bytes.clone()).unwrap()]).unwrap();
        let color = database.type_definitions("Test", "Color");
        assert_eq!(color.len(), 1);
        let color = database.definition(color[0]).unwrap();
        assert_eq!(color.category().unwrap(), TypeCategory::Enum);
        assert_eq!(color.fields().unwrap().len(), 3);
        let values: Vec<_> = color
            .fields()
            .unwrap()
            .filter_map(|field| {
                field
                    .constant()
                    .unwrap()
                    .map(|constant| (field.name().unwrap().to_string(), constant.value().unwrap()))
            })
            .collect();
        assert_eq!(
            values,
            [
                ("Red".to_string(), ConstantValue::I32(1)),
                ("Blue".to_string(), ConstantValue::I32(2)),
            ]
        );
        let point = database.type_definitions("Test", "Point");
        assert_eq!(point.len(), 1);
        let point = database.definition(point[0]).unwrap();
        assert_eq!(point.category().unwrap(), TypeCategory::Struct);
        assert_eq!(point.fields().unwrap().len(), 2);

        let old = windows_metadata::reader::Index::new(vec![
            windows_metadata::reader::File::new(bytes).unwrap(),
        ]);
        let mut definitions: Vec<_> = old
            .iter()
            .map(|(namespace, name, definition)| {
                (
                    namespace.to_string(),
                    name.to_string(),
                    definition.fields().len(),
                )
            })
            .collect();
        definitions.sort();
        assert_eq!(
            definitions,
            [
                ("Test".to_string(), "Color".to_string(), 3),
                ("Test".to_string(), "Point".to_string(), 2),
            ]
        );
    }

    fn summary(bytes: &[u8]) -> Vec<(String, String, u8, u32, Vec<(String, u16, String, String)>)> {
        let database = Database::new([Image::new(bytes.to_vec()).unwrap()]).unwrap();
        let mut result: Vec<_> = database
            .definitions()
            .map(|definition| {
                (
                    definition.namespace().unwrap().to_string(),
                    definition.name().unwrap().to_string(),
                    match definition.category().unwrap() {
                        TypeCategory::Interface => 0,
                        TypeCategory::Class => 1,
                        TypeCategory::Enum => 2,
                        TypeCategory::Delegate => 3,
                        TypeCategory::Struct => 4,
                        TypeCategory::Attribute => 5,
                    },
                    definition.flags().unwrap(),
                    definition
                        .fields()
                        .unwrap()
                        .map(|field| {
                            (
                                field.name().unwrap().to_string(),
                                field.flags().unwrap(),
                                type_text(&database, field),
                                field
                                    .constant()
                                    .unwrap()
                                    .map(|constant| format!("{:?}", constant.value().unwrap()))
                                    .unwrap_or_default(),
                            )
                        })
                        .collect(),
                )
            })
            .collect();
        result.sort();
        result
    }

    fn type_text(database: &Database, field: windows_metadata2::FieldDefinition<'_>) -> String {
        let ty = field.signature().unwrap();
        assert!(ty.modifiers.is_empty());
        match ty.kind {
            windows_metadata2::TypeKind::Value(id) => {
                let (namespace, name) = database
                    .type_name(field.entity().file(), id)
                    .unwrap()
                    .unwrap();
                format!("value {namespace}.{name}")
            }
            windows_metadata2::TypeKind::Class(id) => {
                let (namespace, name) = database
                    .type_name(field.entity().file(), id)
                    .unwrap()
                    .unwrap();
                format!("class {namespace}.{name}")
            }
            kind => format!("{kind:?}"),
        }
    }
}
