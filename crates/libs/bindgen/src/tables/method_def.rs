use super::*;

pub trait MethodDefExt {
    fn import_name(&self) -> Option<&'static str>;
    fn module_name(&self) -> String;
    fn method_signature(&self, generics: &[Type], reader: &Reader) -> Signature;
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
        self.impl_map()
            .map_or("", |map| map.import_scope().name())
            .to_lowercase()
    }

    #[track_caller]
    fn method_signature(&self, generics: &[Type], reader: &Reader) -> Signature {
        let meta_sig = self.signature(&Type::generic_placeholders(generics.len()));
        let call_flags = meta_sig.flags;
        let return_type = Type::from_metadata_type(&meta_sig.return_type, None, generics, reader);
        let param_map = match self.params_by_sequence(meta_sig.types.len()) {
            Ok(params) => Some(params),
            Err(error) => {
                eprintln!(
                    "windows-bindgen: method `{}` has invalid parameter metadata: {error}",
                    self.name()
                );
                None
            }
        };
        let mut params = Vec::with_capacity(meta_sig.types.len());

        for (position, meta_ty) in meta_sig.types.iter().enumerate() {
            let def = param_map
                .as_ref()
                .and_then(|params| params.params()[position]);
            let param_is_input_only = matches!(
                def.map_or(
                    windows_metadata::reader::ParamDirection::Unspecified,
                    |param| param.direction()
                ),
                windows_metadata::reader::ParamDirection::Unspecified
                    | windows_metadata::reader::ParamDirection::Input
            );
            let mut ty = Type::from_metadata_type(meta_ty, None, generics, reader);

            if param_is_input_only {
                ty = ty.to_const_type();
                ty = ty.to_const_ptr();
            }

            let name = def.map_or_else(|| format!("p{position}"), |param| param.name().to_string());
            params.push(Param { def, name, ty });
        }

        Signature {
            call_flags,
            return_type,
            params,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_metadata::ParamAttributes;

    fn reader(rows: &[(&str, u16, ParamAttributes)]) -> Reader {
        let mut file = windows_metadata::writer::File::new("test");
        file.TypeDef(
            "Test",
            "I",
            windows_metadata::writer::TypeDefOrRef::default(),
            TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
        );
        let signature = windows_metadata::Signature {
            return_type: windows_metadata::Type::Void,
            types: vec![
                windows_metadata::Type::I32,
                windows_metadata::Type::U32,
                windows_metadata::Type::I64,
            ],
            ..Default::default()
        };
        file.MethodDef(
            "Method",
            &signature,
            MethodAttributes::Public,
            Default::default(),
        );
        for (name, sequence, flags) in rows {
            file.Param(name, *sequence, *flags);
        }

        Reader::new(vec![
            windows_metadata::reader::File::new(file.into_stream()).unwrap(),
        ])
    }

    fn method(reader: &Reader) -> MethodDef {
        let Type::CppInterface(interface) = reader.with_full_name("Test", "I").next().unwrap()
        else {
            panic!()
        };
        interface.def.methods().next().unwrap()
    }

    #[test]
    fn sparse_out_of_order_params_follow_sequence() {
        let reader = reader(&[
            ("third", 3, ParamAttributes::In | ParamAttributes::Optional),
            ("return", 0, ParamAttributes::Out),
            ("first", 1, ParamAttributes::Out),
        ]);
        let signature = method(&reader).method_signature(&[], &reader);

        assert_eq!(
            signature.params.iter().map(Param::name).collect::<Vec<_>>(),
            ["first", "p1", "third"]
        );
        assert!(
            signature
                .params
                .iter()
                .map(Param::is_input_only)
                .eq([false, true, true])
        );
        assert!(
            signature
                .params
                .iter()
                .map(Param::is_optional_or_reserved)
                .eq([false, false, true])
        );
        assert_eq!(
            signature
                .params
                .iter()
                .map(|param| param.ty.clone())
                .collect::<Vec<_>>(),
            [Type::I32, Type::U32, Type::I64]
        );
    }

    #[test]
    fn unspecified_is_input_only_and_input_output_is_output_capable() {
        let reader = reader(&[
            ("unspecified", 1, ParamAttributes::default()),
            (
                "input_output",
                2,
                ParamAttributes::In | ParamAttributes::Out,
            ),
            ("output", 3, ParamAttributes::Out),
        ]);
        let signature = method(&reader).method_signature(&[], &reader);

        assert!(
            signature
                .params
                .iter()
                .map(Param::is_input_only)
                .eq([true, false, false])
        );
    }

    #[test]
    fn malformed_params_keep_all_signature_types_with_defaults() {
        let reader = reader(&[
            ("first", 1, ParamAttributes::Out),
            ("duplicate", 1, ParamAttributes::In),
        ]);
        let signature = method(&reader).method_signature(&[], &reader);

        assert_eq!(
            signature.params.iter().map(Param::name).collect::<Vec<_>>(),
            ["p0", "p1", "p2"]
        );
        assert_eq!(signature.params.len(), 3);
        assert!(signature.params.iter().all(Param::is_input_only));
    }
}
