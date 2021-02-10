use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericTypeDef {
    pub def: TypeDef,
    pub generics: Vec<ElementType>,
}

impl GenericTypeDef {
    pub fn gen_name(&self, gen: Gen) -> TokenStream {
        self.format_name(gen, to_ident)
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
