use super::*;
macros::table!(MethodDef);

pub struct MethodDefFlags(pub u32);

impl MethodDefFlags {
    pub fn special(&self) -> bool {
        self.0 & 0b1000_0000_0000 != 0
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum MethodCategory {
    Normal,
    Get,
    Set,
    Add,
    Remove,
}

impl MethodDef {
    pub fn flags(&self) -> MethodDefFlags {
        MethodDefFlags(self.reader.u32(self.row, 2))
    }

    pub fn params(&self) -> impl Iterator<Item = Param> + '_ {
        self.reader
            .list(self.row, TableIndex::Param, 5)
            .map(move |row| Param {
                reader: self.reader,
                row,
            })
    }

    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 3)
    }

    // TODO: find uses of MethodDef::blob and replace with MethodDef::signature?
    pub fn blob(&self) -> Blob {
        self.reader.blob(self.row, 4)
    }

    pub fn category(&self) -> MethodCategory {
        if self.flags().special() {
            let name = self.name();

            if name.starts_with("get") {
                MethodCategory::Get
            } else if name.starts_with("put") {
                MethodCategory::Set
            } else if name.starts_with("add") {
                MethodCategory::Add
            } else if name.starts_with("remove") {
                MethodCategory::Remove
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                MethodCategory::Normal
            }
        } else {
            MethodCategory::Normal
        }
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::MethodDef(*self).encode(),
            )
            .map(move |row| Attribute {
                reader: self.reader,
                row,
            })
    }

    pub fn impl_map(&self) -> Option<ImplMap> {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::ImplMap,
                1,
                MemberForwarded::MethodDef(*self).encode(),
            )
            .map(move |row| ImplMap {
                reader: self.reader,
                row,
            })
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

    pub fn dependencies(&self, generics: &[ElementType]) -> Vec<TypeDef> {
        self.signature(generics).dependencies()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_interface(namespace: &str, interface_name:&str) -> Interface {
        if let ElementType::Interface(value) = TypeReader::get()
            .resolve_type(namespace, interface_name) { value.clone() } else { unexpected!(); }
    }

    fn get_method(interface: &Interface, method: &str) -> MethodDef {
        interface.0.def.methods().find(|m|m.name() == method).unwrap()
    }

    #[test]
    fn test_method() {
        let i = get_interface("Windows.Foundation", "IStringable");
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
        let i = get_interface("Windows.Foundation.Collections", "IMap`2");
        let m = get_method(&i, "Lookup");

        let s = m.signature(&i.0.generics);
        assert_eq!(s.params.len(), 1);

        let r = s.return_type.unwrap();
        assert_eq!(r.kind.gen_name(Gen::Absolute).as_str(), "V");
        assert_eq!(r.pointers, 0);
        assert_eq!(r.by_ref, false);
        assert_eq!(r.is_const, false);
        assert_eq!(r.is_array, false);

        let p = &s.params[0];
        assert_eq!(p.param.name(), "key");
        assert_eq!(p.signature.kind.gen_name(Gen::Absolute).as_str(), "K");
        assert_eq!(p.signature.pointers, 0);
        assert_eq!(p.signature.by_ref, false);
        assert_eq!(p.signature.is_const, false);
        assert_eq!(p.signature.is_array, false);
    }
}
