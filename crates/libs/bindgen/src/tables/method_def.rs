use super::*;

pub trait MethodDefExt {
    fn import_name(&self) -> Option<&'static str>;
    fn module_name(&self) -> String;
    fn method_signature(&self, namespace: &str, generics: &[Type]) -> Signature;
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
    fn method_signature(&self, namespace: &str, generics: &[Type]) -> Signature {
        let mut blob = self.blob(4);
        let call_flags = MethodCallAttributes(blob.read_u8());
        let _param_count = blob.read_usize();
        let mut return_type = Type::from_blob(&mut blob, None, generics);

        let mut params = vec![];

        for param in self.params() {
            if param.sequence() == 0 {
                if param.has_attribute("ConstAttribute") {
                    return_type = return_type.to_const_type();
                }
            } else {
                let param_is_const = param.has_attribute("ConstAttribute");
                let param_is_input = !param.flags().contains(ParamAttributes::Out);
                let mut ty = Type::from_blob(&mut blob, None, generics);

                if param_is_const || param_is_input {
                    ty = ty.to_const_type();
                }

                if param_is_input {
                    ty = ty.to_const_ptr();

                    if let Some(attribute) = param.find_attribute("AssociatedEnumAttribute") {
                        if let Some((_, Value::Str(name))) = attribute.args().first() {
                            let overload = current_reader().unwrap_full_name(namespace, name);

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
