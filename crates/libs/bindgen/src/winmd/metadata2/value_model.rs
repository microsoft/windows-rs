use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct StructModel {
    namespace: String,
    name: String,
    fields: Vec<StructFieldModel>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StructFieldModel {
    name: String,
    ty: ModelType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ValueModel {
    Enum(EnumModel),
    Struct(StructModel),
}

type ValueModels = BTreeMap<(String, String), ValueModel>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct EnumModel {
    namespace: String,
    name: String,
    underlying: ModelType,
    fields: Vec<EnumFieldModel>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EnumFieldModel {
    name: String,
    value: IntegerValue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IntegerValue {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    ISize(i64),
    USize(u64),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum ModelType {
    Void,
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
    String,
    Object,
    Generic(u32),
    Vector(Box<Self>),
    Named {
        value_type: bool,
        namespace: String,
        name: String,
        arguments: Vec<Self>,
    },
}

impl StructModel {
    fn from_old(definition: windows_metadata::reader::TypeDef<'_>) -> Option<Self> {
        Some(Self {
            namespace: definition.namespace().to_string(),
            name: definition.name().to_string(),
            fields: definition
                .fields()
                .map(|field| {
                    Some(StructFieldModel {
                        name: field.name().to_string(),
                        ty: ModelType::from_old(field.ty())?,
                    })
                })
                .collect::<Option<_>>()?,
        })
    }

    fn from_new(database: &new::Database, definition: new::TypeDefinition<'_>) -> Option<Self> {
        let file = definition.entity().file();
        Some(Self {
            namespace: definition.namespace().ok()?.to_string(),
            name: definition.name().ok()?.to_string(),
            fields: definition
                .fields()
                .ok()?
                .map(|field| {
                    Some(StructFieldModel {
                        name: field.name().ok()?.to_string(),
                        ty: ModelType::from_new(database, file, field.signature().ok()?.kind)?,
                    })
                })
                .collect::<Option<_>>()?,
        })
    }

    fn write(&self, values: &ValueModels) -> Option<TokenStream> {
        let name = to_ident(&self.name);
        let fields: Vec<_> = self
            .fields
            .iter()
            .map(|field| {
                let name = to_ident(&field.name);
                let ty = field.ty.write(&self.namespace)?;
                Some(quote! { pub #name: #ty, })
            })
            .collect::<Option<_>>()?;
        let signature = Literal::byte_string(self.runtime_signature(values)?.as_bytes());
        let runtime_name =
            Literal::byte_string(format!("{}.{}", self.namespace, self.name).as_bytes());
        let is_copyable = self
            .fields
            .iter()
            .all(|field| field.ty.is_copyable(values) == Some(true));
        let is_eq = self
            .fields
            .iter()
            .all(|field| field.ty.is_eq(values) == Some(true));
        let derive = if is_copyable && is_eq {
            quote! { #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)] }
        } else if is_copyable {
            quote! { #[derive(Clone, Copy, Debug, Default, PartialEq)] }
        } else if is_eq {
            quote! { #[derive(Clone, Debug, Default, Eq, PartialEq)] }
        } else {
            quote! { #[derive(Clone, Debug, Default, PartialEq)] }
        };
        let type_kind = if is_copyable {
            quote! { CopyType }
        } else {
            quote! { CloneType }
        };

        Some(quote! {
            #[repr(C)]
            #derive
            pub struct #name {
                #(#fields)*
            }
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::#type_kind;
            }
            impl windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(#signature);
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(#runtime_name);
            }
        })
    }

    fn runtime_signature(&self, values: &ValueModels) -> Option<String> {
        let mut signature = format!("struct({}.{}", self.namespace, self.name);
        for field in &self.fields {
            signature.push(';');
            signature.push_str(&field.ty.runtime_signature(values)?);
        }
        signature.push(')');
        Some(signature)
    }
}

impl EnumModel {
    fn from_old(definition: windows_metadata::reader::TypeDef<'_>) -> Option<Self> {
        Some(Self {
            namespace: definition.namespace().to_string(),
            name: definition.name().to_string(),
            underlying: ModelType::from_old(definition.underlying_type()?)?,
            fields: definition
                .fields()
                .filter(|field| field.flags().contains(FieldAttributes::Literal))
                .map(|field| {
                    Some(EnumFieldModel {
                        name: field.name().to_string(),
                        value: IntegerValue::from_old(field.constant()?.value())?,
                    })
                })
                .collect::<Option<_>>()?,
        })
    }

    fn from_new(database: &new::Database, definition: new::TypeDefinition<'_>) -> Option<Self> {
        let file = definition.entity().file();
        let mut underlying = None;
        let mut fields = Vec::new();
        for field in definition.fields().ok()? {
            if field.is_literal().ok()? {
                fields.push(EnumFieldModel {
                    name: field.name().ok()?.to_string(),
                    value: IntegerValue::from_new(field.constant().ok()??.value().ok()?)?,
                });
            } else if field.constant().ok()?.is_none() {
                underlying = ModelType::from_new(database, file, field.signature().ok()?.kind);
            }
        }
        Some(Self {
            namespace: definition.namespace().ok()?.to_string(),
            name: definition.name().ok()?.to_string(),
            underlying: underlying?,
            fields,
        })
    }

    fn write(&self, values: &ValueModels) -> Option<TokenStream> {
        let name = to_ident(&self.name);
        let underlying = self.underlying.write(&self.namespace)?;
        let fields = self.fields.iter().map(|field| {
            let field_name = to_ident(&field.name);
            let value = field.value.write();
            quote! { pub const #field_name: Self = Self(#value); }
        });
        let fields = if self.fields.is_empty() {
            quote! {}
        } else {
            quote! { impl #name { #(#fields)* } }
        };
        let flags = if self.underlying == ModelType::U32 {
            write_enum_flags(&name)
        } else {
            quote! {}
        };
        let signature = Literal::byte_string(
            format!(
                "enum({}.{};{})",
                self.namespace,
                self.name,
                self.underlying.runtime_signature(values)?
            )
            .as_bytes(),
        );
        let runtime_name =
            Literal::byte_string(format!("{}.{}", self.namespace, self.name).as_bytes());

        Some(quote! {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct #name(pub #underlying);
            #fields
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(#signature);
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(#runtime_name);
            }
            #flags
        })
    }
}

impl IntegerValue {
    fn from_old(value: Value) -> Option<Self> {
        Some(match value {
            Value::I8(value) => Self::I8(value),
            Value::U8(value) => Self::U8(value),
            Value::I16(value) => Self::I16(value),
            Value::U16(value) => Self::U16(value),
            Value::I32(value) => Self::I32(value),
            Value::U32(value) => Self::U32(value),
            Value::I64(value) => Self::I64(value),
            Value::U64(value) => Self::U64(value),
            Value::ISize(value) => Self::ISize(value),
            Value::USize(value) => Self::USize(value),
            _ => return None,
        })
    }

    fn from_new(value: new::ConstantValue) -> Option<Self> {
        Some(match value {
            new::ConstantValue::I8(value) => Self::I8(value),
            new::ConstantValue::U8(value) => Self::U8(value),
            new::ConstantValue::I16(value) => Self::I16(value),
            new::ConstantValue::U16(value) => Self::U16(value),
            new::ConstantValue::I32(value) => Self::I32(value),
            new::ConstantValue::U32(value) => Self::U32(value),
            new::ConstantValue::I64(value) => Self::I64(value),
            new::ConstantValue::U64(value) => Self::U64(value),
            new::ConstantValue::ISize(value) => Self::ISize(value),
            new::ConstantValue::USize(value) => Self::USize(value),
            _ => return None,
        })
    }

    fn write(self) -> TokenStream {
        let value = match self {
            Self::I8(value) => Literal::i8_unsuffixed(value),
            Self::U8(value) => Literal::u8_unsuffixed(value),
            Self::I16(value) => Literal::i16_unsuffixed(value),
            Self::U16(value) => Literal::u16_unsuffixed(value),
            Self::I32(value) => Literal::i32_unsuffixed(value),
            Self::U32(value) => Literal::u32_unsuffixed(value),
            Self::I64(value) | Self::ISize(value) => Literal::i64_unsuffixed(value),
            Self::U64(value) | Self::USize(value) => Literal::u64_unsuffixed(value),
        };
        quote! { #value }
    }
}

impl ModelType {
    pub(super) fn from_old(ty: windows_metadata::Type) -> Option<Self> {
        Some(match ty {
            windows_metadata::Type::Void => Self::Void,
            windows_metadata::Type::Bool => Self::Boolean,
            windows_metadata::Type::Char => Self::Char,
            windows_metadata::Type::I8 => Self::I8,
            windows_metadata::Type::U8 => Self::U8,
            windows_metadata::Type::I16 => Self::I16,
            windows_metadata::Type::U16 => Self::U16,
            windows_metadata::Type::I32 => Self::I32,
            windows_metadata::Type::U32 => Self::U32,
            windows_metadata::Type::I64 => Self::I64,
            windows_metadata::Type::U64 => Self::U64,
            windows_metadata::Type::F32 => Self::F32,
            windows_metadata::Type::F64 => Self::F64,
            windows_metadata::Type::String => Self::String,
            windows_metadata::Type::Object => Self::Object,
            windows_metadata::Type::Generic(_, index) => Self::Generic(index.into()),
            windows_metadata::Type::Array(element) => {
                Self::Vector(Box::new(Self::from_old(*element)?))
            }
            windows_metadata::Type::ValueName(name) => Self::Named {
                value_type: true,
                namespace: name.namespace,
                name: trim_generic_arity(&name.name).to_string(),
                arguments: name
                    .generics
                    .into_iter()
                    .map(Self::from_old)
                    .collect::<Option<_>>()?,
            },
            windows_metadata::Type::ClassName(name) => Self::Named {
                value_type: false,
                namespace: name.namespace,
                name: trim_generic_arity(&name.name).to_string(),
                arguments: name
                    .generics
                    .into_iter()
                    .map(Self::from_old)
                    .collect::<Option<_>>()?,
            },
            _ => return None,
        })
    }

    pub(super) fn from_new(
        database: &new::Database,
        file: new::FileId,
        ty: new::TypeKind,
    ) -> Option<Self> {
        Some(match ty {
            new::TypeKind::Void => Self::Void,
            new::TypeKind::Boolean => Self::Boolean,
            new::TypeKind::Char => Self::Char,
            new::TypeKind::I8 => Self::I8,
            new::TypeKind::U8 => Self::U8,
            new::TypeKind::I16 => Self::I16,
            new::TypeKind::U16 => Self::U16,
            new::TypeKind::I32 => Self::I32,
            new::TypeKind::U32 => Self::U32,
            new::TypeKind::I64 => Self::I64,
            new::TypeKind::U64 => Self::U64,
            new::TypeKind::F32 => Self::F32,
            new::TypeKind::F64 => Self::F64,
            new::TypeKind::String => Self::String,
            new::TypeKind::Object => Self::Object,
            new::TypeKind::GenericType(index) => Self::Generic(index),
            new::TypeKind::Vector(element) => {
                Self::Vector(Box::new(Self::from_new_type(database, file, &element)?))
            }
            new::TypeKind::Value(ty) => {
                let (namespace, name) = database.type_name(file, ty).ok()??;
                Self::Named {
                    value_type: true,
                    namespace: namespace.to_string(),
                    name: trim_generic_arity(name).to_string(),
                    arguments: Vec::new(),
                }
            }
            new::TypeKind::Class(ty) => {
                let (namespace, name) = database.type_name(file, ty).ok()??;
                Self::Named {
                    value_type: false,
                    namespace: namespace.to_string(),
                    name: trim_generic_arity(name).to_string(),
                    arguments: Vec::new(),
                }
            }
            new::TypeKind::GenericInstance {
                value_type,
                ty,
                arguments,
            } => {
                let (namespace, name) = database.type_name(file, ty).ok()??;
                Self::Named {
                    value_type,
                    namespace: namespace.to_string(),
                    name: trim_generic_arity(name).to_string(),
                    arguments: arguments
                        .into_iter()
                        .map(|argument| Self::from_new_type(database, file, &argument))
                        .collect::<Option<_>>()?,
                }
            }
            _ => return None,
        })
    }

    pub(super) fn from_new_type(
        database: &new::Database,
        file: new::FileId,
        ty: &new::Type,
    ) -> Option<Self> {
        ty.modifiers
            .is_empty()
            .then(|| Self::from_new(database, file, ty.kind.clone()))
            .flatten()
    }

    fn write(&self, current_namespace: &str) -> Option<TokenStream> {
        Some(match self {
            Self::Void | Self::Object | Self::Generic(_) | Self::Vector(_) => return None,
            Self::Boolean => quote! { bool },
            Self::Char => quote! { u16 },
            Self::I8 => quote! { i8 },
            Self::U8 => quote! { u8 },
            Self::I16 => quote! { i16 },
            Self::U16 => quote! { u16 },
            Self::I32 => quote! { i32 },
            Self::U32 => quote! { u32 },
            Self::I64 => quote! { i64 },
            Self::U64 => quote! { u64 },
            Self::F32 => quote! { f32 },
            Self::F64 => quote! { f64 },
            Self::String => quote! { windows_core::HSTRING },
            Self::Named {
                value_type,
                namespace,
                name,
                arguments,
            } => {
                if namespace == "System" && name == "Guid" && arguments.is_empty() {
                    quote! { windows_core::GUID }
                } else {
                    let name = to_ident(name);
                    let namespace = write_namespace(current_namespace, namespace);
                    let arguments = arguments
                        .iter()
                        .map(|argument| argument.write(current_namespace))
                        .collect::<Option<Vec<_>>>()?;
                    let name = if arguments.is_empty() {
                        quote! { #namespace #name }
                    } else {
                        quote! { #namespace #name < #(#arguments),* > }
                    };
                    if *value_type {
                        name
                    } else {
                        quote! { Option<#name> }
                    }
                }
            }
        })
    }

    fn runtime_signature(&self, values: &ValueModels) -> Option<String> {
        Some(
            match self {
                Self::Void | Self::Generic(_) | Self::Vector(_) => return None,
                Self::Boolean => "b1",
                Self::Char => "c2",
                Self::I8 => "i1",
                Self::U8 => "u1",
                Self::I16 => "i2",
                Self::U16 => "u2",
                Self::I32 => "i4",
                Self::U32 => "u4",
                Self::I64 => "i8",
                Self::U64 => "u8",
                Self::F32 => "f4",
                Self::F64 => "f8",
                Self::String => "string",
                Self::Object => "cinterface(IInspectable)",
                Self::Named {
                    value_type: true,
                    namespace,
                    name,
                    arguments,
                } if namespace == "System" && name == "Guid" && arguments.is_empty() => "g16",
                Self::Named {
                    value_type: true,
                    namespace,
                    name,
                    arguments,
                } if arguments.is_empty() => {
                    return match values.get(&(namespace.clone(), name.clone()))? {
                        ValueModel::Enum(model) => Some(format!(
                            "enum({namespace}.{name};{})",
                            model.underlying.runtime_signature(values)?
                        )),
                        ValueModel::Struct(model) => model.runtime_signature(values),
                    };
                }
                Self::Named { .. } => return None,
            }
            .to_string(),
        )
    }

    fn is_copyable(&self, values: &ValueModels) -> Option<bool> {
        match self {
            Self::Void | Self::Generic(_) | Self::Vector(_) => None,
            Self::String | Self::Object => Some(false),
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
            } if namespace == "System" && name == "Guid" && arguments.is_empty() => Some(true),
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
            } if arguments.is_empty() => match values.get(&(namespace.clone(), name.clone()))? {
                ValueModel::Enum(_) => Some(true),
                ValueModel::Struct(model) => Some(
                    model
                        .fields
                        .iter()
                        .all(|field| field.ty.is_copyable(values) == Some(true)),
                ),
            },
            Self::Named { .. } => Some(false),
            _ => Some(true),
        }
    }

    fn is_eq(&self, values: &ValueModels) -> Option<bool> {
        match self {
            Self::Void | Self::Generic(_) | Self::Vector(_) => None,
            Self::F32 | Self::F64 => Some(false),
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
            } if arguments.is_empty() && !(namespace == "System" && name == "Guid") => {
                match values.get(&(namespace.clone(), name.clone()))? {
                    ValueModel::Enum(_) => Some(true),
                    ValueModel::Struct(model) => Some(
                        model
                            .fields
                            .iter()
                            .all(|field| field.ty.is_eq(values) == Some(true)),
                    ),
                }
            }
            _ => Some(true),
        }
    }
}

fn trim_generic_arity(name: &str) -> &str {
    name.split_once('`').map_or(name, |(name, _)| name)
}

fn write_namespace(current: &str, target: &str) -> TokenStream {
    if target.is_empty() || target == current {
        return quote! {};
    }

    let mut current = current.split('.').peekable();
    let mut target = target.split('.').peekable();
    while current.peek() == target.peek() {
        current.next();
        target.next();
    }

    let mut path = String::new();
    for _ in current {
        path.push_str("super::");
    }
    for part in target {
        path.push_str(part);
        path.push_str("::");
    }
    path.parse().unwrap()
}

fn value_models_from_old(index: &windows_metadata::reader::Index) -> ValueModels {
    let mut result = ValueModels::new();
    for (namespace, name, definition) in index
        .iter()
        .filter(|(_, _, definition)| definition.flags().contains(TypeAttributes::WindowsRuntime))
    {
        let model = match definition.category() {
            windows_metadata::reader::TypeCategory::Enum => {
                ValueModel::Enum(EnumModel::from_old(definition).unwrap())
            }
            windows_metadata::reader::TypeCategory::Struct => {
                ValueModel::Struct(StructModel::from_old(definition).unwrap())
            }
            _ => continue,
        };
        assert!(
            result
                .insert((namespace.to_string(), name.to_string()), model)
                .is_none(),
            "duplicate WinRT value type {namespace}.{name}"
        );
    }
    result
}

fn value_models_from_new(database: &new::Database) -> ValueModels {
    let mut result = ValueModels::new();
    for definition in database
        .definitions()
        .filter(|definition| definition.is_windows_runtime().unwrap())
    {
        let namespace = definition.namespace().unwrap().to_string();
        let name = definition.name().unwrap().to_string();
        let model = match definition.category().unwrap() {
            new::TypeCategory::Enum => {
                ValueModel::Enum(EnumModel::from_new(database, definition).unwrap())
            }
            new::TypeCategory::Struct => {
                ValueModel::Struct(StructModel::from_new(database, definition).unwrap())
            }
            _ => continue,
        };
        assert!(
            result
                .insert((namespace.clone(), name.clone()), model)
                .is_none(),
            "duplicate WinRT value type {namespace}.{name}"
        );
    }
    result
}

#[test]
fn value_models_and_output_match() {
    let old = windows_metadata::reader::Index::new(vec![
        File::new(windows_default::WINRT.to_vec()).unwrap(),
        File::new(windows_default::WIN32.to_vec()).unwrap(),
    ]);
    let new = new::Database::new([
        new::Image::new(windows_default::WINRT).unwrap(),
        new::Image::new(windows_default::WIN32).unwrap(),
    ])
    .unwrap();
    let old_values = value_models_from_old(&old);
    let new_values = value_models_from_new(&new);
    assert_eq!(new_values.len(), old_values.len());
    for (name, old_value) in &old_values {
        assert_eq!(
            new_values.get(name),
            Some(old_value),
            "value model differs for {}.{}",
            name.0,
            name.1
        );
    }

    let mut matched = 0;
    for (namespace, _, old_definition) in old.iter() {
        if old_definition.category() != windows_metadata::reader::TypeCategory::Struct {
            continue;
        }
        let Some(old_model) = StructModel::from_old(old_definition) else {
            continue;
        };
        let definitions = new.type_definitions(namespace, old_definition.name());
        let new_model = definitions.iter().find_map(|definition| {
            StructModel::from_new(&new, new.definition(*definition).unwrap())
                .filter(|model| model == &old_model)
        });
        let new_model = new_model.unwrap_or_else(|| {
            panic!(
                "metadata2 model not found for {}.{}",
                namespace,
                old_definition.name()
            )
        });
        assert_eq!(new_model, old_model);
        matched += 1;
    }
    assert!(matched > 3_000, "only matched {matched} supported structs");

    let reader = Reader::new(vec![
        File::new(windows_default::WINRT.to_vec()).unwrap(),
        File::new(windows_default::WIN32.to_vec()).unwrap(),
    ]);
    let bindgen = Bindgen::default();
    let types = TypeMap::new();
    let references = References::default();
    let filter = Filter::default();
    let derive = Derive::new(&reader, &types, &[]);
    let event_only_delegates = HashSet::new();
    let mut rendered = 0;
    let mut total = 0;
    let mut skipped = Vec::new();
    for (namespace, namespace_types) in reader.iter() {
        for items in namespace_types.values() {
            for item in items {
                let Type::Struct(item) = item else {
                    continue;
                };
                total += 1;
                let Some(old_model) = StructModel::from_old(item.def) else {
                    skipped.push(format!("{namespace}.{}", item.def.name()));
                    continue;
                };
                let Some(new_model) = new
                    .type_definitions(namespace, item.def.name())
                    .iter()
                    .find_map(|definition| {
                        StructModel::from_new(&new, new.definition(*definition).unwrap())
                            .filter(|model| model == &old_model)
                    })
                else {
                    panic!(
                        "metadata2 model not found for {namespace}.{}",
                        item.def.name()
                    );
                };
                let Some(model_output) = new_model.write(&new_values) else {
                    skipped.push(format!("{namespace}.{}", item.def.name()));
                    continue;
                };
                let config = Config {
                    bindgen: &bindgen,
                    reader: &reader,
                    types: &types,
                    references: &references,
                    filter: &filter,
                    implement: None,
                    derive: &derive,
                    link: "windows_core",
                    namespace,
                    event_only_delegates: &event_only_delegates,
                    self_ty: None,
                    self_generics: Vec::new(),
                    prunable: std::sync::Arc::new(BTreeSet::new()),
                };
                assert_eq!(
                    model_output.to_string(),
                    item.write(&config).to_string(),
                    "{}.{}",
                    namespace,
                    item.def.name()
                );
                rendered += 1;
            }
        }
    }
    assert_eq!(skipped, ["Windows.Web.Http.HttpProgress"]);
    assert_eq!(rendered + skipped.len(), total);

    let mut rendered = 0;
    let mut total = 0;
    for (namespace, namespace_types) in reader.iter() {
        for items in namespace_types.values() {
            for item in items {
                let Type::Enum(item) = item else {
                    continue;
                };
                total += 1;
                let Some(ValueModel::Enum(model)) =
                    new_values.get(&(namespace.to_string(), item.def.name().to_string()))
                else {
                    panic!(
                        "metadata2 enum model not found for {namespace}.{}",
                        item.def.name()
                    );
                };
                let config = Config {
                    bindgen: &bindgen,
                    reader: &reader,
                    types: &types,
                    references: &references,
                    filter: &filter,
                    implement: None,
                    derive: &derive,
                    link: "windows_core",
                    namespace,
                    event_only_delegates: &event_only_delegates,
                    self_ty: None,
                    self_generics: Vec::new(),
                    prunable: std::sync::Arc::new(BTreeSet::new()),
                };
                assert_eq!(
                    model.write(&new_values).unwrap().to_string(),
                    item.write(&config).to_string(),
                    "{}.{}",
                    namespace,
                    item.def.name()
                );
                rendered += 1;
            }
        }
    }
    assert_eq!(rendered, total);
}
