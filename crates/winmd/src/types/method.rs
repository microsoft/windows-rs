use crate::case;
use crate::tables::*;
use crate::types::*;
use crate::TypeReader;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub kind: MethodKind,
    pub params: Vec<Param>,
    pub return_type: Option<Param>,
}

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

impl Param {
    pub fn to_stream(&self) -> TokenStream {
        quote! {}
    }

    pub fn to_abi_stream(&self, calling_namespace: &str) -> TokenStream {
        let tokens = self.kind.to_abi_stream(calling_namespace);

        if self.array {
            if self.input {
                quote! { u32, *const #tokens }
            } else if self.by_ref {
                quote! { *mut u32, *mut *mut #tokens }
            } else {
                quote! { u32, *mut #tokens }
            }
        } else if self.input {
            tokens
        } else {
            quote! { *mut #tokens }
        }
    }
}

impl Method {
    pub fn from_method_def(
        reader: &TypeReader,
        method: MethodDef,
        generics: &Vec<TypeKind>,
    ) -> Method {
        let (name, kind) = if method.flags(reader).special() {
            let name = method.name(reader);

            if name.starts_with("get") {
                (case::to_snake(&name[4..], MethodKind::Get), MethodKind::Get)
            } else if name.starts_with("put") {
                (case::to_snake(&name[4..], MethodKind::Set), MethodKind::Set)
            } else if name.starts_with("add") {
                (case::to_snake(&name[4..], MethodKind::Add), MethodKind::Add)
            } else if name.starts_with("remove") {
                (
                    case::to_snake(&name[7..], MethodKind::Remove),
                    MethodKind::Remove,
                )
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                ("invoke".to_owned(), MethodKind::Normal)
            }
        } else {
            (Method::name(reader, method), MethodKind::Normal)
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

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.return_type
            .iter()
            .chain(self.params.iter())
            .flat_map(|i| i.kind.dependencies())
            .collect()
    }

    fn name(reader: &TypeReader, method: MethodDef) -> String {
        if let Some(attribute) =
            method.find_attribute(reader, ("Windows.Foundation.Metadata", "OverloadAttribute"))
        {
            for (_, arg) in attribute.args(reader) {
                if let AttributeArg::String(name) = arg {
                    return case::to_snake(&name, MethodKind::Normal);
                }
            }
        }

        case::to_snake(method.name(reader), MethodKind::Normal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method((namespace, type_name): (&str, &str), method_name: &str) -> Method {
        let reader = &TypeReader::from_os();
        let def = reader.resolve_type_def((namespace, type_name));

        let t = match def.into_type(reader) {
            Type::Interface(t) => t,
            _ => panic!("Type not an interface"),
        };

        for interface in t.interfaces {
            for method in interface.methods {
                if method.name == method_name {
                    return method;
                }
            }
        }

        panic!("Method not found");
    }

    #[test]
    fn test_to_string() {
        let method = method(("Windows.Foundation", "IStringable"), "to_string");
        assert!(method.kind == MethodKind::Normal);
        assert!(method.params.is_empty());

        let param = method.return_type.as_ref().unwrap();
        assert!(param.kind == TypeKind::String);
    }

    #[test]
    fn test_map_changed() {
        let method = method(
            ("Windows.Foundation.Collections", "IObservableMap`2"),
            "map_changed",
        );

        assert!(method.kind == MethodKind::Add);
        assert!(method.params.len() == 1);

        let handler = &method.params[0];
        assert!(handler.array == false);
        assert!(handler.input == true);
        assert!(handler.by_ref == false);

        let handler = match &handler.kind {
            TypeKind::Delegate(delegate) => delegate,
            _ => panic!("Wrong type"),
        };

        assert!(
            handler.runtime_name()
                == "Windows.Foundation.Collections.MapChangedEventHandler`2<K, V>"
        );

        let token = method.return_type.as_ref().unwrap();
        assert!(token.array == false);
        assert!(token.input == false);
        assert!(token.by_ref == true);

        let token = match &token.kind {
            TypeKind::Struct(token) => token,
            _ => panic!("Wrong type"),
        };

        assert!(token.runtime_name() == "Windows.Foundation.EventRegistrationToken");
    }

    #[test]
    fn test_remove_map_changed() {
        let method = method(
            ("Windows.Foundation.Collections", "IObservableMap`2"),
            "remove_map_changed",
        );

        assert!(method.kind == MethodKind::Remove);
        assert!(method.params.len() == 1);

        let token = &method.params[0];
        assert!(token.array == false);
        assert!(token.input == true);
        assert!(token.by_ref == false);

        let token = match &token.kind {
            TypeKind::Struct(token) => token,
            _ => panic!("Wrong type"),
        };

        assert!(token.runtime_name() == "Windows.Foundation.EventRegistrationToken");
    }
}
