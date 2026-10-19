use super::guid;
use super::*;

syn::custom_keyword!(interface);
syn::custom_keyword!(event);

#[derive(Debug)]
pub struct Interface {
    pub attrs: Vec<syn::Attribute>,
    pub token: interface,
    pub name: syn::Ident,
    pub generics: syn::Generics,
    pub requires: Vec<syn::Path>,
    pub members: Vec<InterfaceMember>,
    pub winrt: bool,
}

#[derive(Debug)]
pub struct Property {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub ty: syn::Type,
}

#[derive(Debug)]
pub struct Event {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub handler_ty: syn::Type,
}

#[derive(Debug)]
pub enum InterfaceMember {
    Method(Method),
    Property(Property),
    Event(Event),
}

impl syn::parse::Parse for InterfaceMember {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Peek past any outer attributes to determine which member kind follows.
        let fork = input.fork();
        fork.call(syn::Attribute::parse_outer)?;

        if fork.peek(syn::Token![fn]) {
            return input.parse().map(Self::Method);
        }

        if fork.peek(event) {
            let attrs = input.call(syn::Attribute::parse_outer)?;
            let _event_token: event = input.parse()?;
            let name: syn::Ident = input.parse()?;
            input.parse::<syn::Token![:]>()?;
            let handler_ty: syn::Type = input.parse()?;
            input.parse::<syn::Token![;]>()?;
            return Ok(Self::Event(Event {
                attrs,
                name,
                handler_ty,
            }));
        }

        // Property shorthand: `[#[get] | #[set]] Name: Type;`
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty: syn::Type = input.parse()?;
        input.parse::<syn::Token![;]>()?;
        Ok(Self::Property(Property { attrs, name, ty }))
    }
}

impl syn::parse::Parse for Interface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;
        let generics = input.parse()?;

        let requires = if input.parse::<syn::Token![:]>().is_ok() {
            let mut requires = vec![input.parse::<syn::Path>()?];
            while input.parse::<syn::Token![+]>().is_ok() {
                requires.push(input.parse::<syn::Path>()?);
            }
            requires
        } else {
            vec![]
        };

        let content;
        syn::braced!(content in input);
        let mut members = vec![];

        while !content.is_empty() {
            members.push(content.parse()?);
        }

        Ok(Self {
            attrs,
            token,
            name,
            generics,
            requires,
            members,
            winrt: false,
        })
    }
}

impl Encoder<'_> {
    pub fn encode_interface(&mut self, item: &Interface) -> Result<(), Error> {
        let has_exclusive_to = item
            .attrs
            .iter()
            .any(|attr| self.is_exclusive_to_attribute(attr));

        let mut flags = metadata::TypeAttributes::Abstract | metadata::TypeAttributes::Interface;

        if !has_exclusive_to {
            flags |= metadata::TypeAttributes::Public;
        }

        if item.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        let mut generics = Vec::with_capacity(item.generics.params.len());
        for generic in &item.generics.params {
            let syn::GenericParam::Type(generic) = generic else {
                return self.err(generic, "only type generic parameters are supported");
            };
            generics.push(generic.ident.to_string());
        }
        self.generics = generics;

        let mut name = self.name.to_string();

        if !self.generics.is_empty() {
            name = format!("{name}`{}", self.generics.len());
        }

        let interface = self.output.TypeDef(
            self.namespace,
            &name,
            metadata::writer::TypeDefOrRef::default(),
            flags,
        );

        for (number, name) in self.generics.iter().enumerate() {
            self.output.GenericParam(
                name,
                metadata::writer::TypeOrMethodDef::TypeDef(interface),
                number.try_into().unwrap(),
                metadata::GenericParamAttributes::None,
            );
        }

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(interface),
            &item.attrs,
            &["guid", "no_guid"],
        )?;

        if let Some(arch_bits) = self.read_arch(&item.attrs)? {
            self.emit_arch_attribute(
                metadata::writer::HasAttribute::TypeDef(interface),
                arch_bits,
            );
        }

        let already_has_guid = self.encode_guid_pseudo_attrs(
            metadata::writer::HasAttribute::TypeDef(interface),
            &item.attrs,
        )?;

        if !item.winrt && item.requires.len() > 1 {
            return self.err(
                &item.requires[1],
                "non-WinRT interface can only inherit from one interface",
            );
        }

        for require in &item.requires {
            let ty = self.encode_path(require)?;
            if !matches!(&ty, metadata::Type::ClassName(_)) {
                return self.err(require, "interface base must resolve to an interface type");
            }
            self.output.InterfaceImpl(interface, &ty);
        }

        let base_flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Abstract
            | metadata::MethodAttributes::NewSlot
            | metadata::MethodAttributes::Virtual;

        let mut method_signatures: Vec<(String, Vec<metadata::Type>, metadata::Type)> = Vec::new();
        let mut accessors: Vec<Accessor> = Vec::new();

        for member in &item.members {
            match member {
                InterfaceMember::Method(method) => {
                    let mut params = vec![];

                    if method.sig.inputs.is_empty() {
                        return self.err(&method.sig.ident, "`&self` parameter not found");
                    }

                    for (sequence, arg) in method.sig.inputs.iter().enumerate() {
                        match arg {
                            syn::FnArg::Receiver(receiver) => {
                                if *receiver != syn::parse_quote! { &self } {
                                    return self.err(receiver, "`&self` parameter not found");
                                }
                            }
                            syn::FnArg::Typed(pt) => {
                                if sequence == 0 {
                                    return self.err(arg, "`&self` parameter not found");
                                }
                                let p = self.param(pt)?;
                                if item.winrt {
                                    self.validate_type_is_winrt(&pt.ty, &p.ty)?;
                                }
                                params.push(p);
                            }
                        }
                    }

                    let types: Vec<metadata::Type> =
                        params.iter().map(|param| param.ty.clone()).collect();
                    let return_type = self.encode_return_type(&method.sig.output)?;

                    if item.winrt
                        && let syn::ReturnType::Type(_, return_syn_ty) = &method.sig.output
                    {
                        self.validate_type_is_winrt(return_syn_ty.as_ref(), &return_type)?;
                    }

                    if !already_has_guid {
                        method_signatures.push((
                            method.sig.ident.to_string(),
                            types.clone(),
                            return_type.clone(),
                        ));
                    }

                    let signature = metadata::Signature {
                        flags: metadata::MethodCallAttributes::HASTHIS,
                        return_type,
                        types,
                    };

                    let mut is_special = false;
                    for attr in &method.attrs {
                        if attr.path().is_ident("special") {
                            if !matches!(attr.meta, syn::Meta::Path(_)) {
                                return self
                                    .err(attr, "`special` attribute does not accept arguments");
                            }
                            is_special = true;
                        }
                    }

                    let mut flags = base_flags;
                    if is_special {
                        flags |= metadata::MethodAttributes::SpecialName;
                    }

                    let method_def = self.output.MethodDef(
                        &method.sig.ident.to_string(),
                        &signature,
                        flags,
                        Default::default(),
                    );

                    // Explicit WinRT accessors belong to their property or event just like the
                    // shorthand ones, so the shorthand is only a spelling choice.
                    if item.winrt
                        && is_special
                        && let Some((kind, name)) = accessor_kind(
                            &method.sig.ident.to_string(),
                            &signature.types,
                            &signature.return_type,
                        )
                    {
                        let ty = match kind {
                            AccessorKind::Get => signature.return_type.clone(),
                            _ => signature.types[0].clone(),
                        };
                        accessors.push(Accessor {
                            kind,
                            name: name.to_string(),
                            ty,
                            method: method_def,
                            span: &method.sig.ident,
                        });
                    }

                    self.encode_attrs(
                        metadata::writer::HasAttribute::MethodDef(method_def),
                        &method.attrs,
                        &["special"],
                    )?;

                    self.encode_return_attrs(&method.return_attrs)?;
                    self.encode_params(&params)?;
                }

                InterfaceMember::Property(prop) => {
                    let is_get_only = prop.attrs.iter().any(|a| a.path().is_ident("get"));
                    let is_set_only = prop.attrs.iter().any(|a| a.path().is_ident("set"));

                    if is_get_only && is_set_only {
                        return self.err(
                            &prop.name,
                            "property cannot have both `#[get]` and `#[set]` attributes",
                        );
                    }

                    for attr in &prop.attrs {
                        if (attr.path().is_ident("get") || attr.path().is_ident("set"))
                            && !matches!(attr.meta, syn::Meta::Path(_))
                        {
                            return self
                                .err(attr, "`get`/`set` attribute does not accept arguments");
                        }
                    }

                    let ty = self.encode_type(&prop.ty)?;
                    let method_flags = base_flags | metadata::MethodAttributes::SpecialName;

                    let mut get_method = None;
                    let mut put_method = None;

                    if !is_set_only {
                        let get_name = format!("get_{}", prop.name);
                        let signature = metadata::Signature {
                            flags: metadata::MethodCallAttributes::HASTHIS,
                            return_type: ty.clone(),
                            types: vec![],
                        };
                        if !already_has_guid {
                            method_signatures.push((get_name.clone(), vec![], ty.clone()));
                        }
                        let method = self.output.MethodDef(
                            &get_name,
                            &signature,
                            method_flags,
                            Default::default(),
                        );
                        // Other property attributes apply to each accessor, as in MIDL.
                        self.encode_attrs(
                            metadata::writer::HasAttribute::MethodDef(method),
                            &prop.attrs,
                            &["get", "set"],
                        )?;
                        self.encode_simple_params(&[])?;
                        get_method = Some(method);
                    }

                    if !is_get_only {
                        let put_name = format!("put_{}", prop.name);
                        let signature = metadata::Signature {
                            flags: metadata::MethodCallAttributes::HASTHIS,
                            return_type: metadata::Type::Void,
                            types: vec![ty.clone()],
                        };
                        if !already_has_guid {
                            method_signatures.push((
                                put_name.clone(),
                                vec![ty.clone()],
                                metadata::Type::Void,
                            ));
                        }
                        let method = self.output.MethodDef(
                            &put_name,
                            &signature,
                            method_flags,
                            Default::default(),
                        );
                        self.encode_attrs(
                            metadata::writer::HasAttribute::MethodDef(method),
                            &prop.attrs,
                            &["get", "set"],
                        )?;
                        self.encode_simple_params(&[("value", &ty)])?;
                        put_method = Some(method);
                    }

                    if let Some(method) = get_method {
                        accessors.push(Accessor {
                            kind: AccessorKind::Get,
                            name: prop.name.to_string(),
                            ty: ty.clone(),
                            method,
                            span: &prop.name,
                        });
                    }

                    if let Some(method) = put_method {
                        accessors.push(Accessor {
                            kind: AccessorKind::Set,
                            name: prop.name.to_string(),
                            ty,
                            method,
                            span: &prop.name,
                        });
                    }
                }

                InterfaceMember::Event(evt) => {
                    let handler_ty = self.encode_type(&evt.handler_ty)?;
                    if !matches!(handler_ty, metadata::Type::ClassName(_)) {
                        return self.err(&evt.handler_ty, "event handler must be a delegate type");
                    }
                    let token_ty = event_token();
                    let method_flags = base_flags | metadata::MethodAttributes::SpecialName;

                    let add_name = format!("add_{}", evt.name);
                    let add_signature = metadata::Signature {
                        flags: metadata::MethodCallAttributes::HASTHIS,
                        return_type: token_ty.clone(),
                        types: vec![handler_ty.clone()],
                    };
                    if !already_has_guid {
                        method_signatures.push((
                            add_name.clone(),
                            vec![handler_ty.clone()],
                            token_ty.clone(),
                        ));
                    }
                    let add_method = self.output.MethodDef(
                        &add_name,
                        &add_signature,
                        method_flags,
                        Default::default(),
                    );
                    // Event attributes apply to both accessors, as in MIDL.
                    self.encode_attrs(
                        metadata::writer::HasAttribute::MethodDef(add_method),
                        &evt.attrs,
                        &[],
                    )?;
                    self.encode_simple_params(&[("handler", &handler_ty)])?;

                    let remove_name = format!("remove_{}", evt.name);
                    let remove_signature = metadata::Signature {
                        flags: metadata::MethodCallAttributes::HASTHIS,
                        return_type: metadata::Type::Void,
                        types: vec![token_ty.clone()],
                    };
                    if !already_has_guid {
                        method_signatures.push((
                            remove_name.clone(),
                            vec![token_ty.clone()],
                            metadata::Type::Void,
                        ));
                    }
                    let remove_method = self.output.MethodDef(
                        &remove_name,
                        &remove_signature,
                        method_flags,
                        Default::default(),
                    );
                    self.encode_attrs(
                        metadata::writer::HasAttribute::MethodDef(remove_method),
                        &evt.attrs,
                        &[],
                    )?;
                    self.encode_simple_params(&[("token", &token_ty)])?;

                    accessors.push(Accessor {
                        kind: AccessorKind::Add,
                        name: evt.name.to_string(),
                        ty: handler_ty,
                        method: add_method,
                        span: &evt.name,
                    });
                    accessors.push(Accessor {
                        kind: AccessorKind::Remove,
                        name: evt.name.to_string(),
                        ty: token_ty,
                        method: remove_method,
                        span: &evt.name,
                    });
                }
            }
        }

        self.encode_accessors(interface, &accessors)?;

        if !already_has_guid {
            let methods: Vec<(&str, &[metadata::Type], &metadata::Type)> = method_signatures
                .iter()
                .map(|(name, types, ret)| (name.as_str(), types.as_slice(), ret))
                .collect();

            guid::derive_and_emit_guid(
                self.output,
                metadata::writer::HasAttribute::TypeDef(interface),
                self.namespace,
                self.name,
                &methods,
            );
        }

        Ok(())
    }

    /// Writes one Property row per (name, type) and one Event row per name, associating every
    /// accessor through MethodSemantics. The rows are written after the interface's methods so
    /// each type's property and event lists stay contiguous.
    fn encode_accessors(
        &mut self,
        interface: metadata::writer::TypeDef,
        accessors: &[Accessor],
    ) -> Result<(), Error> {
        let mut properties: Vec<(&str, &metadata::Type, Vec<&Accessor>)> = vec![];
        let mut events: Vec<(&str, Vec<&Accessor>)> = vec![];

        for accessor in accessors {
            let group = match accessor.kind {
                AccessorKind::Get | AccessorKind::Set => {
                    let pos = properties
                        .iter()
                        .position(|(name, ty, _)| *name == accessor.name && **ty == accessor.ty)
                        .unwrap_or_else(|| {
                            properties.push((&accessor.name, &accessor.ty, vec![]));
                            properties.len() - 1
                        });
                    &mut properties[pos].2
                }
                AccessorKind::Add | AccessorKind::Remove => {
                    let pos = events
                        .iter()
                        .position(|(name, _)| *name == accessor.name)
                        .unwrap_or_else(|| {
                            events.push((&accessor.name, vec![]));
                            events.len() - 1
                        });
                    &mut events[pos].1
                }
            };

            if group.iter().any(|other| other.kind == accessor.kind) {
                return self.err(accessor.span, "duplicate property or event accessor");
            }

            group.push(accessor);
        }

        let mut first = true;

        for (name, ty, group) in properties {
            let property = self.output.Property(name, ty);

            if std::mem::take(&mut first) {
                self.output.PropertyMap(interface, property);
            }

            for kind in [AccessorKind::Get, AccessorKind::Set] {
                if let Some(accessor) = group.iter().find(|accessor| accessor.kind == kind) {
                    self.output.MethodSemantics(
                        if kind == AccessorKind::Get {
                            metadata::MethodSemanticsAttributes::Getter.0
                        } else {
                            metadata::MethodSemanticsAttributes::Setter.0
                        },
                        accessor.method,
                        metadata::writer::HasSemantics::Property(property),
                    );
                }
            }
        }

        first = true;

        for (name, group) in events {
            // The handler type comes from `add`; a lone `remove` has no event to describe.
            let Some(add) = group
                .iter()
                .find(|accessor| accessor.kind == AccessorKind::Add)
            else {
                continue;
            };

            let event = self.output.Event(name, &add.ty);

            if std::mem::take(&mut first) {
                self.output.EventMap(interface, event);
            }

            for kind in [AccessorKind::Add, AccessorKind::Remove] {
                if let Some(accessor) = group.iter().find(|accessor| accessor.kind == kind) {
                    self.output.MethodSemantics(
                        if kind == AccessorKind::Add {
                            metadata::MethodSemanticsAttributes::AddOn.0
                        } else {
                            metadata::MethodSemanticsAttributes::RemoveOn.0
                        },
                        accessor.method,
                        metadata::writer::HasSemantics::Event(event),
                    );
                }
            }
        }

        Ok(())
    }
}

/// A property or event accessor method awaiting its Property/Event row.
struct Accessor<'a> {
    kind: AccessorKind,
    name: String,
    /// The property type, the event handler type, or the event token type.
    ty: metadata::Type,
    method: metadata::writer::MethodDef,
    span: &'a syn::Ident,
}
