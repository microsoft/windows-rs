use super::*;

#[derive(Debug, Clone)]
pub struct Union {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub body: RecordBody,
}

impl syn::parse::Parse for Union {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let _: syn::Token![union] = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let body = parse_record_body(&content)?;

        Ok(Self { attrs, name, body })
    }
}

impl Encoder<'_> {
    pub fn encode_union(&mut self, item: &Union) -> Result<(), Error> {
        let name = item.name.to_string();
        self.encode_record(&name, false, true, &item.body, &item.attrs, None)?;
        Ok(())
    }
}
