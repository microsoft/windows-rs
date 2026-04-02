use super::guid;
use super::*;

syn::custom_keyword!(interface);
syn::custom_keyword!(event);

const EVENT_REGISTRATION_TOKEN_NAMESPACE: &str = "Windows.Foundation";
const EVENT_REGISTRATION_TOKEN_NAME: &str = "EventRegistrationToken";

#[derive(Debug)]
pub struct Property {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub ty: syn::Type,
}

#[derive(Debug)]
pub struct Event {
    pub attrs: Vec<syn::Attribute>,
    pub _token: event,
    pub name: syn::Ident,
    pub ty: syn::Type,
}

#[derive(Debug)]
pub enum InterfaceItem {
    Method(Method),
    Property(Property),
    Event(Event),
}

impl syn::parse::Parse for InterfaceItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;

        if input.peek(event) {
            let _token = input.parse::<event>()?;
            let name = input.parse::<syn::Ident>()?;
            input.parse::<syn::Token![:]>()?;
            let ty = input.parse::<syn::Type>()?;
            input.parse::<syn::Token![;]>()?;
            Ok(InterfaceItem::Event(Event {
                attrs,
                _token,
                name,
                ty,
            }))
        } else if input.peek(syn::Token![fn]) {
            // Regular method (attrs already consumed).
            let sig = input.parse::<syn::Signature>()?;
            input.parse::<syn::Token![;]>()?;
            Ok(InterfaceItem::Method(Method { attrs, sig }))
        } else {
            // Inferred read-write property: `PropertyName: Type;`
            let name = input.parse::<syn::Ident>()?;
            input.parse::<syn::Token![:]>()?;
            let ty = input.parse::<syn::Type>()?;
            input.parse::<syn::Token![;]>()?;
            Ok(InterfaceItem::Property(Property { attrs, name, ty }))
        }
    }
}

#[derive(Debug)]
pub struct Interface {
    pub attrs: Vec<syn::Attribute>,
    pub token: interface,
    pub name: syn::Ident,
    pub generics: syn::Generics,
    pub requires: Vec<syn::Path>,
    pub methods: Vec<InterfaceItem>,
    pub winrt: bool,
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
        let mut methods = vec![];

        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        Ok(Self {
            attrs,
            token,
            name,
            generics,
            requires,
            methods,
            winrt: false,
        })
    }
}

impl Encoder<'_> {
    pub fn encode_interface(&mut self, item: &Interface) -> Result<(), Error> {
        let mut flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Abstract
            | metadata::TypeAttributes::Interface;

        if item.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        let mut generics = Vec::with_capacity(item.generics.params.len());
        for generic in item.generics.params.iter() {
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

        let already_has_guid = item.attrs.iter().any(|attr| self.is_guid_attribute(attr));

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(interface),
            &item.attrs,
            &[],
        )?;

        if !item.winrt && item.requires.len() > 1 {
            return self.err(
                &item.requires[1],
                "non-WinRT interface can only inherit from one interface",
            );
        }

        for require in &item.requires {
            let ty = self.encode_path(require)?;
            self.output.InterfaceImpl(interface, &ty);
        }

        let base_flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Abstract
            | metadata::MethodAttributes::NewSlot
            | metadata::MethodAttributes::Virtual;

        let mut method_signatures: Vec<(String, Vec<metadata::Type>, metadata::Type)> = Vec::new();

        for interface_item in &item.methods {
            match interface_item {
                InterfaceItem::Method(method) => {
                    self.encode_interface_method(
                        method,
                        base_flags,
                        already_has_guid,
                        &mut method_signatures,
                    )?;
                }
                InterfaceItem::Property(prop) => {
                    self.encode_interface_property(
                        prop,
                        base_flags,
                        already_has_guid,
                        &mut method_signatures,
                    )?;
                }
                InterfaceItem::Event(ev) => {
                    self.encode_interface_event(
                        ev,
                        base_flags,
                        already_has_guid,
                        &mut method_signatures,
                    )?;
                }
            }
        }

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

    fn encode_interface_method(
        &mut self,
        method: &Method,
        base_flags: metadata::MethodAttributes,
        already_has_guid: bool,
        method_signatures: &mut Vec<(String, Vec<metadata::Type>, metadata::Type)>,
    ) -> Result<(), Error> {
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
                    params.push(self.param(pt)?);
                }
            }
        }

        let types: Vec<metadata::Type> = params.iter().map(|param| param.ty.clone()).collect();
        let return_type = self.encode_return_type(&method.sig.output)?;

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
                    return self.err(attr, "`special` attribute does not accept arguments");
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

        self.encode_attrs(
            metadata::writer::HasAttribute::MethodDef(method_def),
            &method.attrs,
            &["special"],
        )?;

        for (sequence, param) in params.iter().enumerate() {
            let param_id = self.output.Param(
                &param.name,
                (sequence + 1).try_into().unwrap(),
                param.attributes,
            );

            self.encode_attrs(
                metadata::writer::HasAttribute::Param(param_id),
                &param.attrs,
                &["input", "output", "optional"],
            )?;
        }

        Ok(())
    }

    fn encode_interface_property(
        &mut self,
        prop: &Property,
        base_flags: metadata::MethodAttributes,
        already_has_guid: bool,
        method_signatures: &mut Vec<(String, Vec<metadata::Type>, metadata::Type)>,
    ) -> Result<(), Error> {
        let ty = self.encode_type(&prop.ty)?;
        let special_flags = base_flags | metadata::MethodAttributes::SpecialName;
        let name = prop.name.to_string();

        // Encode `get_Name() -> Type`
        let get_name = format!("get_{name}");
        let get_sig = metadata::Signature {
            flags: metadata::MethodCallAttributes::HASTHIS,
            return_type: ty.clone(),
            types: vec![],
        };
        if !already_has_guid {
            method_signatures.push((get_name.clone(), vec![], ty.clone()));
        }
        let get_def = self
            .output
            .MethodDef(&get_name, &get_sig, special_flags, Default::default());
        self.encode_attrs(
            metadata::writer::HasAttribute::MethodDef(get_def),
            &prop.attrs,
            &[],
        )?;

        // Encode `put_Name(value: Type)`
        let put_name = format!("put_{name}");
        let put_sig = metadata::Signature {
            flags: metadata::MethodCallAttributes::HASTHIS,
            return_type: metadata::Type::Void,
            types: vec![ty.clone()],
        };
        if !already_has_guid {
            method_signatures.push((put_name.clone(), vec![ty.clone()], metadata::Type::Void));
        }
        let put_def = self
            .output
            .MethodDef(&put_name, &put_sig, special_flags, Default::default());
        self.encode_attrs(
            metadata::writer::HasAttribute::MethodDef(put_def),
            &prop.attrs,
            &[],
        )?;
        self.output.Param("value", 1, metadata::ParamAttributes::In);

        Ok(())
    }

    fn encode_interface_event(
        &mut self,
        ev: &Event,
        base_flags: metadata::MethodAttributes,
        already_has_guid: bool,
        method_signatures: &mut Vec<(String, Vec<metadata::Type>, metadata::Type)>,
    ) -> Result<(), Error> {
        let handler_ty = self.encode_type(&ev.ty)?;
        let token_ty = metadata::Type::value_named(
            EVENT_REGISTRATION_TOKEN_NAMESPACE,
            EVENT_REGISTRATION_TOKEN_NAME,
        );
        let special_flags = base_flags | metadata::MethodAttributes::SpecialName;
        let name = ev.name.to_string();

        // Encode `add_Name(handler: HandlerType) -> EventRegistrationToken`
        let add_name = format!("add_{name}");
        let add_sig = metadata::Signature {
            flags: metadata::MethodCallAttributes::HASTHIS,
            return_type: token_ty.clone(),
            types: vec![handler_ty.clone()],
        };
        if !already_has_guid {
            method_signatures.push((add_name.clone(), vec![handler_ty.clone()], token_ty.clone()));
        }
        let add_def = self
            .output
            .MethodDef(&add_name, &add_sig, special_flags, Default::default());
        self.encode_attrs(
            metadata::writer::HasAttribute::MethodDef(add_def),
            &ev.attrs,
            &[],
        )?;
        self.output
            .Param("handler", 1, metadata::ParamAttributes::In);

        // Encode `remove_Name(token: EventRegistrationToken)`
        let remove_name = format!("remove_{name}");
        let remove_sig = metadata::Signature {
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
        let remove_def =
            self.output
                .MethodDef(&remove_name, &remove_sig, special_flags, Default::default());
        self.encode_attrs(
            metadata::writer::HasAttribute::MethodDef(remove_def),
            &ev.attrs,
            &[],
        )?;
        self.output.Param("token", 1, metadata::ParamAttributes::In);

        Ok(())
    }
}
