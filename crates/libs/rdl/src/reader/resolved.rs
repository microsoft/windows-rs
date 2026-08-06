use super::*;

pub struct ResolvedModel<'a> {
    pub items: Vec<ResolvedItem<'a>>,
}

pub struct ResolvedItem<'a> {
    pub id: usize,
    pub file: &'a File,
    pub item: &'a Item,
    pub kind: ResolvedItemKind,
}

pub enum ResolvedItemKind {
    Attribute {
        constructors: Vec<ResolvedBareSignature>,
    },
    Class {
        interfaces: Vec<ResolvedClassInterface>,
    },
    Interface {
        requires: Vec<ResolvedTypeRef>,
        methods: Vec<ResolvedMethod>,
        properties: Vec<ResolvedProperty>,
    },
    Other,
}

pub struct ResolvedBareSignature {
    pub span: Span,
    pub types: Vec<metadata::Type>,
}

pub struct ResolvedClassInterface {
    pub span: Span,
    pub ty: metadata::Type,
}

pub struct ResolvedTypeRef {
    pub span: Span,
    pub ty: metadata::Type,
}

pub struct ResolvedMethod {
    pub name: String,
    pub span: Span,
    pub signature: ResolvedSignature,
}

#[derive(PartialEq)]
pub struct ResolvedSignature {
    pub receiver: bool,
    pub types: Vec<metadata::Type>,
}

pub struct ResolvedProperty {
    pub name: String,
    pub span: Span,
    pub ty: metadata::Type,
    pub get: bool,
    pub set: bool,
}

pub fn resolve_model<'a>(
    index: &'a Index<'a>,
    reference: &metadata::reader::Index,
) -> (ResolvedModel<'a>, Vec<Error>) {
    let mut items = vec![];
    let mut diagnostics = vec![];

    for (namespace, members) in &index.namespaces {
        for variants in members
            .types
            .values()
            .chain(members.functions.values())
            .chain(members.constants.values())
        {
            for (file, item) in variants {
                let id = items.len();
                let kind = resolve_item(index, reference, file, namespace, item, &mut diagnostics);
                items.push(ResolvedItem {
                    id,
                    file,
                    item,
                    kind,
                });
            }
        }
    }

    (ResolvedModel { items }, diagnostics)
}

fn resolve_item(
    index: &Index,
    reference: &metadata::reader::Index,
    file: &File,
    namespace: &str,
    item: &Item,
    diagnostics: &mut Vec<Error>,
) -> ResolvedItemKind {
    let generics: Vec<String> = match item {
        Item::Delegate(item) => item
            .sig
            .generics
            .type_params()
            .map(|param| param.ident.to_string())
            .collect(),
        Item::Interface(item) => item
            .generics
            .type_params()
            .map(|param| param.ident.to_string())
            .collect(),
        _ => vec![],
    };
    let resolver = Resolver {
        index,
        reference,
        file,
        namespace,
        generics: &generics,
    };

    match item {
        Item::Attribute(item) => ResolvedItemKind::Attribute {
            constructors: item
                .methods
                .iter()
                .filter_map(|method| resolve_bare_signature(&resolver, method, diagnostics))
                .collect(),
        },
        Item::Callback(item) => {
            resolve_signature(&resolver, &item.sig, diagnostics);
            ResolvedItemKind::Other
        }
        Item::Class(item) => {
            if let Some(extends) = &item.extends {
                resolve_path(&resolver, extends, diagnostics);
            }
            ResolvedItemKind::Class {
                interfaces: item
                    .interfaces
                    .iter()
                    .filter_map(|interface| {
                        resolve_path(&resolver, &interface.ty, diagnostics).map(|ty| {
                            ResolvedClassInterface {
                                span: interface.ty.span(),
                                ty,
                            }
                        })
                    })
                    .collect(),
            }
        }
        Item::Const(item) => {
            resolve_type(&resolver, &item.ty, diagnostics);
            ResolvedItemKind::Other
        }
        Item::Delegate(item) => {
            resolve_signature(&resolver, &item.sig, diagnostics);
            ResolvedItemKind::Other
        }
        Item::Enum(item) => {
            if let Some(repr) = item.attrs.iter().find(|attr| attr.path().is_ident("repr"))
                && let Ok(path) = repr.parse_args::<syn::Path>()
            {
                resolve_path(&resolver, &path, diagnostics);
            }
            ResolvedItemKind::Other
        }
        Item::Fn(item) => {
            resolve_signature(&resolver, &item.sig, diagnostics);
            ResolvedItemKind::Other
        }
        Item::Interface(item) => {
            let requires = item
                .requires
                .iter()
                .filter_map(|require| {
                    resolve_path(&resolver, require, diagnostics).map(|ty| ResolvedTypeRef {
                        span: require.span(),
                        ty,
                    })
                })
                .collect();
            let mut methods = vec![];
            let mut properties = vec![];
            for member in &item.members {
                match member {
                    InterfaceMember::Method(method) => {
                        if let Some(signature) =
                            resolve_signature(&resolver, &method.sig, diagnostics)
                        {
                            methods.push(ResolvedMethod {
                                name: method.sig.ident.to_string(),
                                span: method.sig.ident.span(),
                                signature,
                            });
                        }
                    }
                    InterfaceMember::Property(property) => {
                        if let Some(ty) = resolve_type(&resolver, &property.ty, diagnostics) {
                            let get_only = property
                                .attrs
                                .iter()
                                .any(|attr| attr.path().is_ident("get"));
                            let set_only = property
                                .attrs
                                .iter()
                                .any(|attr| attr.path().is_ident("set"));
                            let (get, set) = if get_only || set_only {
                                (get_only, set_only)
                            } else {
                                (true, true)
                            };
                            properties.push(ResolvedProperty {
                                name: property.name.to_string(),
                                span: property.name.span(),
                                ty,
                                get,
                                set,
                            });
                        }
                    }
                    InterfaceMember::Event(event) => {
                        resolve_type(&resolver, &event.handler_ty, diagnostics);
                    }
                }
            }
            ResolvedItemKind::Interface {
                requires,
                methods,
                properties,
            }
        }
        Item::Struct(item) => {
            resolve_fields(&resolver, &item.fields, diagnostics);
            ResolvedItemKind::Other
        }
        Item::Typedef(item) => {
            resolve_type(&resolver, &item.ty, diagnostics);
            ResolvedItemKind::Other
        }
        Item::Union(item) => {
            resolve_fields(&resolver, &item.fields, diagnostics);
            ResolvedItemKind::Other
        }
        Item::Module(_) => unreachable!("modules are expanded before resolution"),
    }
}

fn resolve_signature(
    resolver: &Resolver,
    signature: &syn::Signature,
    diagnostics: &mut Vec<Error>,
) -> Option<ResolvedSignature> {
    let mut valid = true;
    let mut receiver = false;
    let mut types = vec![];
    for input in &signature.inputs {
        match input {
            syn::FnArg::Receiver(_) => receiver = true,
            syn::FnArg::Typed(param) => match resolver.resolve_type(&param.ty) {
                Ok(ty) => types.push(ty),
                Err(error) => {
                    diagnostics.push(error);
                    valid = false;
                }
            },
        }
    }
    if let syn::ReturnType::Type(_, ty) = &signature.output
        && let Err(error) = resolver.resolve_type(ty)
    {
        diagnostics.push(error);
        valid = false;
    }
    valid.then_some(ResolvedSignature { receiver, types })
}

fn resolve_bare_signature(
    resolver: &Resolver,
    signature: &syn::TypeBareFn,
    diagnostics: &mut Vec<Error>,
) -> Option<ResolvedBareSignature> {
    let mut valid = true;
    let mut types = vec![];
    for param in &signature.inputs {
        match resolver.resolve_type(&param.ty) {
            Ok(ty) => types.push(ty),
            Err(error) => {
                diagnostics.push(error);
                valid = false;
            }
        }
    }
    if let syn::ReturnType::Type(_, ty) = &signature.output
        && let Err(error) = resolver.resolve_type(ty)
    {
        diagnostics.push(error);
        valid = false;
    }
    valid.then_some(ResolvedBareSignature {
        span: signature.span(),
        types,
    })
}

fn resolve_fields(resolver: &Resolver, fields: &[Field], diagnostics: &mut Vec<Error>) {
    for field in fields {
        match &field.ty {
            FieldType::Type(ty) => {
                resolve_type(resolver, ty, diagnostics);
            }
            FieldType::Nested(record) => {
                resolve_fields(resolver, &record.fields, diagnostics);
            }
        }
    }
}

fn resolve_type(
    resolver: &Resolver,
    ty: &syn::Type,
    diagnostics: &mut Vec<Error>,
) -> Option<metadata::Type> {
    match resolver.resolve_type(ty) {
        Ok(ty) => Some(ty),
        Err(error) => {
            diagnostics.push(error);
            None
        }
    }
}

fn resolve_path(
    resolver: &Resolver,
    path: &syn::Path,
    diagnostics: &mut Vec<Error>,
) -> Option<metadata::Type> {
    match resolver.resolve_path(path) {
        Ok(ty) => Some(ty),
        Err(error) => {
            diagnostics.push(error);
            None
        }
    }
}
