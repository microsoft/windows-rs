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
    ty: StructFieldType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ValueModel {
    Enum(StructFieldType),
    Struct(StructModel),
}

type ValueModels = BTreeMap<(String, String), ValueModel>;

#[derive(Clone, Debug, Eq, PartialEq)]
enum StructFieldType {
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
                        ty: StructFieldType::from_old(field.ty())?,
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
                        ty: StructFieldType::from_new(
                            database,
                            file,
                            field.signature().ok()?.kind,
                        )?,
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

impl StructFieldType {
    fn from_old(ty: windows_metadata::Type) -> Option<Self> {
        Some(match ty {
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

    fn from_new(database: &new::Database, file: new::FileId, ty: new::TypeKind) -> Option<Self> {
        Some(match ty {
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
                        .map(|argument| Self::from_new(database, file, argument.kind))
                        .collect::<Option<_>>()?,
                }
            }
            _ => return None,
        })
    }

    fn write(&self, current_namespace: &str) -> Option<TokenStream> {
        Some(match self {
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
                        ValueModel::Enum(underlying) => Some(format!(
                            "enum({namespace}.{name};{})",
                            underlying.runtime_signature(values)?
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
            Self::String => Some(false),
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
    index
        .iter()
        .filter(|(_, _, definition)| definition.flags().contains(TypeAttributes::WindowsRuntime))
        .filter_map(|(namespace, name, definition)| {
            let model = match definition.category() {
                windows_metadata::reader::TypeCategory::Enum => {
                    ValueModel::Enum(StructFieldType::from_old(definition.underlying_type()?)?)
                }
                windows_metadata::reader::TypeCategory::Struct => {
                    ValueModel::Struct(StructModel::from_old(definition)?)
                }
                _ => return None,
            };
            Some(((namespace.to_string(), name.to_string()), model))
        })
        .collect()
}

fn value_models_from_new(database: &new::Database) -> ValueModels {
    database
        .definitions()
        .filter(|definition| matches!(definition.is_windows_runtime(), Ok(true)))
        .filter_map(|definition| {
            let namespace = definition.namespace().ok()?.to_string();
            let name = definition.name().ok()?.to_string();
            let model = match definition.category().ok()? {
                new::TypeCategory::Enum => ValueModel::Enum(
                    definition
                        .fields()
                        .ok()?
                        .find_map(|field| {
                            if field.constant().ok()?.is_none() {
                                Some(field.signature().ok()?.kind)
                            } else {
                                None
                            }
                        })
                        .and_then(|ty| {
                            StructFieldType::from_new(database, definition.entity().file(), ty)
                        })?,
                ),
                new::TypeCategory::Struct => {
                    ValueModel::Struct(StructModel::from_new(database, definition)?)
                }
                _ => return None,
            };
            Some(((namespace, name), model))
        })
        .collect()
}

#[test]
fn struct_value_models_and_output_match() {
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
    assert!(matched > 3_000, "only matched {matched} scalar structs");

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
    for (namespace, namespace_types) in reader.iter() {
        for items in namespace_types.values() {
            for item in items {
                let Type::Struct(item) = item else {
                    continue;
                };
                total += 1;
                let Some(old_model) = StructModel::from_old(item.def) else {
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
                    continue;
                };
                let Some(model_output) = new_model.write(&new_values) else {
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
    assert!(
        rendered > 120 && total > 100,
        "only rendered {rendered} of {total} WinRT structs"
    );
}
