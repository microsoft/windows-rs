use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppEnum {
    pub def: TypeDef,
}

impl Ord for CppEnum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.def.name().cmp(other.def.name())
    }
}

impl PartialOrd for CppEnum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppEnum {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        let tn = self.def.type_name();
        let is_scoped = self.def.has_attribute("ScopedEnumAttribute");

        if !is_scoped && config.bindgen.style.emit_bare_typedef() {
            return config.write_cpp_handle(self.def);
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

        let fields = if is_scoped {
            let fields = write_enum_constants(self.def, config);

            quote! {
                impl #name {
                    #(#fields)*
                }
            }
        } else {
            quote! {}
        };

        let flags = if config.bindgen.style.is_sys() || !self.def.has_attribute("FlagsAttribute") {
            quote! {}
        } else {
            write_enum_flags(&name)
        };

        quote! {
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
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        if let Some(attribute) = self.def.find_attribute("AlsoUsableForAttribute") {
            if let Some((_, Value::Utf8(type_name))) = attribute.value().first() {
                reader
                    .unwrap_full_name(self.def.namespace(), type_name)
                    .combine(dependencies, reader);
            }
        }
    }
}
