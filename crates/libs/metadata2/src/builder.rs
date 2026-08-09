use super::*;

/// An error encountered while building a metadata image.
#[derive(Debug)]
pub struct BuildError(&'static str);

impl BuildError {
    /// Creates a build error with a static diagnostic.
    pub const fn new(message: &'static str) -> Self {
        Self(message)
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for BuildError {}

/// A typed identity for a TypeDef row being built.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeDefinitionId(u32);

/// A typed identity for a TypeRef row being built.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeReferenceId(u32);

/// A type identity accepted by signatures and base-type references.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildTypeIdentity {
    Definition(TypeDefinitionId),
    Reference(TypeReferenceId),
}

/// A field type accepted by the initial metadata builder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildType {
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
    Value(BuildTypeIdentity),
    Class(BuildTypeIdentity),
}

/// Incrementally builds a bounded ECMA-335 metadata image.
pub struct MetadataBuilder {
    strings: Heap,
    blobs: Heap,
    type_refs: Vec<TypeRefRow>,
    type_defs: Vec<TypeDefRow>,
    fields: Vec<FieldRow>,
    constants: Vec<ConstantRow>,
    assembly_name: u32,
    module_name: u32,
    mscorlib_name: u32,
    mscorlib_key: u32,
}

struct Heap {
    bytes: Vec<u8>,
    values: HashMap<Vec<u8>, u32>,
    terminated: bool,
}

struct TypeRefRow {
    scope: u32,
    name: u32,
    namespace: u32,
}

struct TypeDefRow {
    flags: u32,
    name: u32,
    namespace: u32,
    extends: u32,
    field_list: u32,
}

struct FieldRow {
    flags: u16,
    name: u32,
    signature: u32,
}

struct ConstantRow {
    ty: u8,
    parent: u32,
    value: u32,
}

/// Adds fields to one type while preserving the TypeDef field-list invariant.
pub struct TypeFields<'a> {
    builder: &'a mut MetadataBuilder,
    definition: TypeDefinitionId,
}

impl MetadataBuilder {
    /// Creates a metadata image builder with one module and assembly.
    pub fn new(name: &str) -> Self {
        let mut strings = Heap::strings();
        let mut blobs = Heap::blobs();
        let assembly_name = strings.insert(name.as_bytes());
        let module_name = assembly_name;
        let mscorlib_name = strings.insert(b"mscorlib");
        let mscorlib_key = blobs.insert(&[0xb7, 0x7a, 0x5c, 0x56, 0x19, 0x34, 0xe0, 0x89]);
        let mut result = Self {
            strings,
            blobs,
            type_refs: Vec::new(),
            type_defs: Vec::new(),
            fields: Vec::new(),
            constants: Vec::new(),
            assembly_name,
            module_name,
            mscorlib_name,
            mscorlib_key,
        };
        result.type_defs.push(TypeDefRow {
            flags: 0,
            name: result.strings.insert(b"<Module>"),
            namespace: 0,
            extends: 0,
            field_list: 1,
        });
        result
    }

    /// Adds a type reference scoped to mscorlib.
    pub fn type_reference(&mut self, namespace: &str, name: &str) -> TypeReferenceId {
        if let Some((index, _)) = self.type_refs.iter().enumerate().find(|(_, row)| {
            self.strings.value(row.namespace) == namespace.as_bytes()
                && self.strings.value(row.name) == name.as_bytes()
        }) {
            return TypeReferenceId(index as u32 + 1);
        }
        self.type_refs.push(TypeRefRow {
            scope: (1 << 2) | 2,
            name: self.strings.insert(name.as_bytes()),
            namespace: self.strings.insert(namespace.as_bytes()),
        });
        TypeReferenceId(self.type_refs.len() as u32)
    }

    /// Adds one type definition and all of its fields as one ordered unit.
    pub fn type_definition(
        &mut self,
        namespace: &str,
        name: &str,
        extends: Option<BuildTypeIdentity>,
        flags: u32,
        add_fields: impl FnOnce(&mut TypeFields<'_>) -> Result<(), BuildError>,
    ) -> Result<TypeDefinitionId, BuildError> {
        let type_count = self.type_defs.len();
        let field_count = self.fields.len();
        let constant_count = self.constants.len();
        let definition = TypeDefinitionId(self.type_defs.len() as u32 + 1);
        self.type_defs.push(TypeDefRow {
            flags,
            name: self.strings.insert(name.as_bytes()),
            namespace: self.strings.insert(namespace.as_bytes()),
            extends: extends.map_or(0, encode_type_identity),
            field_list: self.fields.len() as u32 + 1,
        });
        if let Err(error) = add_fields(&mut TypeFields {
            builder: self,
            definition,
        }) {
            self.type_defs.truncate(type_count);
            self.fields.truncate(field_count);
            self.constants.truncate(constant_count);
            return Err(error);
        }
        Ok(definition)
    }

    /// Serializes the metadata tables, heaps, and PE/CLI container.
    pub fn finish(mut self) -> Result<Vec<u8>, BuildError> {
        self.constants.sort_by_key(|row| row.parent);
        let tables = self.tables()?;
        if self.strings.bytes.len() > u16::MAX as usize
            || self.blobs.bytes.len() > u16::MAX as usize
        {
            return Err(BuildError("initial metadata builder heap limit exceeded"));
        }
        builder_image::metadata_image(&tables, &self.strings.bytes, &[0; 16], &self.blobs.bytes)
    }

    fn tables(&self) -> Result<Vec<u8>, BuildError> {
        let counts = [
            (0x00, 1usize),
            (0x01, self.type_refs.len()),
            (0x02, self.type_defs.len()),
            (0x04, self.fields.len()),
            (0x0b, self.constants.len()),
            (0x20, 1),
            (0x23, 1),
        ];
        if counts.iter().any(|(_, count)| *count > u16::MAX as usize) {
            return Err(BuildError("initial metadata builder table limit exceeded"));
        }
        let valid = counts
            .iter()
            .filter(|(_, count)| *count != 0)
            .fold(0u64, |mask, (table, _)| mask | (1u64 << table));
        let sorted = if self.constants.is_empty() {
            0
        } else {
            1u64 << 0x0b
        };
        let mut bytes = Vec::new();
        bytes.u32(0);
        bytes.extend([2, 0, 0, 1]);
        bytes.u64(valid);
        bytes.u64(sorted);
        for (_, count) in counts.iter().filter(|(_, count)| *count != 0) {
            bytes.u32(*count as u32);
        }

        bytes.u16(0);
        bytes.index(self.module_name)?;
        bytes.u16(1);
        bytes.u16(0);
        bytes.u16(0);

        for row in &self.type_refs {
            bytes.index(row.scope)?;
            bytes.index(row.name)?;
            bytes.index(row.namespace)?;
        }
        for row in &self.type_defs {
            bytes.u32(row.flags);
            bytes.index(row.name)?;
            bytes.index(row.namespace)?;
            bytes.index(row.extends)?;
            bytes.index(row.field_list)?;
            bytes.index(1)?;
        }
        for row in &self.fields {
            bytes.u16(row.flags);
            bytes.index(row.name)?;
            bytes.index(row.signature)?;
        }
        for row in &self.constants {
            bytes.extend([row.ty, 0]);
            bytes.index(row.parent)?;
            bytes.index(row.value)?;
        }

        bytes.u32(0x0000_8004);
        bytes.u16(0xff);
        bytes.u16(0xff);
        bytes.u16(0xff);
        bytes.u16(0xff);
        bytes.u32(0x0200);
        bytes.u16(0);
        bytes.index(self.assembly_name)?;
        bytes.u16(0);

        bytes.u16(4);
        bytes.u16(0);
        bytes.u16(0);
        bytes.u16(0);
        bytes.u32(0);
        bytes.index(self.mscorlib_key)?;
        bytes.index(self.mscorlib_name)?;
        bytes.u16(0);
        bytes.u16(0);
        Ok(bytes)
    }
}

impl TypeFields<'_> {
    /// Returns the enclosing TypeDef identity.
    pub const fn definition(&self) -> TypeDefinitionId {
        self.definition
    }

    /// Adds a field and returns its one-based row number.
    pub fn field(&mut self, name: &str, ty: BuildType, flags: u16) -> Result<u32, BuildError> {
        let mut signature = vec![CALL_CONVENTION_FIELD];
        write_type(&mut signature, ty);
        let signature = self.builder.blobs.insert(&signature);
        self.builder.fields.push(FieldRow {
            flags,
            name: self.builder.strings.insert(name.as_bytes()),
            signature,
        });
        Ok(self.builder.fields.len() as u32)
    }

    /// Adds a constant to a field.
    pub fn constant(&mut self, field: u32, value: ConstantValue) -> Result<(), BuildError> {
        if field == 0 || field > self.builder.fields.len() as u32 {
            return Err(BuildError("constant field identity is out of bounds"));
        }
        let (ty, bytes) = write_value(value);
        self.builder.constants.push(ConstantRow {
            ty,
            parent: field << 2,
            value: self.builder.blobs.insert(&bytes),
        });
        Ok(())
    }
}

impl Heap {
    fn strings() -> Self {
        Self {
            bytes: vec![0],
            values: HashMap::new(),
            terminated: true,
        }
    }

    fn blobs() -> Self {
        Self {
            bytes: vec![0],
            values: HashMap::new(),
            terminated: false,
        }
    }

    fn insert(&mut self, value: &[u8]) -> u32 {
        if let Some(offset) = self.values.get(value) {
            return *offset;
        }
        let offset = self.bytes.len() as u32;
        if self.terminated {
            self.bytes.extend(value);
            self.bytes.push(0);
        } else {
            write_compressed(&mut self.bytes, value.len() as u32);
            self.bytes.extend(value);
        }
        self.values.insert(value.to_vec(), offset);
        offset
    }

    fn value(&self, offset: u32) -> &[u8] {
        let value = &self.bytes[offset as usize..];
        &value[..value.iter().position(|byte| *byte == 0).unwrap()]
    }
}

fn encode_type_identity(value: BuildTypeIdentity) -> u32 {
    match value {
        BuildTypeIdentity::Definition(value) => value.0 << 2,
        BuildTypeIdentity::Reference(value) => (value.0 << 2) | 1,
    }
}

fn write_type(bytes: &mut Vec<u8>, ty: BuildType) {
    let code = match ty {
        BuildType::Boolean => TYPE_BOOLEAN,
        BuildType::Char => TYPE_CHAR,
        BuildType::I8 => TYPE_I8,
        BuildType::U8 => TYPE_U8,
        BuildType::I16 => TYPE_I16,
        BuildType::U16 => TYPE_U16,
        BuildType::I32 => TYPE_I32,
        BuildType::U32 => TYPE_U32,
        BuildType::I64 => TYPE_I64,
        BuildType::U64 => TYPE_U64,
        BuildType::F32 => TYPE_F32,
        BuildType::F64 => TYPE_F64,
        BuildType::ISize => TYPE_ISIZE,
        BuildType::USize => TYPE_USIZE,
        BuildType::Value(value) => {
            bytes.push(TYPE_VALUE);
            write_compressed(bytes, encode_type_identity(value));
            return;
        }
        BuildType::Class(value) => {
            bytes.push(TYPE_CLASS);
            write_compressed(bytes, encode_type_identity(value));
            return;
        }
    };
    bytes.push(code);
}

fn write_value(value: ConstantValue) -> (u8, Vec<u8>) {
    match value {
        ConstantValue::Boolean(value) => (TYPE_BOOLEAN, vec![value as u8]),
        ConstantValue::Char(value) => (TYPE_CHAR, value.to_le_bytes().to_vec()),
        ConstantValue::I8(value) => (TYPE_I8, value.to_le_bytes().to_vec()),
        ConstantValue::U8(value) => (TYPE_U8, value.to_le_bytes().to_vec()),
        ConstantValue::I16(value) => (TYPE_I16, value.to_le_bytes().to_vec()),
        ConstantValue::U16(value) => (TYPE_U16, value.to_le_bytes().to_vec()),
        ConstantValue::I32(value) => (TYPE_I32, value.to_le_bytes().to_vec()),
        ConstantValue::U32(value) => (TYPE_U32, value.to_le_bytes().to_vec()),
        ConstantValue::I64(value) => (TYPE_I64, value.to_le_bytes().to_vec()),
        ConstantValue::U64(value) => (TYPE_U64, value.to_le_bytes().to_vec()),
        ConstantValue::ISize(value) => (TYPE_ISIZE, value.to_le_bytes().to_vec()),
        ConstantValue::USize(value) => (TYPE_USIZE, value.to_le_bytes().to_vec()),
        ConstantValue::F32(value) => (TYPE_F32, value.to_le_bytes().to_vec()),
        ConstantValue::F64(value) => (TYPE_F64, value.to_le_bytes().to_vec()),
        ConstantValue::String(value) => (
            TYPE_STRING,
            value.encode_utf16().flat_map(u16::to_le_bytes).collect(),
        ),
        ConstantValue::Null => (TYPE_CLASS, 0u32.to_le_bytes().to_vec()),
    }
}

fn write_compressed(bytes: &mut Vec<u8>, value: u32) {
    if value < 0x80 {
        bytes.push(value as u8);
    } else if value < 0x4000 {
        bytes.push((0x80 | value >> 8) as u8);
        bytes.push(value as u8);
    } else {
        bytes.push((0xc0 | value >> 24) as u8);
        bytes.push((value >> 16) as u8);
        bytes.push((value >> 8) as u8);
        bytes.push(value as u8);
    }
}

trait WriteBytes {
    fn u16(&mut self, value: u16);
    fn u32(&mut self, value: u32);
    fn u64(&mut self, value: u64);
    fn index(&mut self, value: u32) -> Result<(), BuildError>;
}

impl WriteBytes for Vec<u8> {
    fn u16(&mut self, value: u16) {
        self.extend(value.to_le_bytes());
    }

    fn u32(&mut self, value: u32) {
        self.extend(value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.extend(value.to_le_bytes());
    }

    fn index(&mut self, value: u32) -> Result<(), BuildError> {
        self.u16(
            value
                .try_into()
                .map_err(|_| BuildError("initial metadata builder index limit exceeded"))?,
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_checked_value_type_image() {
        let mut builder = MetadataBuilder::new("builder");
        let extends = builder.type_reference("System", "ValueType");
        builder
            .type_definition(
                "Test",
                "Point",
                Some(BuildTypeIdentity::Reference(extends)),
                0x0000_4109,
                |fields| {
                    fields.field("x", BuildType::I32, 6)?;
                    fields.field("y", BuildType::I32, 6)?;
                    Ok(())
                },
            )
            .unwrap();
        let image = Image::new(builder.finish().unwrap()).unwrap();
        assert_eq!(image.table(TableId::TypeDef).rows(), 2);
        assert_eq!(image.table(TableId::Field).rows(), 2);
    }
}
