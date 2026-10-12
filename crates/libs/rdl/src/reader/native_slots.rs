use super::*;

pub struct Slot {
    pub index: i32,
    pub reuse: Option<syn::Path>,
}

impl syn::parse::Parse for Slot {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let index: syn::LitInt = input.parse()?;
        let index = index.base10_parse::<i32>()?;
        let reuse = if !input.is_empty() {
            input.parse::<syn::Token![,]>()?;
            let option: syn::Ident = input.parse()?;
            if option != "reuse" {
                return Err(input.error("unknown native slot option"));
            }
            input.parse::<syn::Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self { index, reuse })
    }
}

struct Vtable {
    size: i32,
    inherited: i32,
}

impl syn::parse::Parse for Vtable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut size = None;
        let mut inherited = None;
        while !input.is_empty() {
            let option: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            let value: syn::LitInt = input.parse()?;
            let value = value.base10_parse::<i32>()?;
            match option.to_string().as_str() {
                "size" if size.is_none() => size = Some(value),
                "inherited" if inherited.is_none() => inherited = Some(value),
                _ => return Err(input.error("unknown or duplicate native vtable option")),
            }
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            size: size.ok_or_else(|| input.error("native vtable size is required"))?,
            inherited: inherited
                .ok_or_else(|| input.error("native inherited count is required"))?,
        })
    }
}

impl Encoder<'_> {
    pub fn encode_native_vtable(
        &mut self,
        target: metadata::writer::HasAttribute,
        attrs: &[syn::Attribute],
    ) -> Result<bool, Error> {
        let mut attributes = attrs.iter().filter(|attr| attr.path().is_ident("vtable"));
        let Some(attr) = attributes.next() else {
            return Ok(false);
        };
        if attributes.next().is_some() {
            return self.err(attr, "duplicate native vtable attribute");
        }
        let table: Vtable = attr
            .parse_args()
            .map_err(|error| self.error(attr, &error.to_string()))?;
        self.encode_slot_descriptor(
            target,
            "NativeVtableAttribute",
            vec![
                metadata::Value::I32(table.size),
                metadata::Value::I32(table.inherited),
            ],
        );
        Ok(true)
    }

    pub fn read_native_slot(&self, attrs: &[syn::Attribute]) -> Result<Option<Slot>, Error> {
        let mut attributes = attrs.iter().filter(|attr| attr.path().is_ident("slot"));
        let Some(attr) = attributes.next() else {
            return Ok(None);
        };
        if attributes.next().is_some() {
            return self.err(attr, "duplicate native slot attribute");
        }
        attr.parse_args()
            .map(Some)
            .map_err(|error| self.error(attr, &error.to_string()))
    }

    pub fn encode_native_slot(
        &mut self,
        target: metadata::writer::HasAttribute,
        slot: &Slot,
    ) -> Result<(), Error> {
        self.encode_slot_descriptor(
            target,
            "NativeSlotAttribute",
            vec![metadata::Value::I32(slot.index)],
        );
        if let Some(owner) = &slot.reuse {
            let metadata::Type::ClassName(owner_type) = self.encode_path(owner)? else {
                return self.err(owner, "native slot reuse owner must be an interface");
            };
            self.encode_slot_descriptor(
                target,
                "NativeSlotReuseAttribute",
                vec![metadata::Value::TypeName(owner_type)],
            );
        }
        Ok(())
    }

    fn encode_slot_descriptor(
        &mut self,
        target: metadata::writer::HasAttribute,
        name: &str,
        values: Vec<metadata::Value>,
    ) {
        self.encode_named_attribute(
            target,
            &attribute_ref::AttributeRef {
                type_name: metadata::TypeName::named(METADATA_NAMESPACE, name),
                args: values
                    .into_iter()
                    .map(|value| (String::new(), value))
                    .collect(),
            },
        );
    }
}
