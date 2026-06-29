use super::*;
mod into_stream;

mod rec;

mod blobs;
use blobs::*;

mod strings;
use strings::*;

mod helpers;
use helpers::*;

/// Represents an ECMA-335 file in memory so that it can be built incrementally.
#[derive(Default)]
pub struct File {
    strings: Strings,
    blobs: Blobs,
    records: rec::Records,
    reference: Option<reader::Index>,

    // Indexes for fast lookup of preexisting rows.
    TypeRef: HashMap<String, HashMap<String, TypeRef>>,
    TypeSpec: HashMap<BlobId, TypeSpec>,
    AssemblyRef: HashMap<String, AssemblyRef>,
    ModuleRef: HashMap<String, ModuleRef>,
    MemberRef: HashMap<rec::MemberRef, MemberRef>,

    // Staging for sorted rows before these records can be written. BTreeMap is used rather than HashMap to allow reproducible builds.
    Constant: BTreeMap<HasConstant, rec::Constant>,
    Attribute: BTreeMap<HasAttribute, Vec<rec::Attribute>>,
    GenericParam: BTreeMap<TypeOrMethodDef, Vec<rec::GenericParam>>,
}

impl File {
    /// Creates a minimal ECMA-335 file representation.
    pub fn new(name: &str) -> Self {
        let mut file = Self::default();

        // This assembly.
        file.records.Assembly.push(rec::Assembly {
            Name: file.strings.insert(name),
            HashAlgId: 0x00008004,
            MajorVersion: 0xFF,
            MinorVersion: 0xFF,
            BuildNumber: 0xFF,
            RevisionNumber: 0xFF,
            Flags: AssemblyFlags::WindowsRuntime,
            ..Default::default()
        });

        // This module.
        file.records.Module.push(rec::Module {
            Name: file.strings.insert(name),
            Mvid: 1,
            ..Default::default()
        });

        // Some parsers will fail to read without an `mscorlib` reference implied by "System" types.
        file.AssemblyRef("System");

        // The parent type of "globals" expected by most parsers.
        file.TypeDef("", "<Module>", TypeDefOrRef::default(), TypeAttributes(0));

        file
    }

    /// Sets the reference `Index` used to resolve whether a `TypeRef` refers to a type
    /// defined locally in this file or in an external assembly.
    pub fn set_reference(&mut self, reference: reader::Index) {
        self.reference = Some(reference);
    }

    /// Returns the reference `Index`, if one has been set via [`Self::set_reference`].
    pub fn reference(&self) -> Option<&reader::Index> {
        self.reference.as_ref()
    }

    fn ModuleRef(&mut self, name: &str) -> ModuleRef {
        if let Some(pos) = self.ModuleRef.get(name) {
            return *pos;
        }

        let pos = ModuleRef(self.records.ModuleRef.push_pos(rec::ModuleRef {
            Name: self.strings.insert(name),
        }));

        self.ModuleRef.insert(name.to_string(), pos);
        pos
    }

    pub fn ImplMap(
        &mut self,
        method: MethodDef,
        flags: PInvokeAttributes,
        import_name: &str,
        import_scope: &str,
    ) {
        let scope = self.ModuleRef(import_scope);

        self.records.ImplMap.push(rec::ImplMap {
            MappingFlags: flags,
            MemberForwarded: MemberForwarded::MethodDef(method),
            ImportName: self.strings.insert(import_name),
            ImportScope: scope,
        });
    }

    /// Adds an `AssemblyRef` row for the given assembly name, returning the row offset.
    fn AssemblyRef(&mut self, assembly_name: &str) -> AssemblyRef {
        if let Some(pos) = self.AssemblyRef.get(assembly_name) {
            return *pos;
        }

        let pos = AssemblyRef(if assembly_name == "System" {
            self.records.AssemblyRef.push_pos(rec::AssemblyRef {
                Name: self.strings.insert("mscorlib"),
                MajorVersion: 4,
                PublicKeyOrToken: self
                    .blobs
                    .insert(&[0xB7, 0x7A, 0x5C, 0x56, 0x19, 0x34, 0xE0, 0x89]),
                ..Default::default()
            })
        } else {
            self.records.AssemblyRef.push_pos(rec::AssemblyRef {
                Name: self.strings.insert(assembly_name),
                MajorVersion: 0xFF,
                MinorVersion: 0xFF,
                BuildNumber: 0xFF,
                RevisionNumber: 0xFF,
                Flags: AssemblyFlags::WindowsRuntime,
                ..Default::default()
            })
        });

        self.AssemblyRef.insert(assembly_name.to_string(), pos);
        pos
    }

    /// Adds a `TypeDef` row to the file, returning the row offset.
    pub fn TypeDef(
        &mut self,
        namespace: &str,
        name: &str,
        extends: TypeDefOrRef,
        flags: TypeAttributes,
    ) -> TypeDef {
        TypeDef(self.records.TypeDef.push_pos(rec::TypeDef {
            TypeName: self.strings.insert(name),
            TypeNamespace: self.strings.insert(namespace),
            Flags: flags,
            Extends: extends,
            FieldList: self.records.Field.len() as u32,
            MethodList: self.records.MethodDef.len() as u32,
        }))
    }

    /// Adds a `TypeRef` row to the file, returning the row offset.
    pub fn TypeRef(&mut self, namespace: &str, name: &str) -> TypeRef {
        if let Some(key) = self.TypeRef.get(namespace) {
            if let Some(pos) = key.get(name) {
                return *pos;
            }
        }

        let pos = if let Some((parent, leaf)) = name.rsplit_once('/') {
            let enclosing = self.TypeRef(namespace, parent);
            TypeRef(self.records.TypeRef.push_pos(rec::TypeRef {
                TypeName: self.strings.insert(leaf),
                TypeNamespace: self.strings.insert(""),
                ResolutionScope: ResolutionScope::TypeRef(enclosing),
            }))
        } else {
            let assembly_name = self
                .reference
                .as_ref()
                .and_then(|r| r.assembly_name(namespace, name))
                .map(str::to_string);

            let scope = if let Some(assembly_name) = assembly_name {
                ResolutionScope::AssemblyRef(self.AssemblyRef(&assembly_name))
            } else if namespace == "System" {
                ResolutionScope::AssemblyRef(self.AssemblyRef("System"))
            } else {
                ResolutionScope::Module(Module(0))
            };

            TypeRef(self.records.TypeRef.push_pos(rec::TypeRef {
                TypeName: self.strings.insert(name),
                TypeNamespace: self.strings.insert(namespace),
                ResolutionScope: scope,
            }))
        };

        self.TypeRef
            .entry(namespace.to_string())
            .or_default()
            .insert(name.to_string(), pos);

        pos
    }

    pub fn TypeSpec(&mut self, namespace: &str, name: &str, generics: &[Type]) -> TypeSpec {
        debug_assert!(!generics.is_empty());
        // Generic type references use the backtick-suffix convention per ECMA-335 §II.10.7.2.
        let name = format!("{name}`{}", generics.len());
        let type_ref = self.TypeRef(namespace, &name);

        let mut buffer = vec![];
        buffer.push(ELEMENT_TYPE_GENERICINST);
        buffer.push(ELEMENT_TYPE_CLASS);
        buffer.write_compressed(TypeDefOrRef::TypeRef(type_ref).encode() as usize);
        buffer.write_compressed(generics.len());

        for ty in generics {
            self.Type(ty, &mut buffer);
        }

        let signature = self.blobs.insert(&buffer);

        if let Some(pos) = self.TypeSpec.get(&signature) {
            return *pos;
        }

        let pos = TypeSpec(self.records.TypeSpec.push_pos(rec::TypeSpec {
            Signature: signature,
        }));
        self.TypeSpec.insert(signature, pos);
        pos
    }

    /// Adds a `Field` row to the file, returning the row offset.
    pub fn Field(&mut self, name: &str, ty: &Type, flags: FieldAttributes) -> Field {
        let signature = self.FieldSig(ty);

        Field(self.records.Field.push_pos(rec::Field {
            Name: self.strings.insert(name),
            Flags: flags,
            Signature: signature,
        }))
    }

    /// Adds a `MethodDef` row to the file, returning the row offset.
    pub fn MethodDef(
        &mut self,
        name: &str,
        signature: &Signature,
        flags: MethodAttributes,
        impl_flags: MethodImplAttributes,
    ) -> MethodDef {
        let signature = self.MethodDefSig(signature);

        MethodDef(self.records.MethodDef.push_pos(rec::MethodDef {
            RVA: 0,
            ImplFlags: impl_flags,
            Flags: flags,
            Name: self.strings.insert(name),
            Signature: signature,
            ParamList: self.records.Param.len() as u32,
        }))
    }

    pub fn MemberRef(
        &mut self,
        name: &str,
        signature: &Signature,
        parent: MemberRefParent,
    ) -> MemberRef {
        let signature = self.MethodDefSig(signature);

        let record = rec::MemberRef {
            Name: self.strings.insert(name),
            Signature: signature,
            Parent: parent,
        };

        if let Some(pos) = self.MemberRef.get(&record) {
            return *pos;
        }

        let pos = MemberRef(self.records.MemberRef.push_pos(record));
        self.MemberRef.insert(record, pos);
        pos
    }

    /// Adds a `Param` row to the file, returning the row offset.
    pub fn Param(&mut self, name: &str, sequence: u16, flags: ParamAttributes) -> Param {
        Param(self.records.Param.push_pos(rec::Param {
            Flags: flags,
            Sequence: sequence,
            Name: self.strings.insert(name),
        }))
    }

    /// Adds a `Property` row to the file, returning the row offset.
    pub fn Property(&mut self, name: &str, ty: &Type) -> Property {
        let signature = self.PropertySig(ty);

        Property(self.records.Property.push_pos(rec::Property {
            Flags: 0,
            Name: self.strings.insert(name),
            Type: signature,
        }))
    }

    /// Adds a `PropertyMap` row associating a type with its first property.
    pub fn PropertyMap(&mut self, parent: TypeDef, property_list: Property) -> PropertyMap {
        PropertyMap(self.records.PropertyMap.push_pos(rec::PropertyMap {
            Parent: parent,
            PropertyList: property_list,
        }))
    }

    /// Adds an `Event` row to the file, returning the row offset. `ty` is the event's
    /// handler delegate type.
    pub fn Event(&mut self, name: &str, ty: &Type) -> Event {
        let Type::ClassName(ty) = ty else {
            panic!("invalid event type");
        };
        let event_type = TypeDefOrRef::TypeRef(self.TypeRef(&ty.namespace, &ty.name));

        Event(self.records.Event.push_pos(rec::Event {
            Flags: 0,
            Name: self.strings.insert(name),
            EventType: event_type,
        }))
    }

    /// Adds an `EventMap` row associating a type with its first event.
    pub fn EventMap(&mut self, parent: TypeDef, event_list: Event) -> EventMap {
        EventMap(self.records.EventMap.push_pos(rec::EventMap {
            Parent: parent,
            EventList: event_list,
        }))
    }

    /// Adds a `MethodSemantics` row linking an accessor method to a property.
    pub fn MethodSemantics(
        &mut self,
        semantics: u16,
        method: MethodDef,
        association: HasSemantics,
    ) -> MethodSemantics {
        MethodSemantics(self.records.MethodSemantics.push_pos(rec::MethodSemantics {
            Semantics: semantics,
            Method: method,
            Association: association,
        }))
    }

    /// Adds an `Attribute` row to the file. This is a sorted table so the row offset is not yet available.
    pub fn Attribute(
        &mut self,
        parent: HasAttribute,
        ty: AttributeType,
        value: &[(String, Value)],
    ) {
        let value = self.AttributeValue(value);

        self.Attribute
            .entry(parent)
            .or_default()
            .push(rec::Attribute {
                Parent: parent,
                Type: ty,
                Value: value,
            });
    }

    pub fn Constant(&mut self, parent: HasConstant, value: &Value) {
        let ty = value.ty().code();
        let value = self.ConstantValue(value);

        self.Constant.insert(
            parent,
            rec::Constant {
                Parent: parent,
                Type: ty,
                Value: value,
            },
        );
    }

    pub fn GenericParam(
        &mut self,
        name: &str,
        owner: TypeOrMethodDef,
        number: u16,
        flags: GenericParamAttributes,
    ) {
        self.GenericParam
            .entry(owner)
            .or_default()
            .push(rec::GenericParam {
                Name: self.strings.insert(name),
                Number: number,
                Owner: owner,
                Flags: flags,
            });
    }

    pub fn ClassLayout(&mut self, parent: TypeDef, packing_size: u16, class_size: u32) {
        self.records.ClassLayout.push(rec::ClassLayout {
            PackingSize: packing_size,
            ClassSize: class_size,
            Parent: parent.0,
        });
    }

    pub fn FieldLayout(&mut self, field: Field, offset: u32) {
        self.records.FieldLayout.push(rec::FieldLayout {
            Offset: offset,
            Field: field.0,
        });
    }

    pub fn NestedClass(&mut self, inner: TypeDef, outer: TypeDef) {
        debug_assert!(inner.0 > outer.0);

        self.records.NestedClass.push(rec::NestedClass {
            NestedClass: inner.0,
            EnclosingClass: outer.0,
        });
    }

    pub fn InterfaceImpl(&mut self, class: TypeDef, interface: &Type) -> InterfaceImpl {
        let Type::ClassName(interface) = interface else {
            panic!("invalid interface type");
        };

        let interface = if interface.generics.is_empty() {
            TypeDefOrRef::TypeRef(self.TypeRef(&interface.namespace, &interface.name))
        } else {
            TypeDefOrRef::TypeSpec(self.TypeSpec(
                &interface.namespace,
                &interface.name,
                &interface.generics,
            ))
        };

        InterfaceImpl(self.records.InterfaceImpl.push_pos(rec::InterfaceImpl {
            Class: class,
            Interface: interface,
        }))
    }

    /// Encodes the `Type` in the buffer. Any required `TypeRef` rows will be added to the file, returning the blob offset.
    fn Type(&mut self, ty: &Type, buffer: &mut Vec<u8>) {
        match ty {
            Type::Void => buffer.push(ELEMENT_TYPE_VOID),
            Type::Bool => buffer.push(ELEMENT_TYPE_BOOLEAN),
            Type::Char => buffer.push(ELEMENT_TYPE_CHAR),
            Type::I8 => buffer.push(ELEMENT_TYPE_I1),
            Type::U8 => buffer.push(ELEMENT_TYPE_U1),
            Type::I16 => buffer.push(ELEMENT_TYPE_I2),
            Type::U16 => buffer.push(ELEMENT_TYPE_U2),
            Type::I32 => buffer.push(ELEMENT_TYPE_I4),
            Type::U32 => buffer.push(ELEMENT_TYPE_U4),
            Type::I64 => buffer.push(ELEMENT_TYPE_I8),
            Type::U64 => buffer.push(ELEMENT_TYPE_U8),
            Type::F32 => buffer.push(ELEMENT_TYPE_R4),
            Type::F64 => buffer.push(ELEMENT_TYPE_R8),
            Type::ISize => buffer.push(ELEMENT_TYPE_I),
            Type::USize => buffer.push(ELEMENT_TYPE_U),
            Type::String => buffer.push(ELEMENT_TYPE_STRING),
            Type::Object => buffer.push(ELEMENT_TYPE_OBJECT),

            Type::Array(ty) => {
                buffer.push(ELEMENT_TYPE_SZARRAY);
                self.Type(ty, buffer);
            }

            Type::RefMut(ty) => {
                buffer.push(ELEMENT_TYPE_BYREF);
                self.Type(ty, buffer);
            }

            Type::RefConst(ty) => {
                buffer.write_compressed(ELEMENT_TYPE_CMOD_REQD as usize);
                let pos = self.TypeRef("System.Runtime.CompilerServices", "IsConst");
                buffer.write_compressed(TypeDefOrRef::TypeRef(pos).encode() as usize);
                buffer.push(ELEMENT_TYPE_BYREF);
                self.Type(ty, buffer);
            }

            Type::PtrMut(ty, pointers) => {
                for _ in 0..*pointers {
                    buffer.write_compressed(ELEMENT_TYPE_PTR as usize);
                }

                self.Type(ty, buffer);
            }

            Type::PtrConst(ty, pointers) => {
                buffer.write_compressed(ELEMENT_TYPE_CMOD_REQD as usize);
                let pos = self.TypeRef("System.Runtime.CompilerServices", "IsConst");
                buffer.write_compressed(TypeDefOrRef::TypeRef(pos).encode() as usize);

                for _ in 0..*pointers {
                    buffer.write_compressed(ELEMENT_TYPE_PTR as usize);
                }

                self.Type(ty, buffer);
            }

            Type::ArrayFixed(ty, len) => {
                // See II.23.2.13 ArrayShape
                buffer.push(ELEMENT_TYPE_ARRAY);
                self.Type(ty, buffer);
                buffer.write_compressed(1); // rank
                buffer.write_compressed(1); // num_sizes
                buffer.write_compressed(*len); // size
                buffer.write_compressed(0); // num_lo_bounds
            }

            Type::Generic(_, number) => {
                buffer.push(ELEMENT_TYPE_VAR);
                buffer.write_compressed((*number).into());
            }

            Type::ClassName(ty) => {
                self.TypeName(false, &ty.namespace, &ty.name, &ty.generics, buffer);
            }
            Type::ValueName(ty) => {
                self.TypeName(true, &ty.namespace, &ty.name, &ty.generics, buffer);
            }
        }
    }

    fn TypeName(
        &mut self,
        is_value_type: bool,
        namespace: &str,
        name: &str,
        generics: &[Type],
        buffer: &mut Vec<u8>,
    ) {
        let pos = if !generics.is_empty() {
            buffer.push(ELEMENT_TYPE_GENERICINST);
            let name = format!("{name}`{}", generics.len());
            self.TypeRef(namespace, &name)
        } else {
            self.TypeRef(namespace, name)
        };

        buffer.push(if is_value_type {
            ELEMENT_TYPE_VALUETYPE
        } else {
            ELEMENT_TYPE_CLASS
        });
        buffer.write_compressed(TypeDefOrRef::TypeRef(pos).encode() as usize);

        if !generics.is_empty() {
            buffer.write_compressed(generics.len());

            for ty in generics {
                self.Type(ty, buffer);
            }
        }
    }

    /// Writes the `Type` into a `FileSig` buffer and stores it in the file, returning the blob offset.
    fn FieldSig(&mut self, ty: &Type) -> BlobId {
        let mut buffer = vec![0x6]; // FIELD
        self.Type(ty, &mut buffer);
        self.blobs.insert(&buffer)
    }

    fn PropertySig(&mut self, ty: &Type) -> BlobId {
        let mut buffer = vec![0x28]; // HASTHIS | PROPERTY
        buffer.write_compressed(0); // parameter count
        self.Type(ty, &mut buffer);
        self.blobs.insert(&buffer)
    }

    /// Writes the method signature into a `MethodDefSig` buffer and stores it in the file, returning the blob offset.
    fn MethodDefSig(&mut self, signature: &Signature) -> BlobId {
        let mut buffer = vec![signature.flags.0];
        buffer.write_compressed(signature.types.len());
        self.Type(&signature.return_type, &mut buffer);

        for ty in &signature.types {
            self.Type(ty, &mut buffer);
        }

        self.blobs.insert(&buffer)
    }

    fn ConstantValue(&mut self, value: &Value) -> BlobId {
        let mut buffer = vec![];
        buffer.write_value(value);
        self.blobs.insert(&buffer)
    }

    fn AttributeValue(&mut self, values: &[(String, Value)]) -> BlobId {
        let mut buffer = vec![];
        buffer.write_u16(1); // prolog

        let mut count = 0;

        for (name, value) in values {
            if name.is_empty() {
                count += 1;
                buffer.write_value(value);
            } else {
                break;
            }
        }

        buffer.write_u16((values.len() - count).try_into().unwrap());

        for (name, value) in &values[count..] {
            buffer.push(0x53); // field=0x53 property=0x54

            if let Value::EnumValue(tn, _) = value {
                // SERIALIZATION_TYPE_ENUM (ECMA-335 §II.23.1.16): 0x55 followed by
                // a SerString of the fully-qualified enum type name.
                buffer.push(0x55);
                let enum_name = if tn.namespace.is_empty() {
                    tn.name.clone()
                } else {
                    format!("{}.{}", tn.namespace, tn.name)
                };
                buffer.write_compressed(enum_name.len());
                buffer.extend_from_slice(enum_name.as_bytes());
            } else {
                buffer.push(value.ty().code());
            }

            buffer.write_compressed(name.len());
            buffer.extend_from_slice(name.as_bytes());
            buffer.write_value(value);
        }

        self.blobs.insert(&buffer)
    }
}
