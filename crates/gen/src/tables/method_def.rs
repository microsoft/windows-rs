use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MethodDef(pub Row);

impl MethodDef {
    pub fn flags(&self) -> MethodFlags {
        MethodFlags(self.0.u32(2))
    }

    pub fn params(&self) -> impl Iterator<Item = Param> + '_ {
        self.0.list(5, TableIndex::Param).map(Param)
    }

    pub fn name(&self) -> &'static str {
        self.0.str(3)
    }

    pub fn parent(&self) -> TypeDef {
        // TODO: stick this in TypeReader
        let row = self.0.file.upper_bound_of(
            TableIndex::TypeDef,
            0,
            self.0.file.tables[TableIndex::TypeDef as usize].row_count,
            5,
            self.0.row + 1,
        ) - 1;

        TypeDef(Row::new(row, TableIndex::TypeDef, self.0.file))
    }

    pub fn rust_name(&self) -> String {
        if self.flags().special() {
            let name = self.name();

            if name.starts_with("get") {
                to_snake(&name[4..])
            } else if name.starts_with("put") {
                let mut name = to_snake(name);
                name.replace_range(..3, "set");
                name
            } else if name.starts_with("add") {
                to_snake(&name[4..])
            } else if name.starts_with("remove") {
                to_snake(name)
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                "invoke".to_owned()
            }
        } else {
            for attribute in self.attributes() {
                if attribute.name() == "OverloadAttribute" {
                    for (_, arg) in attribute.args() {
                        if let ConstantValue::String(name) = arg {
                            return to_snake(&name);
                        }
                    }
                }
            }

            to_snake(self.name())
        }
    }

    // TODO: find uses of MethodDef::blob and replace with MethodDef::signature?
    pub fn blob(&self) -> Blob {
        self.0.blob(4)
    }

    pub fn kind(&self) -> MethodKind {
        if self.flags().special() {
            let name = self.name();

            if name.starts_with("get") {
                MethodKind::Get
            } else if name.starts_with("put") {
                MethodKind::Set
            } else if name.starts_with("add") {
                MethodKind::Add
            } else if name.starts_with("remove") {
                MethodKind::Remove
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                MethodKind::Normal
            }
        } else {
            MethodKind::Normal
        }
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        self.0
            .file
            .equal_range(
                TableIndex::CustomAttribute,
                0,
                HasAttribute::MethodDef(*self).encode(),
            )
            .map(Attribute)
    }

    pub fn impl_map(&self) -> Option<ImplMap> {
        self.0
            .file
            .equal_range(
                TableIndex::ImplMap,
                1,
                MemberForwarded::MethodDef(*self).encode(),
            )
            .map(ImplMap)
            .next()
    }

    pub fn signature(&self, generics: &[ElementType]) -> MethodSignature {
        let params = self.params();

        let mut blob = self.blob();
        blob.read_unsigned();
        blob.read_unsigned(); // parameter count

        let return_type = Signature::from_blob(&mut blob, generics);

        let params = params
            .filter_map(|param| {
                if param.sequence() == 0 {
                    None
                } else {
                    Some(MethodParam {
                        param,
                        signature: Signature::from_blob(&mut blob, generics).expect("MethodDef"),
                    })
                }
            })
            .collect();

        MethodSignature {
            params,
            return_type,
        }
    }

    pub fn dependencies(&self, generics: &[ElementType]) -> Vec<ElementType> {
        self.signature(generics).dependencies()
    }
}

impl std::fmt::Debug for MethodDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_method(interface: &types::Interface, method: &str) -> MethodDef {
        interface
            .0
            .def
            .methods()
            .find(|m| m.name() == method)
            .unwrap()
    }

    #[test]
    fn test_method() {
        let i = TypeReader::get_interface("Windows.Foundation", "IStringable");
        let m = get_method(&i, "ToString");
        assert_eq!(m.name(), "ToString");

        let s = m.signature(&[]);
        assert_eq!(s.params.len(), 0);

        let s = s.return_type.unwrap();
        assert_eq!(s.kind, ElementType::String);
        assert_eq!(s.pointers, 0);
        assert_eq!(s.by_ref, false);
        assert_eq!(s.is_const, false);
        assert_eq!(s.is_array, false);
    }

    #[test]
    fn test_generic() {
        let i = TypeReader::get_interface("Windows.Foundation.Collections", "IMap`2");
        let m = get_method(&i, "Lookup");

        let s = m.signature(&i.0.generics);
        assert_eq!(s.params.len(), 1);

        let r = s.return_type.unwrap();
        assert_eq!(
            r.kind
                .gen_name(&Gen::absolute(&TypeTree::from_namespace("")))
                .as_str(),
            "V"
        );
        assert_eq!(r.pointers, 0);
        assert_eq!(r.by_ref, false);
        assert_eq!(r.is_const, false);
        assert_eq!(r.is_array, false);

        let p = &s.params[0];
        assert_eq!(p.param.name(), "key");
        assert_eq!(
            p.signature
                .kind
                .gen_name(&Gen::absolute(&TypeTree::from_namespace("")))
                .as_str(),
            "K"
        );
        assert_eq!(p.signature.pointers, 0);
        assert_eq!(p.signature.by_ref, false);
        assert_eq!(p.signature.is_const, false);
        assert_eq!(p.signature.is_array, false);
    }
}
