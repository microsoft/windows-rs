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
        self.0.file.attributes(HasAttribute::MethodDef(self.clone()))
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn is_deprecated(&self) -> bool {
        self.has_attribute("DeprecatedAttribute")
    }

    pub fn static_lib(&self) -> Option<String> {
        self.attributes()
            .filter_map(|attribute| match attribute.name() {
                "StaticLibraryAttribute" => {
                    let args = attribute.args();
                    match &args[0].1 {
                        ConstantValue::String(value) => Some(value.clone()),
                        _ => None,
                    }
                }
                _ => None,
            })
            .next()
    }

    pub fn impl_map(&self) -> Option<ImplMap> {
        self.0.file.equal_range(TableIndex::ImplMap, 1, MemberForwarded::MethodDef(self.clone()).encode()).map(ImplMap).next()
    }

    pub fn signature(&self, generics: &[ElementType]) -> MethodSignature {
        let reader = TypeReader::get();
        let params = self.params();

        let mut blob = self.0.blob(4);
        blob.read_unsigned();
        blob.read_unsigned(); // parameter count

        let return_sig = reader.signature_from_blob(&mut blob, None, generics);
        let mut return_param = None;

        let params = params
            .filter_map(|param| {
                if param.sequence() == 0 {
                    return_param = Some(param);
                    None
                } else {
                    Some(MethodParam { param, signature: reader.signature_from_blob(&mut blob, None, generics).expect("MethodDef") })
                }
            })
            .collect();

        MethodSignature { params, return_sig, return_param }
    }

    pub fn include_dependencies(&self) {
        self.signature(&[]).include_dependencies(TypeInclude::Minimal)
    }
}
