#![doc = include_str!("../readme.md")]

use std::collections::BTreeMap;
use windows_metadata2::{
    BuildError, BuildType, BuildTypeIdentity, ConstantValue, FieldAttributes, MetadataBuilder,
    TypeAttributes, TypeDefinitionId,
};

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
    pub ty: Type,
}

/// A source type supported by the first authoring checkpoints.
pub enum Type {
    Primitive(Primitive),
    Named { namespace: String, name: String },
}

impl Type {
    /// Creates a named value type reference.
    pub fn named(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self::Named {
            namespace: namespace.into(),
            name: name.into(),
        }
    }
}

impl From<Primitive> for Type {
    fn from(value: Primitive) -> Self {
        Self::Primitive(value)
    }
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
        let mut identities = BTreeMap::<String, BTreeMap<String, TypeDefinitionId>>::new();
        let mut declared = Vec::new();
        for module in &self.modules {
            let mut module_ids = Vec::new();
            for definition in &module.definitions {
                let id = module.declare(definition, &mut output)?;
                if identities
                    .entry(module.namespace.clone())
                    .or_default()
                    .insert(definition.name().to_string(), id)
                    .is_some()
                {
                    return Err(BuildError::new("duplicate source type definition"));
                }
                module_ids.push(id);
            }
            declared.push(module_ids);
        }
        for (module, module_ids) in self.modules.iter().zip(declared) {
            for (definition, id) in module.definitions.iter().zip(module_ids) {
                module.compile(definition, id, &identities, &mut output)?;
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

    fn declare(
        &self,
        definition: &Definition,
        output: &mut MetadataBuilder,
    ) -> Result<TypeDefinitionId, BuildError> {
        match definition {
            Definition::Enum { name, .. } => {
                let extends = output.type_reference("System", "Enum");
                output.declare_type(
                    &self.namespace,
                    name,
                    Some(BuildTypeIdentity::Reference(extends)),
                    TypeAttributes::PUBLIC
                        | TypeAttributes::SEALED
                        | TypeAttributes::WINDOWS_RUNTIME,
                )
            }
            Definition::Struct { name, .. } => {
                let extends = output.type_reference("System", "ValueType");
                output.declare_type(
                    &self.namespace,
                    name,
                    Some(BuildTypeIdentity::Reference(extends)),
                    TypeAttributes::PUBLIC
                        | TypeAttributes::SEQUENTIAL_LAYOUT
                        | TypeAttributes::SEALED
                        | TypeAttributes::WINDOWS_RUNTIME,
                )
            }
        }
    }

    fn compile(
        &self,
        definition: &Definition,
        id: TypeDefinitionId,
        identities: &BTreeMap<String, BTreeMap<String, TypeDefinitionId>>,
        output: &mut MetadataBuilder,
    ) -> Result<(), BuildError> {
        output.define_type(id, |fields| {
            match definition {
                Definition::Enum {
                    underlying,
                    variants,
                    ..
                } => {
                    if !underlying.is_enum_integer() {
                        return Err(BuildError::new(
                            "enum underlying type is not an ECMA integer",
                        ));
                    }
                    fields.field(
                        "value__",
                        (*underlying).into(),
                        FieldAttributes::PRIVATE
                            | FieldAttributes::SPECIAL_NAME
                            | FieldAttributes::RT_SPECIAL_NAME,
                    )?;
                    let ty = BuildType::Value(BuildTypeIdentity::Definition(id));
                    for variant in variants {
                        if !underlying.matches(variant.value) {
                            return Err(BuildError::new(
                                "enum value does not match its underlying type",
                            ));
                        }
                        let field = fields.field(
                            &variant.name,
                            ty,
                            FieldAttributes::PUBLIC
                                | FieldAttributes::STATIC
                                | FieldAttributes::LITERAL,
                        )?;
                        fields.constant(field, variant.value.into())?;
                    }
                }
                Definition::Struct {
                    fields: source_fields,
                    ..
                } => {
                    for field in source_fields {
                        let ty = match &field.ty {
                            Type::Primitive(ty) => (*ty).into(),
                            Type::Named { namespace, name } => {
                                let Some(id) = identities
                                    .get(namespace)
                                    .and_then(|types| types.get(name))
                                    .copied()
                                else {
                                    return Err(BuildError::new("named field type is not defined"));
                                };
                                BuildType::Value(BuildTypeIdentity::Definition(id))
                            }
                        };
                        fields.field(&field.name, ty, FieldAttributes::PUBLIC)?;
                    }
                }
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl Definition {
    fn name(&self) -> &str {
        match self {
            Self::Enum { name, .. } | Self::Struct { name, .. } => name,
        }
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
        module.struct_type(
            "Pixel",
            vec![
                Field {
                    name: "color".to_string(),
                    ty: Type::named("Test", "Color"),
                },
                Field {
                    name: "alpha".to_string(),
                    ty: Primitive::U8.into(),
                },
            ],
        );
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
        document.module(module);

        let bytes = document.compile().unwrap();
        let path = std::env::temp_dir().join(format!("windows_rdl2_{}.winmd", std::process::id()));
        windows_rdl::reader()
            .input_text(
                r#"
                    #[winrt]
                    mod Test {
                        struct Pixel {
                            color: Color,
                            alpha: u8,
                        }
                        #[repr(i32)]
                        enum Color {
                            Red = 1,
                            Blue = 2,
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
        let pixel = database.type_definitions("Test", "Pixel");
        assert_eq!(pixel.len(), 1);
        let pixel = database.definition(pixel[0]).unwrap();
        assert_eq!(pixel.category().unwrap(), TypeCategory::Struct);
        assert_eq!(pixel.fields().unwrap().len(), 2);

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
                ("Test".to_string(), "Pixel".to_string(), 2),
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
