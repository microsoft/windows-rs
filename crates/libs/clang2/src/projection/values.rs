use super::*;

pub(super) fn write_guid(value: u128) -> TokenStream {
    format!(
        "0x{:08x}_{:04x}_{:04x}_{:04x}_{:012x}",
        value >> 96,
        (value >> 80) & 0xffff,
        (value >> 64) & 0xffff,
        (value >> 48) & 0xffff,
        value & 0xffffffffffff
    )
    .parse()
    .unwrap()
}

pub(super) fn check_constant<'a>(
    constants: &mut HashMap<&'a str, (&'a TypeKind, Option<Layout>, ConstantValue)>,
    target: Target,
    name: &'a str,
    ty: &'a Type,
    value: ConstantValue,
) -> Result<(), Error> {
    let native = (&ty.kind, ty.layout, value);
    if constants.get(name).is_some_and(|old| old != &native) {
        return Err(Error(format!(
            "conflicting native definitions for constant `{name}` on {target:?}"
        )));
    }
    constants.insert(name, native);
    Ok(())
}

impl Projector<'_, '_> {
    pub(super) fn value(
        &self,
        snapshot: usize,
        observation: &ValueObservation,
        name: &str,
        imports: &Imports,
        definition: Option<&ConstantValue>,
    ) -> Result<Body, Error> {
        match &observation.kind {
            ValueKind::Function {
                language,
                mangling,
                stdcall_candidate,
                ..
            } => {
                if *language != LanguageLinkage::C {
                    return Err(Error(format!(
                        "function `{name}` requires C language linkage"
                    )));
                }
                if observation.attributes.iter().any(|attribute| {
                    !matches!(
                        attribute.kind,
                        AttributeKind::DllImport
                            | AttributeKind::DllExport
                            | AttributeKind::Sal(_)
                            | AttributeKind::Deprecated(_)
                            | AttributeKind::NoThrow
                            | AttributeKind::Cxx11NoReturn
                    )
                }) {
                    return Err(Error(format!(
                        "unsupported function attributes on `{name}`"
                    )));
                }
                let native_import = imports.functions.get(name).or_else(|| {
                    imports
                        .symbols
                        .get(&self.resolution.snapshots[snapshot].target)?
                        .get(mangling)
                });
                let alternative = stdcall_candidate.as_ref().and_then(|candidate| {
                    Some((
                        imports
                            .default_stdcall
                            .get(&self.resolution.snapshots[snapshot].target)?
                            .get(&candidate.mangling)?,
                        candidate.written,
                    ))
                });
                let (import, written) = native_import
                    .map(|import| (import, observation.written))
                    .or(alternative)
                    .ok_or_else(|| {
                        Error(format!(
                            "function `{name}` has no configured DLL/export ownership"
                        ))
                    })?;
                if import.library.is_empty() || import.name.as_ref().is_some_and(String::is_empty) {
                    return Err(Error(format!("empty DLL/export ownership for `{name}`")));
                }
                if let Some(ordinal) = import
                    .name
                    .as_deref()
                    .and_then(|name| name.strip_prefix('#'))
                    && ordinal.parse::<u16>().is_err()
                {
                    return Err(Error(format!("invalid import ordinal for `{name}`")));
                }
                let library = &import.library;
                let import_name = import
                    .name
                    .as_ref()
                    .filter(|import| import.as_str() != name)
                    .map(|name| quote! { , import = #name });
                let (parameters, result) = self.callable(snapshot, observation, true)?;
                let function = written_function(self.resolution.snapshots[snapshot], written)?;
                let abi = callable_abi(function.convention);
                let name = ident(name)?;
                let native = annotations::write(&observation.attributes);
                let contracts = annotations::function(function.prototype, &observation.attributes);
                Ok(Body {
                    attributes: quote! { #native #contracts #[library(#library #import_name)] },
                    declaration: quote! { extern #abi fn #name(#(#parameters),*) #result; },
                })
            }
            ValueKind::Constant { value, .. } => {
                let value = value.as_ref().or(definition).ok_or_else(|| {
                    Error(format!("constant `{name}` has no captured definition"))
                })?;
                if observation.attributes.iter().any(|attribute| {
                    !matches!(
                        attribute.kind,
                        AttributeKind::Sal(_)
                            | AttributeKind::Deprecated(_)
                            | AttributeKind::SelectAny
                    )
                }) {
                    return Err(Error(format!(
                        "attributed constant `{name}` is unsupported"
                    )));
                }
                if let ConstantValue::String { value, wide } = value {
                    let encoding = if *wide { "utf-16" } else { "ansi" };
                    let name = ident(name)?;
                    let native = annotations::write(&observation.attributes);
                    return Ok(Body {
                        attributes: quote! { #native #[encoding(#encoding)] },
                        declaration: quote! { const #name: String = #value; },
                    });
                }
                if let ConstantValue::Guid(guid) | ConstantValue::PropertyKey { format: guid, .. } =
                    value
                {
                    let ty = self.ty(snapshot, observation.canonical)?;
                    let expected: &[&str] = if matches!(value, ConstantValue::Guid(_)) {
                        &["GUID"]
                    } else {
                        &["PROPERTYKEY", "DEVPROPKEY"]
                    };
                    if !matches!(&ty, MetadataType::ValueName(name) if expected.contains(&name.name.as_str()))
                    {
                        return Err(Error(format!(
                            "aggregate constant `{name}` requires canonical {} metadata",
                            expected.join(" or ")
                        )));
                    }
                    let ty = write_type(self.namespace, &ty);
                    let guid = write_guid(*guid);
                    let name = ident(name)?;
                    let native = annotations::write(&observation.attributes);
                    return Ok(match value {
                        ConstantValue::Guid(_) => Body {
                            attributes: native,
                            declaration: quote! { const #name: #ty = #guid; },
                        },
                        ConstantValue::PropertyKey { id, .. } => {
                            let id =
                                write_value(self.namespace, &windows_metadata::Value::U32(*id));
                            Body {
                                attributes: quote! { #native #[guid(#guid)] },
                                declaration: quote! { const #name: #ty = #id; },
                            }
                        }
                        _ => unreachable!(),
                    });
                }
                if matches!(value, ConstantValue::Pointer(_))
                    && !matches!(
                        self.ty(snapshot, observation.canonical)?,
                        MetadataType::PtrMut(..) | MetadataType::PtrConst(..)
                    )
                {
                    return Err(Error(
                        "numeric pointer constant requires a native data-pointer representation"
                            .into(),
                    ));
                }
                let ty = match self.resolution.snapshots[snapshot].types[observation.canonical.0]
                    .kind
                {
                    TypeKind::Enum(id)
                        if self.plan.enum_constants(
                            self.resolution,
                            self.resolution.enum_groups[snapshot][id.0],
                        ) =>
                    {
                        let group = self.resolution.enum_group(EnumRef {
                            snapshot,
                            enumeration: id,
                        });
                        let reference = group.definition.ok_or_else(|| {
                            Error(format!(
                                "enum constant `{name}` has no captured enum definition"
                            ))
                        })?;
                        let enumeration = &self.resolution.snapshots[reference.snapshot].enums
                            [reference.enumeration.0]
                            .observations[reference.observation];
                        self.enum_backing(reference.snapshot, enumeration, name)?
                    }
                    _ => write_type(
                        self.namespace,
                        &self.written_ty(snapshot, observation.written, observation.canonical)?,
                    ),
                };
                self.constant_body(observation, name, value, ty)
            }
        }
    }

    fn constant_body(
        &self,
        observation: &ValueObservation,
        name: &str,
        value: &ConstantValue,
        ty: TokenStream,
    ) -> Result<Body, Error> {
        let value = match *value {
            ConstantValue::Pointer(value) => windows_metadata::Value::I64(value),
            ConstantValue::Integer(EnumValue::Signed(value)) => windows_metadata::Value::I64(value),
            ConstantValue::Integer(EnumValue::Unsigned(value)) => {
                windows_metadata::Value::U64(value)
            }
            ConstantValue::Boolean(value) => windows_metadata::Value::Bool(value),
            ConstantValue::Float32(bits) => windows_metadata::Value::F32(f32::from_bits(bits)),
            ConstantValue::Float64(bits) => windows_metadata::Value::F64(f64::from_bits(bits)),
            ConstantValue::Guid(_)
            | ConstantValue::PropertyKey { .. }
            | ConstantValue::String { .. } => unreachable!(),
        };
        let value = write_value(self.namespace, &value);
        let name = ident(name)?;
        Ok(Body {
            attributes: annotations::write(&observation.attributes),
            declaration: quote! { const #name: #ty = #value; },
        })
    }

    pub(super) fn agree_constants(
        &self,
        value: &ValueGroup,
        target: Target,
        definition: Option<&ConstantValue>,
        bodies: &mut Vec<Body>,
    ) -> Result<(), Error> {
        let Err(conflict) = agree(&value.name, target, bodies) else {
            return Ok(());
        };
        let mut canonical = Vec::with_capacity(bodies.len());
        for reference in &value.observations {
            let snapshot = self.resolution.snapshots[reference.snapshot];
            let observation =
                &snapshot.values[reference.value.0].observations[reference.observation];
            if !matches!(
                snapshot.types[observation.canonical.0].kind,
                TypeKind::Scalar(_)
            ) || !self.unannotated_scalar(reference.snapshot, observation.written)
            {
                return Err(conflict);
            }
            let ValueKind::Constant { value: number, .. } = &observation.kind else {
                return Err(Error("expected a constant observation".into()));
            };
            let ty = write_type(
                self.namespace,
                &self.ty(reference.snapshot, observation.canonical)?,
            );
            canonical.push(self.constant_body(
                observation,
                &value.name,
                number.as_ref().or(definition).unwrap(),
                ty,
            )?);
        }
        if agree(&value.name, target, &canonical).is_err() {
            return Err(conflict);
        }
        *bodies = canonical;
        Ok(())
    }

    fn unannotated_scalar(&self, snapshot: usize, written: TypeId) -> bool {
        let mut pending = vec![(snapshot, written)];
        let mut seen = HashSet::new();
        while let Some((snapshot, ty)) = pending.pop() {
            if !seen.insert((snapshot, ty)) {
                continue;
            }
            let snapshot = self.resolution.snapshots[snapshot];
            match snapshot.types[ty.0].kind {
                TypeKind::Scalar(_) => {}
                TypeKind::Alias(id) => {
                    let name = &snapshot.aliases[id.0].name;
                    let index = self
                        .resolution
                        .aliases
                        .binary_search_by(|group| group.name.cmp(name))
                        .unwrap();
                    // Contracts may come from another TU with no use of this constant.
                    for reference in &self.resolution.aliases[index].observations {
                        let alias = &self.resolution.snapshots[reference.snapshot].aliases
                            [reference.alias.0]
                            .observations[reference.observation];
                        if !alias.attributes.is_empty() {
                            return false;
                        }
                        pending.push((reference.snapshot, alias.underlying));
                    }
                }
                _ => return false,
            }
        }
        true
    }

    fn callable(
        &self,
        snapshot_id: usize,
        observation: &ValueObservation,
        named: bool,
    ) -> Result<(Vec<TokenStream>, TokenStream), Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let function = written_function(snapshot, observation.canonical)?.prototype;
        self.check_function_abi(snapshot_id, function)?;
        let written = written_function(snapshot, observation.written)?;
        let ValueKind::Function { parameters, .. } = &observation.kind else {
            return Err(Error("expected a callable observation".into()));
        };
        let parameters = self.parameter_list(snapshot_id, function, parameters, named)?;
        let result = self.written_ty(snapshot_id, written.prototype.result, function.result)?;
        let result = if result == MetadataType::Void {
            TokenStream::new()
        } else {
            let result = write_type(self.namespace, &result);
            quote! { -> #result }
        };
        Ok((parameters, result))
    }

    pub(super) fn parameters(
        &self,
        snapshot_id: usize,
        function: &Function,
        parameters: &[Parameter],
    ) -> Result<Vec<TokenStream>, Error> {
        self.parameter_list(snapshot_id, function, parameters, true)
    }

    fn parameter_list(
        &self,
        snapshot_id: usize,
        function: &Function,
        parameters: &[Parameter],
        named: bool,
    ) -> Result<Vec<TokenStream>, Error> {
        if parameters.len() != function.parameters.len() {
            return Err(Error(
                "parameter declarations and prototype arity differ".into(),
            ));
        }
        let mut names = HashSet::new();
        parameters
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
                let name = if named {
                    ident(&name)?
                } else {
                    ident(&format!("param{index}"))?
                };
                let ty = self
                    .parameter_ty(snapshot_id, parameter.written, canonical)
                    .map_err(|error| Error(format!("parameter `{name}`: {error}")))?;
                let attributes = annotations::parameter(parameter, parameters, &ty)?;
                let size = annotations::buffer(
                    self.resolution.snapshots[snapshot_id],
                    function,
                    parameters,
                    index,
                    &ty,
                )
                .map_err(|error| Error(format!("parameter `{name}`: {error}")))?;
                let default = parameter.default.as_ref().map(|value| {
                    let expression = &value.expression;
                    quote! { #[Windows::Win32::Metadata::NativeDefaultArgument(#expression)] }
                });
                let ty = write_type(self.namespace, &ty);
                Ok(quote! { #attributes #size #default #name: #ty })
            })
            .collect()
    }

    pub(super) fn agree_functions(
        &self,
        value: &ValueGroup,
        target: Target,
        bodies: &mut [Body],
    ) -> Result<(), Error> {
        let Err(conflict) = agree(&value.name, target, bodies) else {
            return Ok(());
        };
        let mut contracts = Vec::with_capacity(bodies.len());
        let mut preferred = None;
        for (index, (reference, body)) in value.observations.iter().zip(bodies.iter()).enumerate() {
            let snapshot = self.resolution.snapshots[reference.snapshot];
            let observation =
                &snapshot.values[reference.value.0].observations[reference.observation];
            let ValueKind::Function { parameters, .. } = &observation.kind else {
                return Err(Error("expected a callable observation".into()));
            };
            let named = parameters
                .iter()
                .filter(|parameter| !parameter.name.is_empty())
                .count();
            let preference = (std::cmp::Reverse(named), body.key());
            if preferred
                .as_ref()
                .is_none_or(|(_, previous)| &preference < previous)
            {
                preferred = Some((index, preference));
            }
            // SAL still binds against the original names; only comparison labels are positional.
            let (parameters, result) = self.callable(reference.snapshot, observation, false)?;
            let abi = callable_abi(written_function(snapshot, observation.written)?.convention);
            contracts.push(Body {
                attributes: body.attributes.clone(),
                declaration: quote! { extern #abi fn(#(#parameters),*) #result; },
            });
        }
        if agree(&value.name, target, &contracts).is_err() {
            return Err(conflict);
        }
        bodies.swap(0, preferred.unwrap().0);
        Ok(())
    }

    pub(super) fn uuid_tag(&self, group: &Group) -> Option<u128> {
        let first = group.records[0];
        let record = &self.resolution.snapshots[first.snapshot].records[first.record.0];
        if group.definition.is_some() || record.owner.is_some() || record.kind == RecordKind::Union
        {
            return None;
        }
        group.records.iter().find_map(|reference| {
            let snapshot = self.resolution.snapshots[reference.snapshot];
            snapshot.records[reference.record.0]
                .observations
                .iter()
                .find_map(|id| resolve::uuid(&snapshot.observations[id.0].attributes))
        })
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
                    | AttributeKind::Deprecated(_)
            )
        }) {
            return Err(Error(format!(
                "unsupported interface attributes on `{name}`"
            )));
        }
        let mut first_slot = 0;
        let mut base_alignment = None;
        let base = if let Some(&base) = definition.bases.first() {
            if base.offset_bits != 0 || !self.interface_record(snapshot_id, base.record) {
                return Err(Error(format!(
                    "interface `{name}` base is not an interface"
                )));
            }
            let (mut base_snapshot, mut record) = (snapshot_id, base.record);
            loop {
                let group = self.resolution.group(RecordRef {
                    snapshot: base_snapshot,
                    record,
                });
                let reference = group.definition.as_ref().unwrap();
                let inherited = self.resolution.snapshots[reference.snapshot].observations
                    [reference.observation.0]
                    .definition
                    .as_ref()
                    .unwrap();
                base_alignment.get_or_insert(inherited.layout.alignment);
                first_slot += inherited.methods.len() as u64;
                let Some(base) = inherited.bases.first() else {
                    break;
                };
                base_snapshot = reference.snapshot;
                record = base.record;
            }
            let base = ident(&self.plan.names[self.plan.family(snapshot_id, base.record)])?;
            quote! { : #base }
        } else {
            TokenStream::new()
        };
        let mut alignment = base_alignment.unwrap_or(size);
        if let Some(bits) = resolve::packing(&observation.attributes).1 {
            alignment = alignment.min(u64::from(bits / 8));
        }
        if definition.layout != (Layout { size, alignment }) {
            return Err(Error(format!(
                "interface `{name}` requires one vptr, no data, and at most one base"
            )));
        }
        let mut methods = Vec::new();
        let mut names = HashSet::new();
        let mut overloads = HashMap::new();
        for (index, method) in definition.methods.iter().enumerate() {
            if method.slot != first_slot + index as u64 {
                return Err(Error(format!(
                    "non-contiguous virtual method slots on `{name}`"
                )));
            }
            if !annotations::callable(&method.observation.attributes) {
                return Err(Error(format!(
                    "attributed interface method `{name}.{}`",
                    method.name
                )));
            }
            let count = overloads.entry(&method.name).or_insert(0_u32);
            *count += 1;
            let projected = if *count == 1 {
                method.name.clone()
            } else {
                format!("{}{count}", method.name)
            };
            if !names.insert(projected.clone()) {
                return Err(Error(format!(
                    "interface method name collision `{name}.{projected}`"
                )));
            }
            let function = written_function(snapshot, method.observation.canonical)?.prototype;
            if function.method_qualifiers.volatile || function.method_qualifiers.restrict {
                return Err(Error(format!(
                    "unsupported volatile/restrict method `{name}.{}`",
                    method.name
                )));
            }
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
            let (parameters, result) = self
                .callable(snapshot_id, &method.observation, true)
                .map_err(|error| {
                    Error(format!(
                        "interface method `{name}.{}`: {error}",
                        method.name
                    ))
                })?;
            let method_name = ident(&method.name)?;
            let attributes = annotations::write(&method.observation.attributes);
            let contracts = annotations::function(
                written_function(snapshot, method.observation.written)?.prototype,
                &method.observation.attributes,
            );
            let constant = function.method_qualifiers.constant.then(|| {
                quote! { #[Windows::Win32::Metadata::NativeConstMethod] }
            });
            methods.push(
                quote! { #attributes #contracts #constant fn #method_name(&self #(, #parameters)*) #result; },
            );
        }
        let uuid = write_guid(uuid);
        let native = annotations::write(&observation.attributes);
        Ok(Body {
            attributes: quote! { #native #[guid(#uuid)] },
            declaration: quote! { interface #name #base { #(#methods)* } },
        })
    }
}
