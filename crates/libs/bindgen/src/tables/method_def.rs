use super::*;

impl std::fmt::Debug for MethodDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MethodDef").field(&self.name()).finish()
    }
}

impl MethodDef {
    pub fn flags(&self) -> MethodAttributes {
        MethodAttributes(self.usize(2) as u16)
    }

    pub fn name(&self) -> &'static str {
        self.str(3)
    }

    pub fn params(&self) -> RowIterator<Param> {
        self.list(5)
    }

    pub fn impl_map(&self) -> Option<ImplMap> {
        self.file()
            .equal_range(1, MemberForwarded::MethodDef(*self).encode())
            .next()
    }

    pub fn module_name(&self) -> &'static str {
        self.impl_map().map_or("", |map| map.scope().name())
    }

    pub fn signature(&self, namespace: &str, generics: &[Type]) -> Signature {
        let mut blob = self.blob(4);
        let call_flags = MethodCallAttributes(blob.read_usize() as u8);
        let _param_count = blob.read_usize();
        let mut return_type = Type::from_blob(&mut blob, None, generics);
        let mut return_param = None;

        let params = self
            .params()
            .filter_map(|param| {
                if param.sequence() == 0 {
                    if param.has_attribute("ConstAttribute") {
                        return_type = return_type.to_const_type();
                    }

                    return_param = Some(param);
                    None
                } else {
                    let param_is_const = param.has_attribute("ConstAttribute");
                    let param_is_output = param.flags().contains(ParamAttributes::Out);
                    let mut ty = Type::from_blob(&mut blob, None, generics);

                    if param_is_const || !param_is_output {
                        ty = ty.to_const_type();
                    }
                    if !param_is_output {
                        ty = ty.to_const_ptr();
                    }

                    if !param_is_output {
                        if let Some(attribute) = param.find_attribute("AssociatedEnumAttribute") {
                            if let Some((_, Value::Str(name))) = attribute.args().first() {
                                let overload = param.reader().unwrap_full_name(namespace, name);

                                ty = Type::PrimitiveOrEnum(Box::new(ty), Box::new(overload));
                            }
                        }
                    }

                    Some((ty, param))
                }
            })
            .collect();

        Signature {
            call_flags,
            return_type: (return_type, return_param),
            params,
        }
    }
}
