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

        // A property-key constant is spelled `#[guid(0x…)] const NAME: PROPERTYKEY = pid;`:
        // the `fmtid` rides on the `#[guid]` attribute and the value is the `pid`. It is
        // distinguished from an ordinary GUID constant (whose value is the GUID itself,
        // written inline) by carrying a `#[guid]` attribute on a non-GUID struct type.
        let has_guid_attr = item.attrs.iter().any(|a| a.path().is_ident("guid"));

        match &ty {
            // A GUID-typed constant is encoded as a `GuidAttribute` blob on the field
            // (the win32metadata representation bindgen reads), not an ECMA `Constant`.
            // Match both the WinRT `System.Guid` and the faithful Win32 `GUID` struct
            // (guiddef.h), which resolves to the closure's own `…Win32.GUID`.
            windows_metadata::Type::ValueName(tn)
                if tn == ("System", "Guid") || tn.name == "GUID" =>
            {
                self.encode_const_guid(&ty, item, &name)?;
            }
            _ if has_guid_attr => {
                self.encode_const_property_key(&ty, item, &name)?;
            }
            _ => self.encode_const_value(&ty, item, &name)?,
        }

        Ok(())
    }

    /// Encodes a `PROPERTYKEY`/`DEVPROPKEY` constant as a field carrying a `GuidAttribute`
    /// (the `fmtid`) plus an ordinary `u32` `Constant` (the `pid`). This reuses the two
    /// structured primitives that already round-trip faithfully instead of a bespoke
    /// struct-initializer string.
    fn encode_const_property_key(
        &mut self,
        ty: &windows_metadata::Type,
        item: &Const,
        name: &str,
    ) -> Result<(), Error> {
        let expr = item.expr.as_ref().ok_or_else(|| {
            self.error(&item.name, "property key constant requires a `pid` value")
        })?;
        let pid = self.encode_lit_uint(expr, 32)? as u32;

        let field = self.output.Field(
            name,
            ty,
            metadata::FieldAttributes::Public | metadata::FieldAttributes::Static,
        );

        self.encode_guid_pseudo_attrs(metadata::writer::HasAttribute::Field(field), &item.attrs)?;
        self.output.Constant(
            metadata::writer::HasConstant::Field(field),
            &metadata::Value::U32(pid),
        );

        self.encode_attrs(
            metadata::writer::HasAttribute::Field(field),
            &item.attrs,
            &["guid", "no_guid"],
        )?;

        if let Some(arch_bits) = self.read_arch(&item.attrs)? {
            self.emit_arch_attribute(metadata::writer::HasAttribute::Field(field), arch_bits);
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

        if let Some(arch_bits) = self.read_arch(&item.attrs)? {
            self.emit_arch_attribute(metadata::writer::HasAttribute::Field(field), arch_bits);
        }

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
