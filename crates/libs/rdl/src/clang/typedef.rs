use super::*;

#[derive(Debug)]
pub struct Typedef {
    pub name: String,
    pub ty: metadata::Type,
}

impl Typedef {
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>) -> Result<Option<Self>, Error> {
        let name = cursor.name();
        let underlying = cursor.typedef_underlying_type();

        // Skip typedefs that alias a struct, union, or enum directly.  In C
        // it is idiomatic to write `typedef struct Foo Foo;` to allow the type
        // to be used without the `struct` keyword.  Such a typedef has the same
        // name as the underlying record and would produce a nonsensical
        // `type Foo = Foo;` alias.  The actual struct/enum definition is
        // emitted separately from the corresponding cursor, so these aliases
        // carry no additional information and must be skipped.
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

        let ty = underlying.to_type(parser);
        Ok(Some(Self { name, ty }))
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let ty = write_type(namespace, &self.ty);

        Ok(quote! {
            type #name = #ty;
        })
    }
}
