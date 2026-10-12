use super::*;
use quote::ToTokens;

/// One parsed RDL declaration or member, without reference resolution or metadata lowering.
///
/// Entries retain source order and duplicate declarations. IDs and parent IDs are local to the
/// input file. Shapes and attributes are normalized token spellings, not native correctness or
/// semantic-equivalence assertions.
#[derive(Debug, Eq, PartialEq)]
pub struct InventoryEntry {
    pub id: usize,
    pub parent: Option<usize>,
    pub namespace: String,
    pub kind: &'static str,
    pub name: String,
    pub shape: String,
    pub attributes: String,
    /// Explicit architecture mask, or the enclosing entry's mask for a member.
    pub architecture: Option<i32>,
    /// One-based source line.
    pub line: usize,
}

/// Inventories one RDL file using the same parser as the metadata reader.
///
/// No references are required, and no output files are written. Parsing failures retain their
/// source location. The inventory describes the existing RDL, not the original header.
pub fn inventory(path: impl AsRef<Path>) -> Result<Vec<InventoryEntry>, Error> {
    let files = expand_rdl_files(&[path.as_ref().to_path_buf()], &[])?;
    let mut visitor = Inventory {
        entries: vec![],
        namespace: String::new(),
        source: path.as_ref().to_string_lossy().into_owned(),
    };
    for file in files {
        for item in &file.uses {
            visitor.push(None, "use", "", tokens(item), &[], item.span())?;
        }
        for item in &file.items {
            visitor.item(item, None)?;
        }
    }
    Ok(visitor.entries)
}

fn tokens(value: impl ToTokens) -> String {
    value.to_token_stream().to_string()
}

struct Inventory {
    entries: Vec<InventoryEntry>,
    namespace: String,
    source: String,
}

impl Inventory {
    fn push(
        &mut self,
        parent: Option<usize>,
        kind: &'static str,
        name: &str,
        shape: String,
        attrs: &[syn::Attribute],
        span: Span,
    ) -> Result<usize, Error> {
        let mut architecture = None;
        for attr in attrs.iter().filter(|attr| attr.path().is_ident("arch")) {
            let parsed = attr
                .parse_args::<syn::Expr>()
                .ok()
                .and_then(|expr| parse_arch_bitmask(&expr));
            let Some(bits) = parsed else {
                let start = attr.span().start();
                return Err(Error::new(
                    "invalid inventory architecture mask",
                    &self.source,
                    start.line,
                    start.column,
                ));
            };
            if architecture.replace(bits).is_some() {
                let start = attr.span().start();
                return Err(Error::new(
                    "duplicate inventory architecture mask",
                    &self.source,
                    start.line,
                    start.column,
                ));
            }
        }
        let id = self.entries.len();
        self.entries.push(InventoryEntry {
            id,
            parent,
            namespace: self.namespace.clone(),
            kind,
            name: name.to_string(),
            shape,
            attributes: attrs.iter().map(tokens).collect::<Vec<_>>().join(" "),
            architecture: architecture
                .or_else(|| parent.and_then(|parent| self.entries[parent].architecture)),
            line: span.start().line,
        });
        Ok(id)
    }

    fn item(&mut self, item: &Item, parent: Option<usize>) -> Result<(), Error> {
        match item {
            Item::Module(item) => {
                let namespace = if self.namespace.is_empty() {
                    item.name.to_string()
                } else {
                    format!("{}.{}", self.namespace, item.name)
                };
                let previous = std::mem::replace(&mut self.namespace, namespace);
                let id = self.push(
                    parent,
                    "namespace",
                    &item.name.to_string(),
                    String::new(),
                    &item.attrs,
                    item.name.span(),
                )?;
                for item in &item.items {
                    self.item(item, Some(id))?;
                }
                self.namespace = previous;
            }
            Item::Struct(item) => {
                let id = self.push(
                    parent,
                    "struct",
                    &item.name.to_string(),
                    String::new(),
                    &item.attrs,
                    item.name.span(),
                )?;
                self.fields(id, &item.fields)?;
            }
            Item::Union(item) => {
                let id = self.push(
                    parent,
                    "union",
                    &item.name.to_string(),
                    String::new(),
                    &item.attrs,
                    item.name.span(),
                )?;
                self.fields(id, &item.fields)?;
            }
            Item::Enum(item) => {
                let id = self.push(
                    parent,
                    "enum",
                    &item.name.to_string(),
                    String::new(),
                    &item.attrs,
                    item.name.span(),
                )?;
                for variant in &item.variants {
                    self.push(
                        Some(id),
                        "variant",
                        &variant.ident.to_string(),
                        tokens(variant),
                        &variant.attrs,
                        variant.ident.span(),
                    )?;
                }
            }
            Item::Typedef(item) => {
                self.push(
                    parent,
                    "typedef",
                    &item.name.to_string(),
                    tokens(&item.ty),
                    &item.attrs,
                    item.name.span(),
                )?;
            }
            Item::Const(item) => {
                let shape = if let Some(expr) = &item.expr {
                    format!("{} = {}", tokens(&item.ty), tokens(expr))
                } else {
                    tokens(&item.ty)
                };
                self.push(
                    parent,
                    "constant",
                    &item.name.to_string(),
                    shape,
                    &item.attrs,
                    item.name.span(),
                )?;
            }
            Item::Fn(item) => {
                let id = self.push(
                    parent,
                    "function",
                    &item.sig.ident.to_string(),
                    format!("extern {} {}", tokens(&item.abi), tokens(&item.sig)),
                    &item.attrs,
                    item.sig.ident.span(),
                )?;
                self.signature(id, &item.sig, &item.return_attrs)?;
            }
            Item::Callback(item) => {
                let id = self.push(
                    parent,
                    "callback",
                    &item.sig.ident.to_string(),
                    format!("extern {} {}", tokens(&item.abi), tokens(&item.sig)),
                    &item.attrs,
                    item.sig.ident.span(),
                )?;
                self.signature(id, &item.sig, &item.return_attrs)?;
            }
            Item::Delegate(item) => {
                let id = self.push(
                    parent,
                    "delegate",
                    &item.sig.ident.to_string(),
                    tokens(&item.sig),
                    &item.attrs,
                    item.sig.ident.span(),
                )?;
                self.signature(id, &item.sig, &item.return_attrs)?;
            }
            Item::Interface(item) => {
                let id = self.push(
                    parent,
                    "interface",
                    &item.name.to_string(),
                    tokens(&item.generics),
                    &item.attrs,
                    item.name.span(),
                )?;
                for base in &item.requires {
                    self.push(Some(id), "base", "", tokens(base), &[], base.span())?;
                }
                for member in &item.members {
                    match member {
                        InterfaceMember::Method(method) => {
                            let method_id = self.push(
                                Some(id),
                                "method",
                                &method.sig.ident.to_string(),
                                tokens(&method.sig),
                                &method.attrs,
                                method.sig.ident.span(),
                            )?;
                            self.signature(method_id, &method.sig, &method.return_attrs)?;
                        }
                        InterfaceMember::Property(property) => {
                            self.push(
                                Some(id),
                                "property",
                                &property.name.to_string(),
                                tokens(&property.ty),
                                &property.attrs,
                                property.name.span(),
                            )?;
                        }
                        InterfaceMember::Event(event) => {
                            self.push(
                                Some(id),
                                "event",
                                &event.name.to_string(),
                                tokens(&event.handler_ty),
                                &event.attrs,
                                event.name.span(),
                            )?;
                        }
                    }
                }
            }
            Item::Class(item) => {
                let id = self.push(
                    parent,
                    "class",
                    &item.name.to_string(),
                    String::new(),
                    &item.attrs,
                    item.name.span(),
                )?;
                if let Some(base) = &item.extends {
                    self.push(Some(id), "base", "", tokens(base), &[], base.span())?;
                }
                for interface in &item.interfaces {
                    self.push(
                        Some(id),
                        "implements",
                        "",
                        tokens(&interface.ty),
                        &interface.attrs,
                        interface.ty.span(),
                    )?;
                }
            }
            Item::Attribute(item) => {
                let id = self.push(
                    parent,
                    "attribute",
                    &item.name.to_string(),
                    String::new(),
                    &item.attrs,
                    item.name.span(),
                )?;
                for method in &item.methods {
                    let method_id = self.push(
                        Some(id),
                        "constructor",
                        "",
                        tokens(method),
                        &[],
                        method.span(),
                    )?;
                    for arg in &method.inputs {
                        self.push(
                            Some(method_id),
                            "parameter",
                            &arg.name
                                .as_ref()
                                .map(|(name, _)| name.to_string())
                                .unwrap_or_default(),
                            tokens(&arg.ty),
                            &arg.attrs,
                            arg.span(),
                        )?;
                    }
                }
                for (name, ty) in &item.properties {
                    self.push(
                        Some(id),
                        "property",
                        &name.to_string(),
                        tokens(ty),
                        &[],
                        name.span(),
                    )?;
                }
            }
        }
        Ok(())
    }

    fn fields(&mut self, parent: usize, fields: &[Field]) -> Result<(), Error> {
        for field in fields {
            let shape = match &field.ty {
                FieldType::Type(ty) => tokens(ty),
                FieldType::Nested(record) => {
                    if record.is_union { "union" } else { "struct" }.to_string()
                }
            };
            let id = self.push(
                Some(parent),
                "field",
                &field.name.to_string(),
                shape,
                &field.attrs,
                field.name.span(),
            )?;
            if let FieldType::Nested(record) = &field.ty {
                let nested = self.push(
                    Some(id),
                    if record.is_union { "union" } else { "struct" },
                    "",
                    String::new(),
                    &record.attrs,
                    field.name.span(),
                )?;
                self.fields(nested, &record.fields)?;
            }
            let mut offset = 0_u64;
            for bitfield in &field.bitfields {
                self.push(
                    Some(id),
                    "bitfield",
                    &bitfield
                        .name
                        .as_ref()
                        .map_or_else(|| "_".into(), ToString::to_string),
                    format!("offset={offset}, width={}", bitfield.width),
                    &[],
                    bitfield
                        .name
                        .as_ref()
                        .map_or(field.name.span(), syn::Ident::span),
                )?;
                offset += u64::from(bitfield.width);
            }
        }
        Ok(())
    }

    fn signature(
        &mut self,
        parent: usize,
        sig: &syn::Signature,
        return_attrs: &[syn::Attribute],
    ) -> Result<(), Error> {
        for arg in &sig.inputs {
            match arg {
                syn::FnArg::Receiver(receiver) => {
                    self.push(
                        Some(parent),
                        "receiver",
                        "self",
                        tokens(receiver),
                        &receiver.attrs,
                        receiver.span(),
                    )?;
                }
                syn::FnArg::Typed(arg) => {
                    self.push(
                        Some(parent),
                        "parameter",
                        &tokens(&arg.pat),
                        tokens(&arg.ty),
                        &arg.attrs,
                        arg.span(),
                    )?;
                }
            }
        }
        if let Some(variadic) = &sig.variadic {
            self.push(
                Some(parent),
                "variadic",
                "",
                tokens(variadic),
                &variadic.attrs,
                variadic.span(),
            )?;
        }
        self.push(
            Some(parent),
            "return",
            "",
            match &sig.output {
                syn::ReturnType::Default => "void".into(),
                syn::ReturnType::Type(_, ty) => tokens(ty),
            },
            return_attrs,
            sig.output.span(),
        )?;
        Ok(())
    }
}
