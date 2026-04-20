use super::*;

#[derive(Debug)]
pub struct Typedef {
    pub name: String,
    pub namespace: String,
    pub ty: metadata::Type,
}

impl Typedef {
    pub fn parse(cursor: Cursor, namespace: &str) -> Result<Option<Self>, Error> {
        let name = cursor.name();
        let underlying = cursor.typedef_underlying_type();

        // TODO: is this needed:
        // Skip typedefs that alias structs, unions, or enums — those are
        // collected separately from the corresponding struct/enum cursors.
        match underlying.kind() {
            CXType_Record | CXType_Enum => return Ok(None),
            CXType_Elaborated => {
                let inner_kind = underlying.underlying_type().kind();
                if inner_kind == CXType_Record || inner_kind == CXType_Enum {
                    return Ok(None);
                }
            }
            _ => {}
        }

        // Skip function-pointer typedefs — those are collected as callbacks.
        if underlying.is_function_pointer() {
            return Ok(None);
        }

        let ty = underlying.to_type(namespace);
        Ok(Some(Self {
            name,
            namespace: namespace.to_string(),
            ty,
        }))
    }

    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let ty = write_type(&self.namespace, &self.ty);

        Ok(quote! {
            type #name = #ty;
        })
    }
}
