use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericType {
    pub def: TypeDef,
    pub generics: Vec<ElementType>,
    pub is_default: bool, // TODO: this should not be stored here (since it doesn't apply to all uses of GenericType)
}

impl GenericType {
    pub fn from_blob(blob: &mut Blob, generics: &[ElementType]) -> Self {
        blob.read_unsigned();
        // TODO: add "read_type_def_or_ref" method to Blob reader.
        let def =
            TypeDefOrRef::decode(blob.reader, blob.read_unsigned(), blob.file_index).resolve();
        let mut args = Vec::with_capacity(blob.read_unsigned() as usize);

        for _ in 0..args.capacity() {
            args.push(ElementType::from_blob(blob, generics));
        }

        Self {
            def,
            generics: args,
            is_default: false,
        }
    }

    pub fn from_type_def(def: TypeDef, generics: Vec<ElementType>) -> Self {
        if generics.is_empty() {
            let generics = def
                .generics()
                .map(|generic| ElementType::GenericParam(generic))
                .collect();

            Self {
                def,
                generics,
                is_default: false,
            }
        } else {
            Self {
                def,
                generics,
                is_default: false,
            }
        }
    }

    // TODO: return a pair of (GenericType, bool) to carry the "is_default" outside of GenericType
    pub fn interfaces(&self) -> impl Iterator<Item = Interface> + '_ {
        self.def.interfaces().map(move |i| {
            let is_default = i.is_default();

            Interface(match i.interface() {
                TypeDefOrRef::TypeDef(def) => Self {
                    def,
                    generics: Vec::new(),
                    is_default,
                },
                TypeDefOrRef::TypeRef(def) => Self {
                    def: def.resolve(),
                    generics: Vec::new(),
                    is_default,
                },
                TypeDefOrRef::TypeSpec(def) => {
                    let mut blob = def.blob();
                    blob.read_unsigned();
                    let mut interface = Self::from_blob(&mut blob, &self.generics);
                    interface.is_default = i.is_default();
                    interface
                }
            })
        })
    }

    pub fn gen_name(&self, gen: Gen) -> TokenStream {
        self.format_name(gen, to_ident)
    }

    pub fn gen_abi_name(&self, gen: Gen) -> TokenStream {
        self.format_name(gen, to_abi_ident)
    }

    pub fn interface_signature(&self) -> String {
        let guid = self.def.guid();

        if self.generics.is_empty() {
            format!("{{{:#?}}}", guid)
        } else {
            let mut result = format!("pinterface({{{:#?}}}", guid);

            for generic in &self.generics {
                result.push(';');
                result.push_str(&generic.signature());
            }

            result.push(')');
            result
        }
    }



    fn format_name<F>(&self, gen: Gen, format_name: F) -> TokenStream
    where
        F: FnOnce(&str) -> Ident,
    {
        let name = self.def.name();
        let namespace = gen.namespace(self.def.namespace());

        if self.generics.is_empty() {
            let name = format_name(name);
            quote! { #namespace#name }
        } else {
            let colon_separated = if namespace.as_str().is_empty() {
                quote! {}
            } else {
                quote! { :: }
            };

            let name = format_name(&name[..name.len() - 2]);
            let generics = self.generics.iter().map(|g| g.gen_name(gen));
            quote! { #namespace#name#colon_separated<#(#generics),*> }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic() {
        let reader = TypeReader::get();
        let t = reader.resolve_type("Windows.Foundation", "IAsyncOperation`1");
        assert_eq!(
            t.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: IAsyncOperation :: < TResult >"
        );
        assert_eq!(
            t.gen_name(Gen::Relative("Windows.Foundation")).as_str(),
            "IAsyncOperation < TResult >"
        );
    }
}
