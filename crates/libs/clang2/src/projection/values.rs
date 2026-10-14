use super::*;

pub(super) fn check_constant<'a>(
    constants: &mut HashMap<&'a str, (&'a TypeKind, Option<Layout>, &'a EnumValue)>,
    target: Target,
    name: &'a str,
    ty: &'a Type,
    value: &'a EnumValue,
) -> Result<(), Error> {
    let native = (&ty.kind, ty.layout, value);
    if constants
        .insert(name, native)
        .is_some_and(|old| old != native)
    {
        return Err(Error(format!(
            "conflicting native definitions for constant `{name}` on {target:?}"
        )));
    }
    Ok(())
}

impl Projector<'_, '_> {
    pub(super) fn value(
        &self,
        snapshot: usize,
        observation: &ValueObservation,
        name: &str,
        imports: &BTreeMap<String, Import>,
    ) -> Result<Body, Error> {
        match &observation.kind {
            ValueKind::Function { language, .. } => {
                if *language != LanguageLinkage::C {
                    return Err(Error(format!(
                        "function `{name}` requires C language linkage"
                    )));
                }
                if observation.attributes.iter().any(|attribute| {
                    !matches!(
                        attribute.kind,
                        AttributeKind::DllImport | AttributeKind::Sal(_)
                    )
                }) {
                    return Err(Error(format!(
                        "unsupported function attributes on `{name}`"
                    )));
                }
                let import = imports.get(name).ok_or_else(|| {
                    Error(format!(
                        "function `{name}` has no configured DLL/export ownership"
                    ))
                })?;
                if import.library.is_empty() || import.name.as_ref().is_some_and(String::is_empty) {
                    return Err(Error(format!("empty DLL/export ownership for `{name}`")));
                }
                let library = &import.library;
                let import_name = import.name.as_ref().map(|name| quote! { , import = #name });
                let (parameters, result) = self.callable(snapshot, observation)?;
                let function =
                    written_function(self.resolution.snapshots[snapshot], observation.canonical)?;
                let abi = match function.convention {
                    CallingConvention::C => quote! { "C" },
                    CallingConvention::Stdcall => TokenStream::new(),
                };
                let name = ident(name)?;
                let native = annotations::write(&observation.attributes);
                Ok(Body {
                    attributes: quote! { #native #[library(#library #import_name)] },
                    declaration: quote! { extern #abi fn #name(#(#parameters),*) #result; },
                })
            }
            ValueKind::Constant { value, .. } => {
                if !annotations::only_sal(&observation.attributes) {
                    return Err(Error(format!(
                        "attributed constant `{name}` is unsupported"
                    )));
                }
                let ty = self.written_ty(snapshot, observation.written, observation.canonical)?;
                let ty = write_type(self.namespace, &ty);
                let value = match *value {
                    EnumValue::Signed(value) => windows_metadata::Value::I64(value),
                    EnumValue::Unsigned(value) => windows_metadata::Value::U64(value),
                };
                let value = write_value(self.namespace, &value);
                let name = ident(name)?;
                Ok(Body {
                    attributes: annotations::write(&observation.attributes),
                    declaration: quote! { const #name: #ty = #value; },
                })
            }
        }
    }

    fn callable(
        &self,
        snapshot_id: usize,
        observation: &ValueObservation,
    ) -> Result<(Vec<TokenStream>, TokenStream), Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let function = written_function(snapshot, observation.canonical)?;
        let written = written_function(snapshot, observation.written)?;
        let ValueKind::Function { parameters, .. } = &observation.kind else {
            return Err(Error("expected a callable observation".into()));
        };
        if parameters.len() != function.parameters.len() {
            return Err(Error(
                "parameter declarations and prototype arity differ".into(),
            ));
        }
        let mut names = HashSet::new();
        let parameters = parameters
            .iter()
            .zip(&function.parameters)
            .enumerate()
            .map(|(index, (parameter, &canonical))| {
                let name = if parameter.name.is_empty() {
                    format!("param{index}")
                } else {
                    parameter.name.clone()
                };
                if !names.insert(name.clone()) {
                    return Err(Error(format!("duplicate projected parameter `{name}`")));
                }
                let name = ident(&name)?;
                let ty = self.parameter_ty(snapshot_id, parameter.written, canonical)?;
                let attributes = annotations::parameter(&parameter.attributes, &ty)?;
                let ty = write_type(self.namespace, &ty);
                Ok(quote! { #attributes #name: #ty })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let result = self.written_ty(snapshot_id, written.result, function.result)?;
        let result = if result == MetadataType::Void {
            TokenStream::new()
        } else {
            let result = write_type(self.namespace, &result);
            quote! { -> #result }
        };
        Ok((parameters, result))
    }

    pub(super) fn interface_record(&self, snapshot: usize, record: RecordId) -> bool {
        let group = self.resolution.group(RecordRef { snapshot, record });
        group.definition.is_some_and(|reference| {
            let observation = &self.resolution.snapshots[reference.snapshot].observations
                [reference.observation.0];
            let definition = observation.definition.as_ref().unwrap();
            resolve::uuid(&observation.attributes).is_some()
                && (!definition.methods.is_empty() || !definition.bases.is_empty())
        })
    }

    pub(super) fn interface(
        &self,
        snapshot_id: usize,
        observation: &RecordObservation,
        name: &TokenStream,
    ) -> Result<Body, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let definition = observation.definition.as_ref().unwrap();
        let uuid = resolve::uuid(&observation.attributes)
            .ok_or_else(|| Error(format!("interface `{name}` has no native UUID")))?;
        let size = if snapshot.target == Target::X86 { 4 } else { 8 };
        if !definition.fields.is_empty()
            || definition.bases.len() > 1
            || definition.layout
                != (Layout {
                    size,
                    alignment: size,
                })
            || snapshot.records[observation.record.0].owner.is_some()
        {
            return Err(Error(format!(
                "interface `{name}` requires one vptr, no data, and at most one base"
            )));
        }
        if observation.attributes.iter().any(|attribute| {
            !matches!(
                attribute.kind,
                AttributeKind::Uuid(_)
                    | AttributeKind::NoVTable
                    | AttributeKind::DefaultTypeVisibility
                    | AttributeKind::MaxFieldAlignment { .. }
                    | AttributeKind::Sal(_)
            )
        }) {
            return Err(Error(format!(
                "unsupported interface attributes on `{name}`"
            )));
        }
        let base = if let Some(&base) = definition.bases.first() {
            if !self.interface_record(snapshot_id, base) {
                return Err(Error(format!(
                    "interface `{name}` base is not an interface"
                )));
            }
            let base = ident(&self.plan.names[self.plan.family(snapshot_id, base)])?;
            quote! { : #base }
        } else {
            TokenStream::new()
        };
        let mut methods = Vec::new();
        let mut names = HashSet::new();
        for method in &definition.methods {
            if !names.insert(&method.name) || !annotations::only_sal(&method.observation.attributes)
            {
                return Err(Error(format!(
                    "overloaded or attributed interface method `{name}.{}`",
                    method.name
                )));
            }
            let function = written_function(snapshot, method.observation.canonical)?;
            let convention = if snapshot.target == Target::X86 {
                CallingConvention::Stdcall
            } else {
                CallingConvention::C
            };
            if function.convention != convention {
                return Err(Error(format!(
                    "interface method `{name}.{}` does not use the platform COM convention",
                    method.name
                )));
            }
            let (parameters, result) = self.callable(snapshot_id, &method.observation)?;
            let method_name = ident(&method.name)?;
            let attributes = annotations::write(&method.observation.attributes);
            methods.push(quote! { #attributes fn #method_name(&self #(, #parameters)*) #result; });
        }
        let uuid: TokenStream = format!(
            "0x{:08x}_{:04x}_{:04x}_{:04x}_{:012x}",
            uuid >> 96,
            (uuid >> 80) & 0xffff,
            (uuid >> 64) & 0xffff,
            (uuid >> 48) & 0xffff,
            uuid & 0xffffffffffff
        )
        .parse()
        .map_err(|_| Error("cannot encode UUID".into()))?;
        let native = annotations::write(&observation.attributes);
        Ok(Body {
            attributes: quote! { #native #[guid(#uuid)] },
            declaration: quote! { interface #name #base { #(#methods)* } },
        })
    }
}
