use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppEnum {
    pub def: TypeDef,
    pub name: &'static str,
    pub enclosing: Option<TypeName>,
}

impl Ord for CppEnum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(other.name)
    }
}

impl PartialOrd for CppEnum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppEnum {
    pub fn type_name(&self) -> TypeName {
        TypeName(
            self.enclosing
                .map_or(self.def.namespace(), |owner| owner.namespace()),
            self.name,
        )
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        if let Some(owner) = self.enclosing {
            let namespace = config.write_namespace(owner);
            let name = to_ident(self.name);
            return quote! { #namespace #name };
        }
        self.type_name().write(config, &[])
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        self.write_with_cfg(config, &quote! {}, self.def.arches())
    }

    pub fn write_with_cfg(&self, config: &Config, cfg: &TokenStream, arches: i32) -> TokenStream {
        let own_arches = if self.def.arches() == arches {
            quote! {}
        } else {
            write_arches(self.def)
        };
        let cfg = &quote! { #cfg #own_arches };
        let tn = self.type_name();
        let is_scoped = self.def.has_attribute("ScopedEnumAttribute");

        // Unscoped C enums are bare integer aliases; scoped enums keep newtype projection.
        if !is_scoped {
            let own_cfg = write_simple_cfg(self, config);
            if self.enclosing.is_some() {
                let name = to_ident(self.name);
                let underlying = self
                    .def
                    .underlying_type_ext(config.reader)
                    .write_name(config);
                return quote! { #cfg #own_cfg pub type #name = #underlying; };
            }
            return config.write_cpp_handle(&Type::CppEnum(self.clone()), self.def, &own_cfg);
        }

        let name = to_ident(tn.name());
        let underlying_type = self
            .def
            .underlying_type_ext(config.reader)
            .write_name(config);

        let mut derive = DeriveWriter::new(config, tn);
        derive.extend(["Copy", "Clone"]);

        if config.bindgen.style.derive_std_traits() {
            derive.extend(["Default", "Debug", "PartialEq", "Eq"]);
        }

        let fields = write_enum_constants(self.def, config);
        let fields = if fields.is_empty() {
            quote! {}
        } else {
            quote! { #cfg impl #name { #(#fields)* } }
        };

        let flags = if config.bindgen.style.is_sys() || !self.def.has_attribute("FlagsAttribute") {
            quote! {}
        } else {
            write_enum_flags(&name, cfg)
        };

        quote! {
            #cfg
            #[repr(transparent)]
            #derive
            pub struct #name(pub #underlying_type);
            #fields
            #flags
        }
    }

    pub fn size(&self, reader: &Reader) -> usize {
        self.def.underlying_type_ext(reader).size(reader)
    }

    pub fn align(&self, reader: &Reader) -> usize {
        self.def.underlying_type_ext(reader).align(reader)
    }
}

impl Dependencies for CppEnum {
    fn combine(&self, _dependencies: &mut TypeMap, _reader: &Reader) {}
}
