use super::*;

#[derive(Debug)]
pub struct Fn {
    pub name: String,
    pub library: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
    pub extern_c: bool,
    pub is_variadic: bool,
}

impl Fn {
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>, extern_c: bool) -> Result<Self, Error> {
        let name = cursor.name();
        let return_type = cursor.result_type().to_type(parser);

        let is_variadic = cursor.ty().is_variadic();

        let mut params = vec![];

        for child in cursor.children() {
            if child.kind() != CXCursor_ParmDecl {
                continue;
            }

            let name = child.name();
            let ty = child.ty().to_type(parser);
            params.push(Param { name, ty });
        }

        Ok(Self {
            name,
            library: parser.library.to_string(),
            params,
            return_type,
            extern_c,
            is_variadic,
        })
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let library = &self.library;

        let mut params: Vec<TokenStream> = self
            .params
            .iter()
            .map(|param| {
                let name = write_ident(&param.name);
                let ty = write_type(namespace, &param.ty);
                quote! { #name: #ty }
            })
            .collect();

        if self.is_variadic {
            params.push(quote! { ... });
        }

        let return_type = match &self.return_type {
            metadata::Type::Void => quote! {},
            ty => {
                let ty = write_type(namespace, ty);
                quote! { -> #ty }
            }
        };

        let abi = if self.extern_c {
            quote! { "C" }
        } else {
            quote! {}
        };

        Ok(quote! {
            #[library(#library)]
            extern #abi fn #name(#(#params),*) #return_type;
        })
    }
}
