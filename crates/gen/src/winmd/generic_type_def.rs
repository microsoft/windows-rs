use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericTypeDef {
    pub def: TypeDef,
    pub generics: Vec<ElementType>,
}

impl GenericTypeDef {
    pub fn from_blob_with_generics(blob: &mut Blob, generics: &[ElementType]) -> Self {
        blob.read_unsigned();
        // TODO: add "read_type_def_or_ref" method to Blob reader.
        let def =
            TypeDefOrRef::decode(blob.reader, blob.read_unsigned(), blob.file_index).resolve();
        let mut args = Vec::with_capacity(blob.read_unsigned() as usize);

        for _ in 0..args.capacity() {
            args.push(ElementType::from_blob_with_generics(blob, generics));
        }

        GenericTypeDef {
            def,
            generics: args,
        }
    }

    pub fn interfaces(&self) -> impl Iterator<Item = Self> + '_ {
        self.def.interfaces().map(move |i| match i.interface() {
            TypeDefOrRef::TypeDef(def) => Self {
                def,
                generics: Vec::new(),
            },
            TypeDefOrRef::TypeRef(def) => Self {
                def: def.resolve(),
                generics: Vec::new(),
            },
            TypeDefOrRef::TypeSpec(def) => {
                let mut blob = def.sig();
                blob.read_unsigned();
                Self::from_blob_with_generics(&mut blob, &self.generics)
            }
        })
    }

    pub fn gen_name(&self, gen: Gen) -> TokenStream {
        self.format_name(gen, to_ident)
    }

    pub fn signature(&self) -> String {
        String::new()
    }
    //     match self.def.category() {
    //         TypeCategory::Interface => {

    //         }
    //         TypeCategory::Class => {
    //             let default = self.def.interfaces().find(|i| i.is_default()).expect("GenericTypeDef");

    //             format!(
    //                 "rc({}.{};{})",
    //                 self.def.namespace(),
    //                 self.def.name(),
    //                 default.interface_signature()
    //         }
    //         TypeCategory::Enum => {

    //         }
    //         TypeCategory::Struct => {

    //         }
    //         TypeCategory::Delegate => {

    //         }
    //         TypeCategory::Attribute => {

    //         }
    //         TypeCategory::Contract => {

    //         }
    //     }
    // }

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

impl From<ElementType> for GenericTypeDef {
    fn from(value: ElementType) -> Self {
        if let ElementType::TypeDef(def) = value {
            return def;
        }

        panic!("GenericTypeDef");
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

    #[test]
    fn test_interfaces() {
        let reader = TypeReader::get();
        let t: GenericTypeDef = reader
            .resolve_type("Windows.Foundation", "IAsyncOperation`1")
            .into();
        let i: Vec<GenericTypeDef> = t.interfaces().collect();
        assert_eq!(i.len(), 1);

        assert_eq!(
            i[0].gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: IAsyncInfo"
        );
    }

    #[test]
    fn test_generic_interfaces() {
        let reader = TypeReader::get();
        let t: GenericTypeDef = reader
            .resolve_type("Windows.Foundation.Collections", "IMap`2")
            .into();
        let i: Vec<GenericTypeDef> = t.interfaces().collect();
        assert_eq!(i.len(), 1);

        assert_eq!(
            i[0].gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IIterable :: < windows :: foundation :: collections :: IKeyValuePair :: < K , V > >"
        );
    }
}
