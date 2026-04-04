use super::guid;
use super::*;

#[derive(Debug)]
pub struct Const {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub ty: syn::Type,
    pub expr: Option<syn::Expr>,
}

impl syn::parse::Parse for Const {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<syn::Token![const]>()?;
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty = input.parse()?;
        let expr = if input.peek(syn::Token![=]) {
            input.parse::<syn::Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        input.parse::<syn::Token![;]>()?;

        Ok(Self {
            attrs,
            name,
            ty,
            expr,
        })
    }
}

impl Encoder<'_> {
    pub fn encode_const(&mut self, item: &Const) -> Result<(), Error> {
        let name = item.name.to_string();
        let ty = self.encode_type(&item.ty)?;

        match &ty {
            windows_metadata::Type::ValueName(tn) if tn == ("System", "Guid") => {
                self.encode_const_guid(&ty, item, &name)?;
            }
            _ => self.encode_const_value(&ty, item, &name)?,
        }

        Ok(())
    }

    fn encode_const_value(
        &mut self,
        ty: &windows_metadata::Type,
        item: &Const,
        name: &str,
    ) -> Result<(), Error> {
        let field = self.output.Field(
            name,
            ty,
            metadata::FieldAttributes::Public
                | metadata::FieldAttributes::Static
                | metadata::FieldAttributes::Literal,
        );

        if let Some(expr) = &item.expr {
            let value = self.encode_value(ty, expr)?;
            self.output
                .Constant(metadata::writer::HasConstant::Field(field), &value);
        }

        self.encode_attrs(
            metadata::writer::HasAttribute::Field(field),
            &item.attrs,
            &[],
        )?;

        Ok(())
    }

    fn encode_const_guid(
        &mut self,
        ty: &windows_metadata::Type,
        item: &Const,
        name: &str,
    ) -> Result<(), Error> {
        let expr = item
            .expr
            .as_ref()
            .ok_or_else(|| self.error(&item.name, "GUID constant requires a value"))?;
        let value: u128 = self.encode_lit_int(expr)?;
        let field = self.output.Field(
            name,
            ty,
            metadata::FieldAttributes::Public | metadata::FieldAttributes::Static,
        );

        let (d1, d2, d3, d4) = guid::u128_to_guid(value);
        guid::emit_guid_attribute(
            self.output,
            metadata::writer::HasAttribute::Field(field),
            d1,
            d2,
            d3,
            d4,
        );

        Ok(())
    }
}
