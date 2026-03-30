use super::*;

pub trait MethodDefExt {
    fn import_name(&self) -> Option<&'static str>;
    fn module_name(&self) -> String;
    fn method_signature(&self, namespace: &str, generics: &[Type], reader: &Reader) -> Signature;
}

impl MethodDefExt for MethodDef {
    fn import_name(&self) -> Option<&'static str> {
        self.impl_map().and_then(|map| {
            let import_name = map.import_name();
            if self.name() != import_name {
                Some(import_name)
            } else {
                None
            }
        })
    }

    fn module_name(&self) -> String {
        const COMBASE_FUNCTIONS: [&str; 5] = [
            "CoCreateFreeThreadedMarshaler",
            "CoIncrementMTAUsage",
            "CoTaskMemAlloc",
            "CoTaskMemFree",
            "RoGetAgileReference",
        ];

        // Workaround for https://github.com/microsoft/windows-rs/pull/3743
        if COMBASE_FUNCTIONS.contains(&self.name()) {
            "combase.dll".to_string()
        } else {
            self.impl_map()
                .map_or("", |map| map.import_scope().name())
                .to_lowercase()
        }
    }

    #[track_caller]
    fn method_signature(&self, namespace: &str, generics: &[Type], reader: &Reader) -> Signature {
        let meta_sig = self.signature(&Type::generic_placeholders(generics.len()));
        let call_flags = meta_sig.flags;
        let mut return_type =
            Type::from_metadata_type(&meta_sig.return_type, None, generics, reader);
        let mut params = vec![];
        let mut meta_types = meta_sig.types.iter();

        for param in self.params() {
            if param.sequence() == 0 {
                if param.has_attribute("ConstAttribute") {
                    return_type = return_type.to_const_type();
                }
            } else {
                let meta_ty = meta_types.next().expect("param count mismatch");
                let param_is_const = param.has_attribute("ConstAttribute");
                let param_is_input = !param.flags().contains(ParamAttributes::Out);
                let mut ty = Type::from_metadata_type(meta_ty, None, generics, reader);

                if param_is_const || param_is_input {
                    ty = ty.to_const_type();
                }

                if param_is_input {
                    ty = ty.to_const_ptr();

                    if let Some(attribute) = param.find_attribute("AssociatedEnumAttribute") {
                        if let Some((_, Value::Utf8(name))) = attribute.value().first() {
                            let overload = reader.unwrap_full_name(namespace, name);

                            ty = Type::PrimitiveOrEnum(Box::new(ty), Box::new(overload));
                        }
                    }
                }

                params.push(Param { def: param, ty });
            }
        }

        Signature {
            call_flags,
            return_type,
            params,
        }
    }
}
