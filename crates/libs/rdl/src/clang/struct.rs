use super::*;

#[derive(Debug)]
pub struct Struct {
    pub name: String,
}

impl Struct {
    pub fn parse(cursor: Cursor) -> Result<Self, Error> {
        let name = cursor.name();

        Ok(Self { name })
    }

    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        Ok(quote! {
            struct #name {
            }
        })
    }
}
