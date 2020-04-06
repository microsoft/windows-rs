use crate::tables::*;
use crate::types::*;
use crate::Reader;

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Param>,
    pub kind: MethodKind,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MethodKind {
    Normal,
    Get,
    Set,
    Add,
    Remove,
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub kind: TypeKind,
    pub array: bool,
    pub input: bool,
    pub by_ref: bool,
}

impl Method {
    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.return_type
            .iter()
            .chain(self.params.iter())
            .flat_map(|i| i.kind.dependencies())
            .collect()
    }

    fn method_name(reader: &Reader, method: MethodDef) -> String {
        if let Some(attribute) =
            method.find_attribute(reader, ("Windows.Foundation.Metadata", "OverloadAttribute"))
        {
            for (_, arg) in attribute.args(reader) {
                if let AttributeArg::String(name) = arg {
                    return name;
                }
            }
        }

        method.name(reader).to_string()
    }

    pub fn from_method_def(reader: &Reader, method: MethodDef, generics: &[TypeKind]) -> Method {
        let (name, kind) = if method.flags(reader).special() {
            let name = method.name(reader);

            if name.starts_with("get") {
                (name[4..].to_string(), MethodKind::Get)
            } else if name.starts_with("put") {
                (name[4..].to_string(), MethodKind::Set)
            } else if name.starts_with("add") {
                (name[4..].to_string(), MethodKind::Add)
            } else if name.starts_with("remove") {
                (name[7..].to_string(), MethodKind::Remove)
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                (name.to_string(), MethodKind::Normal)
            }
        } else {
            (Method::method_name(reader, method), MethodKind::Normal)
        };

        let mut blob = method.sig(reader);

        if blob.read_unsigned() & 0x10 != 0 {
            blob.read_unsigned();
        }

        let param_count = blob.read_unsigned();
        blob.read_modifiers();
        blob.read_expected(0x10);

        let return_type = if blob.read_expected(0x01) {
            None
        } else {
            let name = String::new();
            let array = blob.peek_unsigned().0 == 0x1D;
            let kind = TypeKind::from_blob(&mut blob, generics);
            let input = false;
            let by_ref = true;
            Some(Param {
                name,
                kind,
                array,
                input,
                by_ref,
            })
        };

        let mut params = Vec::with_capacity(param_count as usize);

        for param in method.params(reader) {
            if return_type.is_none() || param.sequence(reader) != 0 {
                let name = param.name(reader).to_string();
                let input = param.flags(reader).input();

                blob.read_modifiers();
                let by_ref = blob.read_expected(0x10);
                let array = blob.peek_unsigned().0 == 0x1D;
                let kind = TypeKind::from_blob(&mut blob, generics);

                params.push(Param {
                    name,
                    kind,
                    array,
                    input,
                    by_ref,
                });
            }
        }

        Method {
            name,
            kind,
            params,
            return_type,
        }
    }
}
