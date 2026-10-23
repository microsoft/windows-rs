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
        let MemberRefParent::TypeDef(owner) = self.parent() else {
            unreachable!()
        };
        let native = !owner.flags().contains(TypeAttributes::WindowsRuntime);
        let convert = if native {
            Type::from_native_signature_type
        } else {
            Type::from_metadata_type
        };
        let return_type = convert(&meta_sig.return_type, None, generics, reader);
        return_type.assert_callable_value(reader);
        let param_map = match self.params_by_sequence(meta_sig.types.len()) {
            Ok(params) => Some(params),
            Err(error) => {
                assert!(
                    native,
                    "windows-bindgen: WinRT method `{}.{}.{}` has invalid parameter metadata: {error}",
                    owner.namespace(),
                    owner.name(),
                    self.name()
                );
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
            let name = def.map_or_else(|| format!("p{position}"), |param| param.name().to_string());
            assert!(
                native
                    || def.is_some_and(|param| matches!(
                        param.direction(),
                        windows_metadata::reader::ParamDirection::Input
                            | windows_metadata::reader::ParamDirection::Output
                    )),
                "windows-bindgen: WinRT method `{}.{}.{}` parameter `{name}` (position {}) \
                 requires exactly one of In or Out",
                owner.namespace(),
                owner.name(),
                self.name(),
                position + 1
            );
            let param_is_input_only = def.is_some_and(|param| {
                param.direction() == windows_metadata::reader::ParamDirection::Input
            });
            let mut ty = convert(meta_ty, None, generics, reader);
            ty.assert_callable_value(reader);

            let native_reference = native
                && matches!(
                    meta_ty,
                    windows_metadata::Type::RefConst(_) | windows_metadata::Type::RefMut(_)
                );
            if param_is_input_only && !native_reference {
                ty = ty.to_const_type();
                ty = ty.to_const_ptr();
            }

            params.push(Param {
                def,
                name,
                ty,
                native_reference,
            });
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

    fn reader(rows: &[(&str, u16, ParamAttributes)], winrt: bool) -> Reader {
        let mut file = windows_metadata::writer::File::new("test");
        let mut flags =
            TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract;
        if winrt {
            flags |= TypeAttributes::WindowsRuntime;
        }
        file.TypeDef(
            "Test",
            "I",
            windows_metadata::writer::TypeDefOrRef::default(),
            flags,
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
        let def = match reader.with_full_name("Test", "I").next().unwrap() {
            Type::CppInterface(interface) => interface.def,
            Type::Interface(interface) => interface.def,
            _ => panic!(),
        };
        def.methods().next().unwrap()
    }

    #[test]
    fn sparse_out_of_order_params_follow_sequence() {
        let reader = reader(
            &[
                ("third", 3, ParamAttributes::In | ParamAttributes::Optional),
                ("return", 0, ParamAttributes::Out),
                ("first", 1, ParamAttributes::Out),
            ],
            false,
        );
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
                .eq([false, false, true])
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
    fn unspecified_does_not_imply_input() {
        let reader = reader(
            &[
                ("unspecified", 1, ParamAttributes::default()),
                (
                    "input_output",
                    2,
                    ParamAttributes::In | ParamAttributes::Out,
                ),
                ("output", 3, ParamAttributes::Out),
            ],
            false,
        );
        let signature = method(&reader).method_signature(&[], &reader);

        assert!(
            signature
                .params
                .iter()
                .map(Param::is_input_only)
                .eq([false, false, false])
        );
        assert!(
            signature
                .params
                .iter()
                .map(Param::has_direction)
                .eq([false, true, true])
        );
    }

    #[test]
    fn malformed_params_keep_all_signature_types_with_defaults() {
        let reader = reader(
            &[
                ("first", 1, ParamAttributes::Out),
                ("duplicate", 1, ParamAttributes::In),
            ],
            false,
        );
        let signature = method(&reader).method_signature(&[], &reader);

        assert_eq!(
            signature.params.iter().map(Param::name).collect::<Vec<_>>(),
            ["p0", "p1", "p2"]
        );
        assert_eq!(signature.params.len(), 3);
        assert!(signature.params.iter().all(|param| !param.has_direction()));
    }

    #[test]
    fn winrt_requires_explicit_parameter_directions() {
        for flags in [
            None,
            Some(ParamAttributes::default()),
            Some(ParamAttributes::Optional),
            Some(ParamAttributes::In | ParamAttributes::Out),
        ] {
            let mut rows = vec![
                ("first", 1, ParamAttributes::In),
                ("third", 3, ParamAttributes::Out),
            ];
            if let Some(flags) = flags {
                rows.push(("second", 2, flags));
            }
            let reader = reader(&rows, true);
            let error = std::panic::catch_unwind(|| {
                method(&reader).method_signature(&[], &reader);
            })
            .unwrap_err();
            let message = error.downcast_ref::<String>().unwrap();
            assert!(message.contains("Test.I.Method"), "{message}");
            assert!(message.contains("position 2"), "{message}");
            assert!(
                message.contains("requires exactly one of In or Out"),
                "{message}"
            );
        }
    }

    #[test]
    fn winrt_directions_use_sequence_not_return_flags() {
        let reader = reader(
            &[
                ("third", 3, ParamAttributes::In),
                ("return", 0, ParamAttributes::default()),
                ("second", 2, ParamAttributes::Out),
                ("first", 1, ParamAttributes::In),
            ],
            true,
        );
        let signature = method(&reader).method_signature(&[], &reader);
        assert!(
            signature
                .params
                .iter()
                .map(Param::is_input_only)
                .eq([true, false, true])
        );
    }

    #[test]
    #[should_panic(expected = "WinRT method `Test.I.Method` has invalid parameter metadata")]
    fn winrt_rejects_malformed_parameter_rows() {
        let reader = reader(
            &[
                ("first", 1, ParamAttributes::In),
                ("duplicate", 1, ParamAttributes::In),
            ],
            true,
        );
        method(&reader).method_signature(&[], &reader);
    }
}
