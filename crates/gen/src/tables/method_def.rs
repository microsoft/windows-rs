use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MethodDef(pub Row);

impl From<Row> for MethodDef {
    fn from(row: Row) -> Self {
        Self(row)
    }
}

impl MethodDef {
    pub fn is_special(&self) -> bool {
        self.0.u32(2) & 0b1000_0000_0000 != 0
    }

    pub fn params(&self) -> impl Iterator<Item = Param> {
        self.0.list(5, TableIndex::Param).map(Param)
    }

    pub fn name(&self) -> &'static str {
        self.0.str(3)
    }

    pub fn parent(&self) -> TypeDef {
        let row = self.0.file.upper_bound_of(
            TableIndex::TypeDef,
            0,
            self.0.file.tables[TableIndex::TypeDef as usize].row_count,
            5,
            self.0.row + 1,
        ) - 1;

        Row::new(row, TableIndex::TypeDef, self.0.file).into()
    }

    pub fn rust_name(&self) -> String {
        let name = self.name();

        if self.is_special() {
            if name.starts_with("get") {
                name[4..].to_string()
            } else if name.starts_with("put") {
                format!("Set{}", &name[4..])
            } else if name.starts_with("add") {
                name[4..].to_string()
            } else if name.starts_with("remove") {
                format!("Remove{}", &name[7..])
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                "Invoke".to_string()
            }
        } else {
            for attribute in self.attributes() {
                if attribute.name() == "OverloadAttribute" {
                    for (_, arg) in attribute.args() {
                        if let ConstantValue::String(name) = arg {
                            return name;
                        }
                    }
                }
            }

            name.to_string()
        }
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0
            .file
            .attributes(HasAttribute::MethodDef(self.clone()))
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn is_deprecated(&self) -> bool {
        self.has_attribute("DeprecatedAttribute")
    }

    pub fn impl_map(&self) -> Option<ImplMap> {
        self.0
            .file
            .equal_range(
                TableIndex::ImplMap,
                1,
                MemberForwarded::MethodDef(self.clone()).encode(),
            )
            .map(ImplMap)
            .next()
    }

    pub fn signature(&self, generics: &[ElementType]) -> MethodSignature {
        let reader = TypeReader::get();
        let params = self.params();

        let mut blob = self.0.blob(4);
        blob.read_unsigned();
        blob.read_unsigned(); // parameter count

        let return_type = reader.signature_from_blob(&mut blob, generics);

        let params = params
            .filter_map(|param| {
                if param.sequence() == 0 {
                    None
                } else {
                    Some(MethodParam {
                        param,
                        signature: reader
                            .signature_from_blob(&mut blob, generics)
                            .expect("MethodDef"),
                    })
                }
            })
            .collect();

        MethodSignature {
            params,
            return_type,
        }
    }

    pub fn dependencies(&self) -> Vec<TypeEntry> {
        self.signature(&[]).dependencies(TypeInclude::Minimal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_method(interface: &types::Interface, method: &str) -> MethodDef {
        interface.0.methods().find(|m| m.name() == method).unwrap()
    }

    #[test]
    fn test_method() {
        let i =
            TypeReader::get().expect_type_def(TypeName::new("Windows.Foundation", "IStringable"));
        let i = types::Interface(i);
        let m = get_method(&i, "ToString");
        assert_eq!(m.name(), "ToString");

        let s = m.signature(&[]);
        assert_eq!(s.params.len(), 0);

        let s = s.return_type.unwrap();
        assert!(s.kind == ElementType::String);
        assert_eq!(s.pointers, 0);
        assert!(!s.by_ref);
        assert!(!s.is_const);
        assert!(!s.is_array);
    }

    #[test]
    fn test_generic() {
        let i = TypeReader::get()
            .expect_type_def(TypeName::new("Windows.Foundation.Collections", "IMap`2"));
        let i = types::Interface(i.with_generics());
        let m = get_method(&i, "Lookup");

        let s = m.signature(&i.0.generics);
        assert_eq!(s.params.len(), 1);

        let r = s.return_type.unwrap();
        assert_eq!(gen_name(&r.kind, &Gen::Absolute).as_str(), "V");
        assert_eq!(r.pointers, 0);
        assert!(!r.by_ref);
        assert!(!r.is_const);
        assert!(!r.is_array);

        let p = &s.params[0];
        assert_eq!(p.param.name(), "key");
        assert_eq!(gen_name(&p.signature.kind, &Gen::Absolute).as_str(), "K");
        assert_eq!(p.signature.pointers, 0);
        assert!(!p.signature.by_ref);
        assert!(!p.signature.is_const);
        assert!(!p.signature.is_array);
    }
}
