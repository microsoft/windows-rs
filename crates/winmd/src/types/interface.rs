use crate::codes::*;
use crate::tables::*;
use crate::types::*;
use crate::TypeReader;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Interface {
    pub name: TypeName,
    pub guid: TypeGuid,
    pub methods: Vec<Method>,
    // pub default: bool,
    // pub exclusive: bool,
    // pub constructors: bool,
    // pub statics: bool,
    // pub overrides: bool,
    pub interfaces: Vec<Interface>,
}

impl Interface {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let guid = TypeGuid::from_args(
            def.attribute(reader, ("Windows.Foundation.Metadata", "GuidAttribute"))
                .args(reader),
        );
        let methods = def
            .methods(reader)
            .map(|method| Method::from_method_def(reader, method, &name.generics))
            .collect();
        let interfaces = Vec::new();
        Self {
            name,
            guid,
            methods,
            interfaces,
        }
    }

    fn from_type_ref(reader: &TypeReader, type_ref: TypeRef) -> Self {
        Self::from_type_def(reader, type_ref.resolve(reader))
    }

    fn from_type_spec(reader: &TypeReader, spec: TypeSpec) -> Self {
        let name = TypeName::from_type_spec(reader, spec);
        let guid = TypeGuid::new(); // TODO: Generate generic guid specialization
        let methods = Vec::new();
        let interfaces = Vec::new();
        Self {
            name,
            guid,
            methods,
            interfaces,
        }
    }

    fn from_type_def_or_ref(reader: &TypeReader, code: TypeDefOrRef) -> Self {
        match code {
            TypeDefOrRef::TypeDef(value) => Self::from_type_def(reader, value),
            TypeDefOrRef::TypeRef(value) => Self::from_type_ref(reader, value),
            TypeDefOrRef::TypeSpec(value) => Self::from_type_spec(reader, value),
        }
    }

    pub fn from_interface_impl(reader: &TypeReader, key: InterfaceImpl) -> Self {
        // TODO: flip default/exclusive/overridable bits as needed
        Self::from_type_def_or_ref(reader, key.interface(reader))
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.interfaces
            .iter()
            .flat_map(|i| i.name.dependencies())
            .chain(self.methods.iter().flat_map(|m| m.dependencies()))
            .collect()
    }

    pub fn to_stream(&self) -> TokenStream {
        let name = self.name.ident();
        let phantoms = self.name.phantoms();
        let constraints = self.name.constraints();
        let projected_methods = TokenStream::new();

        quote! {
            #[repr(C)]
            #[derive(Default, Clone)]
            pub struct #name where #constraints {
                ptr: winrt::IUnknown,
                #phantoms
            }
            impl<#constraints> #name {
                #projected_methods
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::method::MethodKind;
    use crate::types::type_guid::GuidConstant;

    #[test]
    fn can_read_interface_from_reader() {
        let winmd_files = crate::load_winmd::from_os();
        let reader = &TypeReader::new(winmd_files);
        let def = reader.resolve(("Windows.Foundation", "IStringable"));
        let t = def.into_type(reader);

        let name = t.name();
        assert!(name.namespace == "Windows.Foundation");
        assert!(name.name == "IStringable");
        assert!(name.generics.is_empty());

        assert!(name.def == def);

        let t = match t {
            Type::Interface(t) => t,
            _ => panic!("Wrong type"),
        };

        assert!(t.methods.len() == 1);
        let method = &t.methods[0];
        assert!(method.name == "to_string");
        assert!(method.kind == MethodKind::Normal);

        assert!(method.params.is_empty());
        let param = method.return_type.as_ref().unwrap();
        assert!(param.kind == TypeKind::String);

        let guid = &t.guid;
        assert!(guid.0[0] == GuidConstant::U32(0x96369F54));
        assert!(guid.0[1] == GuidConstant::U16(0x8EB6));
        assert!(guid.0[2] == GuidConstant::U16(0x48F0));
        assert!(guid.0[3] == GuidConstant::U8(0xAB));
        assert!(guid.0[4] == GuidConstant::U8(0xCE));
        assert!(guid.0[5] == GuidConstant::U8(0xC1));
        assert!(guid.0[6] == GuidConstant::U8(0xB2));
        assert!(guid.0[7] == GuidConstant::U8(0x11));
        assert!(guid.0[8] == GuidConstant::U8(0xE6));
        assert!(guid.0[9] == GuidConstant::U8(0x27));
        assert!(guid.0[10] == GuidConstant::U8(0xC3));
    }

    #[test]
    fn can_read_generic_interface_from_reader() {
        let winmd_files = crate::load_winmd::from_os();
        let reader = &TypeReader::new(winmd_files);
        let def = reader.resolve(("Windows.Foundation.Collections", "IObservableMap`2"));
        let t = def.into_type(reader);
        let name = t.name();

        assert!(name.namespace == "Windows.Foundation.Collections");
        assert!(name.name == "IObservableMap`2");
        assert!(name.generics.len() == 2);
        assert!(name.generics[0] == TypeKind::Generic("K".to_string()));
        assert!(name.generics[1] == TypeKind::Generic("V".to_string()));

        assert!(name.def == def);

        let t = match t {
            Type::Interface(t) => t,
            _ => panic!("Wrong type"),
        };

        assert!(t.methods.len() == 2);

        let method = &t.methods[0];
        assert!(method.name == "map_changed");
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

        assert!(handler.namespace == "Windows.Foundation.Collections");
        assert!(handler.name == "MapChangedEventHandler`2");
        assert!(handler.generics.len() == 2);
        assert!(handler.generics[0] == TypeKind::Generic("K".to_string()));
        assert!(handler.generics[1] == TypeKind::Generic("V".to_string()));
        assert!(
            handler.def
                == reader.resolve(("Windows.Foundation.Collections", "MapChangedEventHandler`2"))
        );

        let token = method.return_type.as_ref().unwrap();
        assert!(token.array == false);
        assert!(token.input == false);
        assert!(token.by_ref == true);

        let token = match &token.kind {
            TypeKind::Struct(token) => token,
            _ => panic!("Wrong type"),
        };

        assert!(token.namespace == "Windows.Foundation");
        assert!(token.name == "EventRegistrationToken");
        assert!(token.generics.is_empty());
        assert!(token.def == reader.resolve(("Windows.Foundation", "EventRegistrationToken")));

        let method = &t.methods[1];
        assert!(method.name == "remove_map_changed");
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

        assert!(token.namespace == "Windows.Foundation");
        assert!(token.name == "EventRegistrationToken");
        assert!(token.generics.is_empty());
        assert!(token.def == reader.resolve(("Windows.Foundation", "EventRegistrationToken")));

        // TODO: make sure all required interfaces are properly specialized
    }
}
