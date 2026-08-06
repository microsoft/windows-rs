use super::*;

pub struct ResolvedModel<'a> {
    pub items: Vec<ResolvedItem<'a>>,
    pub attributes: Vec<ResolvedAttribute>,
}

pub struct ResolvedItem<'a> {
    pub id: usize,
    pub file: &'a File,
    pub namespace: &'a str,
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

pub struct ResolvedAttribute {
    pub owner: usize,
    pub site: usize,
    pub span: Span,
    pub target: AttributeTarget,
    pub value: AttributeRef,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeTarget {
    Delegate,
    Enum,
    Field,
    Interface,
    Method,
    Parameter,
    RuntimeClass,
    Struct,
    InterfaceImpl,
}

pub fn resolve_model<'a>(
    index: &'a Index<'a>,
    reference: &metadata::reader::Index,
) -> (ResolvedModel<'a>, Vec<Error>) {
    let mut items = vec![];
    let mut attributes = vec![];
    let mut diagnostics = vec![];
    {
        let mut context = ResolveContext {
            index,
            reference,
            attributes: &mut attributes,
            diagnostics: &mut diagnostics,
            next_attribute_site: 0,
        };

        for (namespace, members) in &index.namespaces {
            for variants in members
                .types
                .values()
                .chain(members.functions.values())
                .chain(members.constants.values())
            {
                for (file, item) in variants {
                    let id = items.len();
                    let kind = resolve_item(&mut context, file, namespace, item, id);
                    items.push(ResolvedItem {
                        id,
                        file,
                        namespace,
                        item,
                        kind,
                    });
                }
            }
        }
    }

    (ResolvedModel { items, attributes }, diagnostics)
}

struct ResolveContext<'a, 'input> {
    index: &'a Index<'input>,
    reference: &'a metadata::reader::Index,
    attributes: &'a mut Vec<ResolvedAttribute>,
    diagnostics: &'a mut Vec<Error>,
    next_attribute_site: usize,
}

struct ResolveOutput<'a> {
    attributes: &'a mut Vec<ResolvedAttribute>,
    diagnostics: &'a mut Vec<Error>,
    next_attribute_site: &'a mut usize,
}

fn resolve_item(
    context: &mut ResolveContext,
    file: &File,
    namespace: &str,
    item: &Item,
    owner: usize,
) -> ResolvedItemKind {
    let ResolveContext {
        index,
        reference,
        attributes,
        diagnostics,
        next_attribute_site,
    } = context;
    let mut output = ResolveOutput {
        attributes,
        diagnostics,
        next_attribute_site,
    };
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
        Item::Attribute(item) => {
            resolve_attributes(
                &resolver,
                &item.attrs,
                &[],
                AttributeTarget::RuntimeClass,
                owner,
                &mut output,
            );
            let constructors = item
                .methods
                .iter()
                .filter_map(|method| {
                    for param in &method.inputs {
                        resolve_attributes(
                            &resolver,
                            &param.attrs,
                            &["r#in", "out", "opt"],
                            AttributeTarget::Parameter,
                            owner,
                            &mut output,
                        );
                    }
                    resolve_bare_signature(&resolver, method, output.diagnostics)
                })
                .collect();
            ResolvedItemKind::Attribute { constructors }
        }
        Item::Callback(item) => {
            resolve_attributes(
                &resolver,
                &item.attrs,
                &["invoke"],
                AttributeTarget::Delegate,
                owner,
                &mut output,
            );
            resolve_wrapped_attributes(
                &resolver,
                &item.attrs,
                "invoke",
                AttributeTarget::Method,
                owner,
                &mut output,
            );
            resolve_signature_attributes(
                &resolver,
                &item.sig,
                &item.return_attrs,
                owner,
                &mut output,
            );
            resolve_signature(&resolver, &item.sig, output.diagnostics);
            ResolvedItemKind::Other
        }
        Item::Class(item) => {
            resolve_attributes(
                &resolver,
                &item.attrs,
                &[],
                AttributeTarget::RuntimeClass,
                owner,
                &mut output,
            );
            if let Some(extends) = &item.extends {
                resolve_path(&resolver, extends, output.diagnostics);
            }
            ResolvedItemKind::Class {
                interfaces: item
                    .interfaces
                    .iter()
                    .filter_map(|interface| {
                        resolve_attributes(
                            &resolver,
                            &interface.attrs,
                            &[],
                            AttributeTarget::InterfaceImpl,
                            owner,
                            &mut output,
                        );
                        resolve_path(&resolver, &interface.ty, output.diagnostics).map(|ty| {
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
            resolve_attributes(
                &resolver,
                &item.attrs,
                &["guid", "no_guid"],
                AttributeTarget::Field,
                owner,
                &mut output,
            );
            resolve_type(&resolver, &item.ty, output.diagnostics);
            ResolvedItemKind::Other
        }
        Item::Delegate(item) => {
            resolve_attributes(
                &resolver,
                &item.attrs,
                &["guid", "no_guid", "invoke"],
                AttributeTarget::Delegate,
                owner,
                &mut output,
            );
            resolve_wrapped_attributes(
                &resolver,
                &item.attrs,
                "invoke",
                AttributeTarget::Method,
                owner,
                &mut output,
            );
            resolve_signature_attributes(
                &resolver,
                &item.sig,
                &item.return_attrs,
                owner,
                &mut output,
            );
            resolve_signature(&resolver, &item.sig, output.diagnostics);
            ResolvedItemKind::Other
        }
        Item::Enum(item) => {
            resolve_attributes(
                &resolver,
                &item.attrs,
                &["repr", "flags"],
                AttributeTarget::Enum,
                owner,
                &mut output,
            );
            for variant in &item.variants {
                resolve_attributes(
                    &resolver,
                    &variant.attrs,
                    &[],
                    AttributeTarget::Field,
                    owner,
                    &mut output,
                );
            }
            if let Some(repr) = item.attrs.iter().find(|attr| attr.path().is_ident("repr"))
                && let Ok(path) = repr.parse_args::<syn::Path>()
            {
                resolve_path(&resolver, &path, output.diagnostics);
            }
            ResolvedItemKind::Other
        }
        Item::Fn(item) => {
            resolve_attributes(
                &resolver,
                &item.attrs,
                &["library"],
                AttributeTarget::Method,
                owner,
                &mut output,
            );
            resolve_signature_attributes(
                &resolver,
                &item.sig,
                &item.return_attrs,
                owner,
                &mut output,
            );
            resolve_signature(&resolver, &item.sig, output.diagnostics);
            ResolvedItemKind::Other
        }
        Item::Interface(item) => {
            resolve_attributes(
                &resolver,
                &item.attrs,
                &["guid", "no_guid"],
                AttributeTarget::Interface,
                owner,
                &mut output,
            );
            let requires = item
                .requires
                .iter()
                .filter_map(|require| {
                    resolve_path(&resolver, require, output.diagnostics).map(|ty| ResolvedTypeRef {
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
                        resolve_attributes(
                            &resolver,
                            &method.attrs,
                            &["special"],
                            AttributeTarget::Method,
                            owner,
                            &mut output,
                        );
                        resolve_signature_attributes(
                            &resolver,
                            &method.sig,
                            &method.return_attrs,
                            owner,
                            &mut output,
                        );
                        if let Some(signature) =
                            resolve_signature(&resolver, &method.sig, output.diagnostics)
                        {
                            methods.push(ResolvedMethod {
                                name: method.sig.ident.to_string(),
                                span: method.sig.ident.span(),
                                signature,
                            });
                        }
                    }
                    InterfaceMember::Property(property) => {
                        if let Some(ty) = resolve_type(&resolver, &property.ty, output.diagnostics)
                        {
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
                        resolve_type(&resolver, &event.handler_ty, output.diagnostics);
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
            resolve_attributes(
                &resolver,
                &item.attrs,
                &["packed", "align"],
                AttributeTarget::Struct,
                owner,
                &mut output,
            );
            resolve_fields(&resolver, &item.fields, owner, &mut output);
            ResolvedItemKind::Other
        }
        Item::Typedef(item) => {
            resolve_attributes(
                &resolver,
                &item.attrs,
                &[],
                AttributeTarget::Struct,
                owner,
                &mut output,
            );
            resolve_type(&resolver, &item.ty, output.diagnostics);
            ResolvedItemKind::Other
        }
        Item::Union(item) => {
            resolve_attributes(
                &resolver,
                &item.attrs,
                &["packed", "align"],
                AttributeTarget::Struct,
                owner,
                &mut output,
            );
            resolve_fields(&resolver, &item.fields, owner, &mut output);
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

fn resolve_signature_attributes(
    resolver: &Resolver,
    signature: &syn::Signature,
    return_attrs: &[syn::Attribute],
    owner: usize,
    output: &mut ResolveOutput,
) {
    resolve_attributes(
        resolver,
        return_attrs,
        &[],
        AttributeTarget::Parameter,
        owner,
        output,
    );
    for input in &signature.inputs {
        if let syn::FnArg::Typed(param) = input {
            resolve_attributes(
                resolver,
                &param.attrs,
                &["r#in", "out", "opt"],
                AttributeTarget::Parameter,
                owner,
                output,
            );
        }
    }
}

fn resolve_wrapped_attributes(
    resolver: &Resolver,
    attrs: &[syn::Attribute],
    wrapper: &str,
    target: AttributeTarget,
    owner: usize,
    output: &mut ResolveOutput,
) {
    let site = output.take_attribute_site();
    for attr in attrs.iter().filter(|attr| attr.path().is_ident(wrapper)) {
        match attr.parse_args::<syn::Meta>() {
            Ok(meta) => {
                let nested: syn::Attribute = syn::parse_quote_spanned!(attr.span()=> #[#meta]);
                resolve_attributes_at_site(resolver, &[nested], &[], target, owner, site, output);
            }
            Err(_) => output
                .diagnostics
                .push(resolver.error(attr, "invalid wrapped attribute")),
        }
    }
}

fn resolve_attributes(
    resolver: &Resolver,
    attrs: &[syn::Attribute],
    controls: &[&str],
    target: AttributeTarget,
    owner: usize,
    output: &mut ResolveOutput,
) {
    let site = output.take_attribute_site();
    resolve_attributes_at_site(resolver, attrs, controls, target, owner, site, output);
}

fn resolve_attributes_at_site(
    resolver: &Resolver,
    attrs: &[syn::Attribute],
    controls: &[&str],
    target: AttributeTarget,
    owner: usize,
    site: usize,
    output: &mut ResolveOutput,
) {
    for attr in attrs {
        let path = attr.path();
        if path.is_ident("win32")
            || path.is_ident("winrt")
            || path.is_ident("arch")
            || controls.iter().any(|control| path.is_ident(control))
        {
            continue;
        }

        let result = if let Some(pseudo) = path
            .get_ident()
            .and_then(|ident| pseudo_by_short(&ident.to_string()))
        {
            resolver.resolve_pseudo_attr_ref(attr, pseudo)
        } else {
            resolver.resolve_attribute_ref(attr)
        };
        match result {
            Ok(value) => output.attributes.push(ResolvedAttribute {
                owner,
                site,
                span: attr.span(),
                target,
                value,
            }),
            Err(error) => output.diagnostics.push(error),
        }
    }
}

fn resolve_fields(resolver: &Resolver, fields: &[Field], owner: usize, output: &mut ResolveOutput) {
    for field in fields {
        resolve_attributes(
            resolver,
            &field.attrs,
            &[],
            AttributeTarget::Field,
            owner,
            output,
        );
        match &field.ty {
            FieldType::Type(ty) => {
                resolve_type(resolver, ty, output.diagnostics);
            }
            FieldType::Nested(record) => {
                resolve_attributes(
                    resolver,
                    &record.attrs,
                    &["packed", "align"],
                    AttributeTarget::Struct,
                    owner,
                    output,
                );
                resolve_fields(resolver, &record.fields, owner, output);
            }
        }
    }
}

impl ResolveOutput<'_> {
    fn take_attribute_site(&mut self) -> usize {
        let site = *self.next_attribute_site;
        *self.next_attribute_site += 1;
        site
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
