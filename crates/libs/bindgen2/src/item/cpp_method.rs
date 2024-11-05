use super::*;

#[derive(Clone, Debug)]
pub struct CppMethod {
    pub namespace: &'static str, // for namespace resolution of some attributes
    pub def: MethodDef,
    pub signature: Signature,
    pub dependencies: Dependencies,
    pub return_hint: ReturnHint,
    pub param_hints: Vec<ParamHint>,
}

#[derive(Clone, Debug)]
pub enum ReturnHint {
    None,
    Query(usize, usize),
    QueryOptional(usize, usize),
    ResultValue,
    ResultVoid,
    ReturnStruct,
    ReturnValue,
    ReturnVoid,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ParamHint {
    None,
    ArrayFixed(usize),
    ArrayRelativeLen(usize),
    ArrayRelativeByteLen(usize),
    ArrayRelativePtr(usize),
    IntoParam,
    OptionalPointer,
    ValueType,
    Blittable,
}

impl ParamHint {
    fn from_param(param: Param) -> Self {
        for attribute in param.attributes() {
            match attribute.name() {
                "NativeArrayInfoAttribute" => {
                    for (_, value) in attribute.args() {
                        match value {
                            Value::I16(value) => {
                                return ParamHint::ArrayRelativeLen(value as usize)
                            }
                            Value::I32(value) => return ParamHint::ArrayFixed(value as usize),
                            _ => {}
                        }
                    }
                }
                "MemorySizeAttribute" => {
                    for (_, value) in attribute.args() {
                        if let Value::I16(value) = value {
                            return ParamHint::ArrayRelativeByteLen(value as usize);
                        }
                    }
                }
                _ => {}
            }
        }
        ParamHint::None
    }

    fn is_array(&self) -> bool {
        matches!(
            self,
            Self::ArrayFixed(_)
                | Self::ArrayRelativeLen(_)
                | Self::ArrayRelativeByteLen(_)
                | Self::ArrayRelativePtr(_)
        )
    }
}

impl CppMethod {
    pub fn new(def: MethodDef, namespace: &'static str) -> Self {
        let signature = def.signature(&[]);

        let mut dependencies = Dependencies::new();
        signature.dependencies(&mut dependencies);

        let mut param_hints = vec![ParamHint::None; signature.params.len()];

        for (position, (_, param)) in signature.params.iter().enumerate() {
            // TODO: here's where we resolve PrimitiveOrEnum if needed
            param_hints[position] = ParamHint::from_param(*param);
        }

        for position in 0..signature.params.len() {
            // Point len params back to the corresponding ptr params.
            match param_hints[position] {
                ParamHint::ArrayRelativeLen(relative)
                | ParamHint::ArrayRelativeByteLen(relative) => {
                    // The len params must be input only.
                    if !signature.params[relative]
                        .1
                        .flags()
                        .contains(ParamAttributes::Out)
                        && position != relative
                        && !signature.params[relative].0.is_pointer()
                    {
                        param_hints[relative] = ParamHint::ArrayRelativePtr(position);
                    } else {
                        param_hints[position] = ParamHint::None;
                    }
                }
                ParamHint::ArrayFixed(_) => {
                    if signature.params[position]
                        .1
                        .has_attribute("FreeWithAttribute")
                    {
                        param_hints[position] = ParamHint::None;
                    }
                }
                _ => {}
            }
        }

        let mut sets = BTreeMap::<usize, Vec<usize>>::new();

        // Finds sets of ptr params pointing at the same len param.
        for (position, hint) in param_hints.iter().enumerate() {
            match hint {
                ParamHint::ArrayRelativeLen(relative)
                | ParamHint::ArrayRelativeByteLen(relative) => {
                    sets.entry(*relative).or_default().push(position);
                }
                _ => {}
            }
        }

        // Remove all sets.
        for (len, ptrs) in sets {
            if ptrs.len() > 1 {
                param_hints[len] = ParamHint::None;
                for ptr in ptrs {
                    param_hints[ptr] = ParamHint::None;
                }
            }
        }

        // Remove any byte arrays that aren't byte-sized types.
        for position in 0..param_hints.len() {
            if let ParamHint::ArrayRelativeByteLen(relative) = param_hints[position] {
                if !signature.params[position].0.is_byte_size() {
                    param_hints[position] = ParamHint::None;
                    param_hints[relative] = ParamHint::None;
                }
            }
        }

        for (position, hint) in param_hints.iter_mut().enumerate() {
            if *hint == ParamHint::None {
                if is_convertible(
                    &signature.params[position].0,
                    signature.params[position].1,
                    *hint,
                ) {
                    *hint = ParamHint::IntoParam;
                } else {
                    let flags = signature.params[position].1.flags();
                    if signature.params[position].0.is_pointer()
                        && (flags.contains(ParamAttributes::Optional)
                            || signature.params[position]
                                .1
                                .has_attribute("ReservedAttribute"))
                    {
                        *hint = ParamHint::OptionalPointer;
                    } else if signature.params[position].0.is_primitive()
                        && (!signature.params[position].0.is_pointer()
                            || signature.params[position].0.deref().is_blittable())
                    {
                        *hint = ParamHint::ValueType;
                    } else if signature.params[position].0.is_blittable() {
                        *hint = ParamHint::Blittable;
                    }
                }
            }
        }

        let is_retval = is_retval(&signature, &param_hints);
        let mut return_hint = ReturnHint::None;

        let last_error = if let Some(map) = def.impl_map() {
            map.flags().contains(PInvokeAttributes::SupportsLastError)
        } else {
            false
        };

        // TODO: ignore this attribute and just return HRESULT/BOOL/NTSTATUS/whatever and let the caller user `.ok()` if
        // they want to treat it as a Result with a single success value.
        if !def.has_attribute("CanReturnMultipleSuccessValuesAttribute") {
            match &signature.return_type.0 {
                Type::Void if is_retval => return_hint = ReturnHint::ReturnValue,
                Type::Void => return_hint = ReturnHint::ReturnVoid,
                Type::HRESULT => {
                    if is_retval {
                        return_hint = ReturnHint::ResultValue
                    } else {
                        return_hint = ReturnHint::ResultVoid
                    };

                    if signature.params.len() >= 2 {
                        if let Some((guid, object)) = signature_param_is_query(&signature.params) {
                            if signature.params[object]
                                .1
                                .flags()
                                .contains(ParamAttributes::Optional)
                            {
                                return_hint = ReturnHint::QueryOptional(object, guid);
                            } else {
                                return_hint = ReturnHint::Query(object, guid);
                            }
                        }
                    }
                }
                Type::Item(Item::CppStruct(item))
                    if TypeName(item.def.namespace(), item.def.name()) == TypeName::BOOL
                        && last_error =>
                {
                    // TODO: maybe use ResultBool here to make the code gen less ambiguous
                    return_hint = ReturnHint::ResultVoid
                }
                Type::GUID => return_hint = ReturnHint::ReturnStruct,
                Type::Item(Item::CppStruct(item)) if !item.is_handle() => {
                    return_hint = ReturnHint::ReturnStruct
                }
                _ => {}
            };
        }

        Self {
            def,
            namespace,
            signature,
            dependencies,
            param_hints,
            return_hint,
        }
    }

    pub fn write_generics(&self) -> TokenStream {
        let mut tokens = quote! {};

        for (position, hint) in self.param_hints.iter().enumerate() {
            if *hint == ParamHint::IntoParam {
                let name: TokenStream = format!("P{position},").into();
                tokens.combine(name);
            }
        }

        tokens
    }

    pub fn write_where(&self, writer: &Writer) -> TokenStream {
        let mut tokens = quote! {};

        for (position, (ty, _)) in self.signature.params.iter().enumerate() {
            if self.param_hints[position] == ParamHint::IntoParam {
                let name: TokenStream = format!("P{position}").into();
                let into = ty.write(writer);
                tokens.combine(quote! { #name: windows_core::Param<#into>, })
            }
        }

        tokens
    }

    // TODO: this is CppInterface specific so maybe just put it there.
    pub fn write(
        &self,
        writer: &Writer,
        method_names: &mut MethodNames,
        virtual_names: &mut MethodNames,
    ) -> TokenStream {
        let name = method_names.add(self.def);
        let vname = virtual_names.add(self.def);

        let args = self.write_args();
        let params = self.write_params(writer);
        let generics = self.write_generics();
        let where_clause = self.write_where(writer);
        let abi_return_type = self.write_return(writer);

        // TODO: make sure these are consistent across CppFn and CppInterface -
        // maybe find a way to consolidate code gen?
        match self.return_hint {
            ReturnHint::Query(..) => {
                quote! {
                    pub unsafe fn #name<#generics T>(&self, #params) -> windows_core::Result<T> where #where_clause T: windows_core::Interface {
                        let mut result__ = core::ptr::null_mut();
                        (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).and_then(||windows_core::Type::from_abi(result__))
                    }
                }
            }
            ReturnHint::QueryOptional(..) => {
                quote! {
                    pub unsafe fn #name<#generics T>(&self, #params result__: *mut Option<T>) -> windows_core::Result<()> where #where_clause  T: windows_core::Interface {
                        (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).ok()
                    }
                }
            }
            ReturnHint::ResultValue => {
                let return_type = self.signature.params[self.signature.params.len() - 1]
                    .0
                    .deref();

                let map = if return_type.is_blittable() {
                    quote! { map(||result__) }
                } else {
                    quote! { and_then(||windows_core::Type::from_abi(result__)) }
                };

                let return_type = return_type.write(writer);

                quote! {
                    pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<#return_type> where #where_clause {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).#map
                    }
                }
            }
            ReturnHint::ResultVoid => {
                quote! {
                    pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<()> where #where_clause {
                        (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).ok()
                    }
                }
            }
            ReturnHint::ReturnValue => {
                let return_type = self.signature.params[self.signature.params.len() - 1]
                    .0
                    .deref();

                if return_type.is_nullable() {
                    let return_type = return_type.write(writer);

                    quote! {
                        pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<#return_type> #where_clause {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args);
                            windows_core::Type::from_abi(result__)
                        }
                    }
                } else {
                    let map = if return_type.is_blittable() {
                        quote! { result__ }
                    } else {
                        quote! { core::mem::transmute(result__) }
                    };

                    let return_type = return_type.write(writer);

                    quote! {
                        pub unsafe fn #name<#generics>(&self, #params) -> #return_type where #where_clause {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args);
                            #map
                        }
                    }
                }
            }
            ReturnHint::ReturnStruct | ReturnHint::None => {
                if self.handle_last_error() {
                    let return_type = self.signature.return_type.0.write(writer);

                    quote! {
                        pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<#return_type> where #where_clause {
                            let result__ = (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args);
                            (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
                        }
                    }
                } else {
                    quote! {
                        pub unsafe fn #name<#generics>(&self, #params) #abi_return_type where #where_clause {
                            (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args)
                        }
                    }
                }
            }
            ReturnHint::ReturnVoid => {
                quote! {
                    pub unsafe fn #name<#generics>(&self, #params) where #where_clause {
                        (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args)
                    }
                }
            }
        }
    }

    pub fn write_abi(&self, writer: &Writer, named_params: bool) -> TokenStream {
        let params = self.signature.params.iter().map(|(ty, param)| {
            // TODO: use ty.underlying_types() ?
            let ty = ty.write_abi(writer);

            if named_params {
                let name = to_ident(&param.name().to_lowercase());
                quote! { #name: #ty }
            } else {
                ty
            }
        });

        let return_sig = writer.write_return_sig(self.def, &self.signature, false);

        let this = if named_params {
            quote! { this: *mut core::ffi::c_void }
        } else {
            quote! { *mut core::ffi::c_void }
        };

        quote! { (#this, #(#params),*) #return_sig }
    }

    pub fn write_params(&self, writer: &Writer) -> TokenStream {
        let mut tokens = quote! {};

        for (position, (ty, param)) in self.signature.params.iter().enumerate() {
            match self.return_hint {
                ReturnHint::Query(object, guid) | ReturnHint::QueryOptional(object, guid) => {
                    if object == position || guid == position {
                        continue;
                    }
                }
                ReturnHint::ReturnValue | ReturnHint::ResultValue
                    if self.signature.params.len() - 1 == position =>
                {
                    continue;
                }
                _ => {}
            }

            let name = to_ident(&param.name().to_lowercase());

            match self.param_hints[position] {
                ParamHint::ArrayFixed(fixed) => {
                    let ty = ty.deref();
                    let ty = ty.write_default(writer);
                    let len = Literal::u32_unsuffixed(fixed as u32);
                    let ty = if param.flags().contains(ParamAttributes::Out) {
                        quote! { &mut [#ty; #len] }
                    } else {
                        quote! { &[#ty; #len] }
                    };
                    if param.flags().contains(ParamAttributes::Optional) {
                        tokens.combine(&quote! { #name: Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                ParamHint::ArrayRelativeLen(_) => {
                    let ty = ty.deref();
                    let ty = ty.write_default(writer);
                    let ty = if param.flags().contains(ParamAttributes::Out) {
                        quote! { &mut [#ty] }
                    } else {
                        quote! { &[#ty] }
                    };
                    if param.flags().contains(ParamAttributes::Optional) {
                        tokens.combine(&quote! { #name: Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                ParamHint::ArrayRelativeByteLen(_) => {
                    let ty = if param.flags().contains(ParamAttributes::Out) {
                        quote! { &mut [u8] }
                    } else {
                        quote! { &[u8] }
                    };
                    if param.flags().contains(ParamAttributes::Optional) {
                        tokens.combine(&quote! { #name: Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                ParamHint::ArrayRelativePtr(_) => {}
                ParamHint::IntoParam => {
                    let kind: TokenStream = format!("P{position}").into();
                    tokens.combine(&quote! { #name: #kind, });
                }
                ParamHint::OptionalPointer => {
                    let kind = ty.write_default(writer);
                    tokens.combine(&quote! { #name: Option<#kind>, });
                }
                ParamHint::ValueType | ParamHint::Blittable => {
                    let kind = ty.write_default(writer);
                    tokens.combine(&quote! { #name: #kind, });
                }
                ParamHint::None => {
                    let kind = ty.write_default(writer);
                    tokens.combine(&quote! { #name: &#kind, });
                }
            }
        }

        tokens
    }

    pub fn write_args(&self) -> TokenStream {
        let mut tokens = quote! {};

        for (position, (ty, param)) in self.signature.params.iter().enumerate() {
            let new = match self.return_hint {
                ReturnHint::Query(object, _) if object == position => {
                    quote! { &mut result__, }
                }
                ReturnHint::ReturnValue | ReturnHint::ResultValue
                    if self.signature.params.len() - 1 == position =>
                {
                    quote! { &mut result__, }
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
                    let name = to_ident(&param.name().to_lowercase());
                    let flags = param.flags();
                    match self.param_hints[position] {
                        ParamHint::ArrayFixed(_)
                        | ParamHint::ArrayRelativeLen(_)
                        | ParamHint::ArrayRelativeByteLen(_) => {
                            let map = if flags.contains(ParamAttributes::Optional) {
                                quote! { #name.as_deref().map_or(core::ptr::null(), |slice|slice.as_ptr()) }
                            } else {
                                quote! { #name.as_ptr() }
                            };
                            quote! { core::mem::transmute(#map), }
                        }
                        ParamHint::ArrayRelativePtr(relative) => {
                            let name =
                                to_ident(&self.signature.params[relative].1.name().to_lowercase());
                            let flags = self.signature.params[relative].1.flags();
                            if flags.contains(ParamAttributes::Optional) {
                                quote! { #name.as_deref().map_or(0, |slice|slice.len().try_into().unwrap()), }
                            } else {
                                quote! { #name.len().try_into().unwrap(), }
                            }
                        }
                        ParamHint::IntoParam => {
                            quote! { #name.param().abi(), }
                        }
                        ParamHint::OptionalPointer => {
                            if flags.contains(ParamAttributes::Out) {
                                quote! { core::mem::transmute(#name.unwrap_or(core::ptr::null_mut())), }
                            } else {
                                quote! { core::mem::transmute(#name.unwrap_or(core::ptr::null())), }
                            }
                        }
                        ParamHint::ValueType => {
                            quote! { #name, }
                        }
                        ParamHint::Blittable => {
                            if matches!(ty, Type::PrimitiveOrEnum(_, _)) {
                                quote! { #name.0 as _, }
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
            tokens.combine(&new)
        }

        tokens
    }

    pub fn write_return(&self, writer: &Writer) -> TokenStream {
        match &self.signature.return_type.0 {
            Type::Void if self.def.has_attribute("DoesNotReturnAttribute") => quote! {  -> ! },
            Type::Void => quote! {},
            ty => {
                let ty = ty.write_default(writer);
                quote! { -> #ty }
            }
        }
    }

    pub fn handle_last_error(&self) -> bool {
        if let Some(map) = self.def.impl_map() {
            if map.flags().contains(PInvokeAttributes::SupportsLastError) {
                if let Type::Item(Item::CppStruct(item)) = &self.signature.return_type.0 {
                    if item.is_handle() {
                        // https://github.com/microsoft/windows-rs/issues/2392#issuecomment-1477765781
                        if self.def.name() == "LocalFree" {
                            return false;
                        }
                        if item.def.underlying_type().is_pointer() {
                            return true;
                        }
                        if !item.def.invalid_values().is_empty() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

fn is_convertible(ty: &Type, param: Param, hint: ParamHint) -> bool {
    !param.flags().contains(ParamAttributes::Out)
        && !ty.is_winrt_array()
        && !ty.is_pointer()
        && !hint.is_array()
        && (ty.is_borrowed() || ty.is_trivially_convertible())
}

fn is_retval(signature: &Signature, param_hints: &[ParamHint]) -> bool {
    // First we check whether there's an actual retval parameter.
    if let Some(param) = signature.params.last() {
        if param.1.has_attribute("RetValAttribute") {
            return true;
        }
    }

    if let Some((ty, param)) = signature.params.last() {
        if is_param_retval(ty, *param, param_hints[param_hints.len() - 1]) {
            return signature.params[..signature.params.len() - 1]
                .iter()
                .all(|(_, param)| !param.flags().contains(ParamAttributes::Out));
        }
    }

    false
}

fn is_param_retval(ty: &Type, param: Param, hint: ParamHint) -> bool {
    // The Win32 metadata uses `RetValAttribute` to call out retval methods but it is employed
    // very sparingly, so this heuristic is used to apply the transformation more uniformly.
    if param.has_attribute("RetValAttribute") {
        return true;
    }
    if !ty.is_pointer() {
        return false;
    }
    if ty.is_void() {
        return false;
    }
    let flags = param.flags();
    if flags.contains(ParamAttributes::In)
        || !flags.contains(ParamAttributes::Out)
        || flags.contains(ParamAttributes::Optional)
        || hint.is_array()
    {
        return false;
    }
    if hint.is_array() {
        return false;
    }

    // If it's bigger than 128 bits, best to pass as a reference.
    if ty.deref().size() > 16 {
        return false;
    }
    true
}

fn signature_param_is_query(params: &[(Type, Param)]) -> Option<(usize, usize)> {
    if let Some(guid) = params.iter().rposition(|(ty, param)| {
        *ty == Type::PtrConst(Box::new(Type::GUID), 1)
            && !param.flags().contains(ParamAttributes::Out)
    }) {
        if let Some(object) = params.iter().rposition(|(ty, param)| {
            *ty == Type::PtrMut(Box::new(Type::Void), 2)
                && param.has_attribute("ComOutPtrAttribute")
        }) {
            return Some((guid, object));
        }
    }

    None
}
