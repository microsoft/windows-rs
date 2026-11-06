use super::*;

#[derive(Clone, Debug)]
pub struct CppMethod {
    pub def: MethodDef,
    pub signature: Signature,
    pub dependencies: TypeMap,
    pub return_hint: ReturnHint,
    pub param_hints: Vec<ParamHint>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnHint {
    None,
    Query(usize, usize),
    QueryOptional(usize, usize),
    ResultValue,
    ReturnStruct,
    ReturnValue,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ParamHint {
    None,
    ArrayRaw,
    IntoParam,
    Optional,
    ValueType,
    Blittable,
    Bool,
}

impl CppMethod {
    pub fn new(def: MethodDef, reader: &Reader) -> Self {
        let signature = def.method_signature(&[], reader);
        let mut dependencies = signature.dependencies(reader);
        for ty in def.associated_enums(reader) {
            ty.combine(&mut dependencies, reader);
        }
        let mut param_hints = vec![ParamHint::None; signature.params.len()];

        for (position, hint) in param_hints.iter_mut().enumerate() {
            let param = &signature.params[position];
            let ty = param.projection_type(reader);
            // COM values still use borrowed references, not raw-buffer ownership.
            if (param.has_buffer_attributes() || param.has_unresolved_contracts())
                && !ty.is_interface()
            {
                *hint = ParamHint::ArrayRaw;
                continue;
            }
            if !param.has_direction() {
                if ty.is_pointer() || ty.is_primitive(reader) {
                    *hint = ParamHint::ValueType;
                }
                continue;
            }

            if param.is_convertible_with_reader(reader) {
                *hint = ParamHint::IntoParam;
            } else if ty.is_copyable(reader) && param.is_optional_or_reserved() {
                *hint = ParamHint::Optional;
            } else if param.is_input_only() && ty == Type::BOOL {
                *hint = ParamHint::Bool;
            } else if ty.is_primitive(reader)
                && (!ty.is_pointer() || ty.deref().is_copyable(reader))
            {
                *hint = ParamHint::ValueType;
            } else if ty.is_copyable(reader) {
                *hint = ParamHint::Blittable;
            }
        }

        let is_retval = signature.is_retval(reader);
        let mut return_hint = ReturnHint::None;

        match signature.return_type.projection_type(reader) {
            Type::Void if is_retval => return_hint = ReturnHint::ReturnValue,
            Type::HRESULT => {
                if is_retval {
                    return_hint = ReturnHint::ResultValue;
                }

                if signature.params.len() >= 2
                    && let Some((guid, object)) =
                        signature_param_is_query(&signature.params, reader)
                {
                    if signature.params[object].is_optional_or_reserved() {
                        return_hint = ReturnHint::QueryOptional(object, guid);
                    } else {
                        return_hint = ReturnHint::Query(object, guid);
                    }
                }
            }
            Type::GUID => return_hint = ReturnHint::ReturnStruct,
            Type::CppStruct(ty) if !ty.is_handle(reader) => {
                return_hint = ReturnHint::ReturnStruct;
            }
            _ => {}
        };

        Self {
            def,
            signature,
            dependencies,
            param_hints,
            return_hint,
        }
    }

    pub fn write_cfg(&self, config: &Config, parent: &Cfg, not: bool) -> TokenStream {
        write_method_cfg(&self.dependencies, config, parent, not)
    }

    /// Projects `MustUseAttribute` unless the wrapper already returns `()` or a `#[must_use]` type.
    pub fn write_must_use(&self, reader: &Reader) -> TokenStream {
        if !self.def.attributes().any(|attribute| {
            attribute.namespace() == "Windows.Win32.Metadata"
                && attribute.name() == "MustUseAttribute"
        }) {
            return quote! {};
        }
        let is_must_use_type = |ty: Type| {
            matches!(
                ty,
                Type::BOOL | Type::HRESULT | Type::NTSTATUS | Type::RPC_STATUS
            )
        };
        let projected_must_use = match self.return_hint {
            ReturnHint::Query(..) | ReturnHint::QueryOptional(..) | ReturnHint::ResultValue => true,
            ReturnHint::ReturnValue => {
                let param = self.signature.params.last().unwrap();
                let ty = param.projection_pointee(reader).projection_type(reader);
                ty.is_interface_with_reader(reader) || is_must_use_type(ty)
            }
            ReturnHint::ReturnStruct | ReturnHint::None => {
                let ty = self.signature.return_type.projection_type(reader);
                ty == Type::Void || is_must_use_type(ty)
            }
        };
        if projected_must_use {
            quote! {}
        } else {
            quote! { #[must_use] }
        }
    }

    pub fn write_generics(&self) -> TokenStream {
        let mut tokens = quote! {};

        for (position, hint) in self.param_hints.iter().enumerate() {
            if *hint == ParamHint::IntoParam {
                let name: TokenStream = format!("P{position},").parse().unwrap();
                tokens.combine(name);
            }
        }

        tokens
    }

    pub fn write_where(&self, config: &Config, query: bool) -> TokenStream {
        let mut tokens = quote! {};

        for (position, param) in self.signature.params.iter().enumerate() {
            if self.param_hints[position] == ParamHint::IntoParam {
                let name: TokenStream = format!("P{position}").parse().unwrap();
                let into = param.write_name(config);
                tokens.combine(quote! { #name: windows_core::Param<#into>, });
            }
        }

        if query {
            tokens.combine(quote! { T: windows_core::Interface });
        }

        if tokens.is_empty() {
            tokens
        } else {
            quote! { where #tokens }
        }
    }

    pub fn write(
        &self,
        config: &Config,
        method_names: &mut MethodNames,
        virtual_names: &mut MethodNames,
    ) -> TokenStream {
        let name = method_names.add(self.def);
        let vname = virtual_names.add(self.def);

        let args = self.write_args(config);
        let params = self.write_params(config);
        let generics = self.write_generics();
        let abi_return_type = self.write_return(config);
        let result = config.write_core();

        let vis = config.item_vis();
        let must_use = self.write_must_use(config.reader);

        let wrapper = match self.return_hint {
            ReturnHint::Query(..) => {
                let where_clause = self.write_where(config, true);

                quote! {
                    #vis unsafe fn #name<#generics T>(&self, #params) -> #result Result<T> #where_clause {
                        let mut result__ = core::ptr::null_mut();
                        unsafe { (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).and_then(|| windows_core::imp::Type::from_abi(result__)) }
                    }
                }
            }
            ReturnHint::QueryOptional(..) => {
                let where_clause = self.write_where(config, true);

                quote! {
                    #vis unsafe fn #name<#generics T>(&self, #params result__: *mut Option<T>) -> #result Result<()> #where_clause {
                        unsafe { (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).ok() }
                    }
                }
            }
            ReturnHint::ResultValue => {
                let where_clause = self.write_where(config, false);

                let ty = self
                    .signature
                    .params
                    .last()
                    .unwrap()
                    .projection_pointee(config.reader);
                let map = ty
                    .projection_type(config.reader)
                    .write_result_map(config.reader);
                let return_type = ty.write_name(config);

                quote! {
                    #vis unsafe fn #name<#generics>(&self, #params) -> #result Result<#return_type> #where_clause {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).#map
                        }
                    }
                }
            }
            ReturnHint::ReturnValue => {
                let where_clause = self.write_where(config, false);

                let return_type = self.signature.params[self.signature.params.len() - 1]
                    .projection_pointee(config.reader);
                let projection_type = return_type.projection_type(config.reader);

                if projection_type.is_interface_with_reader(config.reader) {
                    let return_type = return_type.write_name(config);

                    quote! {
                        #vis unsafe fn #name<#generics>(&self, #params) -> #result Result<#return_type> #where_clause {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args);
                                windows_core::imp::Type::from_abi(result__)
                            }
                        }
                    }
                } else {
                    let map = if projection_type.is_copyable(config.reader) {
                        quote! { result__ }
                    } else {
                        quote! { core::mem::transmute(result__) }
                    };

                    let return_type = return_type.write_name(config);
                    let where_clause = self.write_where(config, false);

                    quote! {
                        #vis unsafe fn #name<#generics>(&self, #params) -> #return_type #where_clause {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args);
                                #map
                            }
                        }
                    }
                }
            }
            ReturnHint::ReturnStruct => {
                let return_type = self.signature.return_type.write_name(config);
                let where_clause = self.write_where(config, false);

                quote! {
                    #vis unsafe fn #name<#generics>(&self, #params) -> #return_type #where_clause {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), &mut result__, #args);
                            result__
                        }
                    }
                }
            }
            ReturnHint::None => {
                let where_clause = self.write_where(config, false);

                if matches!(self.signature.return_type, Type::Void) {
                    quote! {
                        #vis unsafe fn #name<#generics>(&self, #params) #abi_return_type #where_clause {
                            unsafe { (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args); }
                        }
                    }
                } else {
                    quote! {
                        #vis unsafe fn #name<#generics>(&self, #params) #abi_return_type #where_clause {
                            unsafe { (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args) }
                        }
                    }
                }
            }
        };

        quote! {
            #must_use
            #wrapper
        }
    }

    pub fn write_upcall(
        &self,
        parent_impl: &TokenStream,
        name: &TokenStream,
        config: &Config,
    ) -> TokenStream {
        let reader = config.reader;
        match self.return_hint {
            ReturnHint::ResultValue => {
                let invoke_args = self.signature.params[..self.signature.params.len() - 1]
                    .iter()
                    .map(|p| write_invoke_arg(p, reader));

                let last_param = &self.signature.params[self.signature.params.len() - 1];
                let result = last_param.write_ident();
                let result = if matches!(last_param.projection_type(reader), Type::PtrConst(..)) {
                    let target = last_param.projection_pointee(reader).write_abi(config);
                    quote! { (#result as *mut #target) }
                } else {
                    result
                };

                // For copyable types the Rust and ABI types are identical, so
                // the transmute would be from a type to itself and tripping
                // `clippy::useless_transmute`.
                let write_result = if last_param
                    .projection_pointee(reader)
                    .projection_type(reader)
                    .is_copyable(reader)
                {
                    quote! { #result.write(ok__); }
                } else {
                    let ty = last_param.projection_pointee(reader).write_name(config);
                    quote! { #result.cast::<#ty>().write(ok__); }
                };
                quote! {
                    match #parent_impl::#name(this, #(#invoke_args,)*) {
                        Ok(ok__) => {
                            // use `ptr::write` since the result could be uninitialized
                            #write_result
                            windows_core::HRESULT(0)
                        }
                        Err(err) => {
                            #result.write_bytes(0, 1);
                            err.into()
                        }
                    }
                }
            }
            ReturnHint::Query(..) | ReturnHint::QueryOptional(..) => {
                let invoke_args = self
                    .signature
                    .params
                    .iter()
                    .map(|p| write_invoke_arg(p, reader));

                quote! {
                    #parent_impl::#name(this, #(#invoke_args,)*).into()
                }
            }
            ReturnHint::ReturnStruct => {
                let invoke_args = self
                    .signature
                    .params
                    .iter()
                    .map(|p| write_invoke_arg(p, reader));

                quote! {
                    *result__ = #parent_impl::#name(this, #(#invoke_args,)*);
                }
            }
            _ => {
                let invoke_args = self
                    .signature
                    .params
                    .iter()
                    .map(|p| write_invoke_arg(p, reader));

                if matches!(self.signature.return_type, Type::Void) {
                    quote! {
                        #parent_impl::#name(this, #(#invoke_args,)*);
                    }
                } else {
                    quote! {
                        #parent_impl::#name(this, #(#invoke_args,)*)
                    }
                }
            }
        }
    }

    pub fn write_impl_signature(&self, config: &Config) -> TokenStream {
        let mut params = quote! {};

        if self.return_hint == ReturnHint::ResultValue {
            for (position, param) in self.signature.params[..self.signature.params.len() - 1]
                .iter()
                .enumerate()
            {
                params.combine(write_produce_type(
                    config,
                    param,
                    self.param_hints[position],
                ));
            }
        } else {
            for (position, param) in self.signature.params.iter().enumerate() {
                params.combine(write_produce_type(
                    config,
                    param,
                    self.param_hints[position],
                ));
            }
        }

        let result = config.write_core();

        let return_type = match self.return_hint {
            ReturnHint::Query(..) | ReturnHint::QueryOptional(..) => {
                quote! { -> #result Result<()> }
            }
            ReturnHint::ResultValue => {
                let return_type = self
                    .signature
                    .params
                    .last()
                    .unwrap()
                    .projection_pointee(config.reader)
                    .write_name(config);

                quote! { -> #result Result<#return_type> }
            }
            _ => self.write_return(config),
        };

        quote! { (&self, #params) #return_type }
    }

    pub fn write_abi(&self, config: &Config, named_params: bool) -> TokenStream {
        let mut params: Vec<_> = self
            .signature
            .params
            .iter()
            .map(|param| {
                let ty = param.write_abi(config);

                if named_params {
                    let name = param.write_ident();
                    quote! { #name: #ty }
                } else {
                    ty
                }
            })
            .collect();

        let mut return_sig = quote! {};

        match self.return_hint {
            ReturnHint::ReturnStruct => {
                let return_type = self.signature.return_type.write_abi(config);

                if named_params {
                    params.insert(0, quote! { result__: *mut #return_type });
                } else {
                    params.insert(0, quote! { *mut #return_type });
                }
            }
            _ => {
                return_sig = config.write_return_sig(self.def, &self.signature, false);
            }
        }

        let this = if named_params {
            quote! { this: *mut core::ffi::c_void }
        } else {
            quote! { *mut core::ffi::c_void }
        };

        quote! { (#this, #(#params),*) #return_sig }
    }

    pub fn write_params(&self, config: &Config) -> TokenStream {
        let mut tokens = quote! {};

        for (position, param) in self.signature.params.iter().enumerate() {
            match self.return_hint {
                ReturnHint::Query(object, guid) | ReturnHint::QueryOptional(object, guid)
                    if (object == position || guid == position) =>
                {
                    continue;
                }
                ReturnHint::ReturnValue | ReturnHint::ResultValue
                    if self.signature.params.len() - 1 == position =>
                {
                    continue;
                }
                _ => {}
            }

            let name = param.write_ident();

            match self.param_hints[position] {
                ParamHint::ArrayRaw => {
                    let kind = param.write_default(config);
                    if param.is_optional_or_reserved() {
                        tokens.combine(&quote! { #name: Option<#kind>, });
                    } else {
                        tokens.combine(&quote! { #name: #kind, });
                    }
                }
                ParamHint::IntoParam => {
                    let kind: TokenStream = format!("P{position}").parse().unwrap();
                    tokens.combine(&quote! { #name: #kind, });
                }
                ParamHint::Optional => {
                    if matches!(param.ty, Type::CppDelegate(..) | Type::FnPtr(_)) {
                        let kind = param.write_name(config);
                        tokens.combine(&quote! { #name: #kind, });
                    } else {
                        let kind = param.write_name(config);
                        tokens.combine(&quote! { #name: Option<#kind>, });
                    }
                }
                ParamHint::Bool => {
                    tokens.combine(&quote! { #name: bool, });
                }
                ParamHint::ValueType | ParamHint::Blittable => {
                    let kind = param.write_default(config);
                    tokens.combine(&quote! { #name: #kind, });
                }
                ParamHint::None => {
                    let kind = param.write_default(config);
                    tokens.combine(&quote! { #name: &#kind, });
                }
            }
        }

        tokens
    }

    pub fn write_args(&self, config: &Config) -> TokenStream {
        let mut tokens = quote! {};

        for (position, param) in self.signature.params.iter().enumerate() {
            let new = match self.return_hint {
                ReturnHint::Query(object, _) if object == position => {
                    quote! { &mut result__, }
                }
                ReturnHint::ReturnValue | ReturnHint::ResultValue
                    if self.signature.params.len() - 1 == position =>
                {
                    // A native pointer typedef spells an interface slot as `*mut Option<T>`, while
                    // the result reads the raw interface pointer.
                    if matches!(&param.ty, Type::CppStruct(ty) if ty.is_native_typedef())
                        && param
                            .projection_pointee(config.reader)
                            .projection_type(config.reader)
                            .is_interface_with_reader(config.reader)
                    {
                        quote! { core::ptr::from_mut::<*mut core::ffi::c_void>(&mut result__).cast(), }
                    } else {
                        quote! { &mut result__, }
                    }
                }
                ReturnHint::QueryOptional(object, _) if object == position => {
                    quote! { result__ as *mut _ as *mut _, }
                }
                ReturnHint::Query(_, guid) | ReturnHint::QueryOptional(_, guid)
                    if guid == position =>
                {
                    quote! { &T::IID, }
                }
                _ => {
                    let name = param.write_ident();
                    match self.param_hints[position] {
                        ParamHint::ArrayRaw => {
                            if param.is_optional_or_reserved() {
                                quote! { #name.unwrap_or(core::mem::zeroed()) as _, }
                            } else if param
                                .projection_type(config.reader)
                                .is_primitive(config.reader)
                                && param
                                    .projection_type(config.reader)
                                    .deref()
                                    .is_copyable(config.reader)
                            {
                                if param.is_input_only() {
                                    quote! { #name, }
                                } else {
                                    quote! { #name as _, }
                                }
                            } else if param.write_default(config).to_string()
                                == param.write_abi(config).to_string()
                            {
                                quote! { #name, }
                            } else if param.projection_type(config.reader).is_pointer() {
                                quote! { #name as _, }
                            } else {
                                quote! { core::mem::transmute(#name), }
                            }
                        }
                        ParamHint::IntoParam => {
                            quote! { #name.param().abi(), }
                        }
                        ParamHint::Optional => {
                            if matches!(param.ty, Type::CppDelegate(..) | Type::FnPtr(_)) {
                                quote! { #name, }
                            } else {
                                quote! { #name.unwrap_or(core::mem::zeroed()) as _, }
                            }
                        }
                        ParamHint::Bool => {
                            quote! { #name.into(), }
                        }
                        ParamHint::ValueType => {
                            if param.is_input_only()
                                || (!param.has_direction()
                                    && !param.projection_type(config.reader).is_pointer())
                            {
                                quote! { #name, }
                            } else {
                                quote! { #name as _, }
                            }
                        }
                        ParamHint::Blittable => {
                            // A blittable (copyable) param's Rust type usually matches
                            // its ABI type, making the transmute redundant. It is only
                            // needed when the two differ (e.g. a generic's `AbiType`).
                            if param.write_default(config).to_string()
                                == param.write_abi(config).to_string()
                            {
                                quote! { #name, }
                            } else if param.projection_type(config.reader).is_pointer() {
                                quote! { #name as _, }
                            } else {
                                quote! { core::mem::transmute(#name), }
                            }
                        }
                        ParamHint::None => {
                            quote! { core::mem::transmute_copy(#name), }
                        }
                    }
                }
            };
            tokens.combine(&new);
        }

        tokens
    }

    pub fn write_return(&self, config: &Config) -> TokenStream {
        match &self.signature.return_type {
            Type::Void if self.def.has_attribute("DoesNotReturnAttribute") => quote! {  -> ! },
            Type::Void => quote! {},
            ty => {
                let ty = ty.write_default(config);
                quote! { -> #ty }
            }
        }
    }
}

fn write_produce_type(config: &Config, param: &Param, hint: ParamHint) -> TokenStream {
    let name = param.write_ident();
    let kind = param.write_default(config);

    let ty = param.projection_type(config.reader);

    if ty.is_interface_with_reader(config.reader) {
        let type_name = param.write_name(config);
        quote! { #name: windows_core::Ref<#type_name>, }
    } else if param.has_direction()
        && !param.is_input_only()
        && ty.deref().is_interface_with_reader(config.reader)
        && hint != ParamHint::ArrayRaw
    {
        let type_name = param.projection_pointee(config.reader).write_name(config);
        quote! { #name: windows_core::OutRef<#type_name>, }
    } else if param.is_input_only() {
        if ty.is_primitive(config.reader) {
            quote! { #name: #kind, }
        } else {
            quote! { #name: &#kind, }
        }
    } else {
        quote! { #name: #kind, }
    }
}

fn write_invoke_arg(param: &Param, reader: &Reader) -> TokenStream {
    let name = param.write_ident();

    let ty = param.projection_type(reader);

    if ty.is_interface_with_reader(reader) {
        quote! { core::mem::transmute_copy(&#name) }
    } else if param.is_input_only() && !ty.is_primitive(reader) {
        quote! { &*(&raw const #name).cast() }
    } else {
        quote! { core::mem::transmute_copy(&#name) }
    }
}

fn signature_param_is_query(params: &[Param], reader: &Reader) -> Option<(usize, usize)> {
    if let Some(guid) = params.iter().rposition(|param| {
        param.projection_type(reader) == Type::PtrConst(Box::new(Type::GUID), 1)
            && param.is_input_only()
            && !param.has_buffer_attributes()
            && !param.has_unresolved_contracts()
    }) && let Some(object) = params.iter().rposition(|param| {
        let ty = param.projection_type(reader);
        matches!(ty, Type::PtrMut(..))
            && ty.deref() == Type::PtrMut(Box::new(Type::Void), 1)
            && param.is_metadata_output_only()
            && param.has_attribute("ComOutPtrAttribute")
            && !param.has_buffer_attributes()
            && !param.has_unresolved_contracts()
    }) {
        return Some((guid, object));
    }

    None
}
