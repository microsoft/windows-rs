use super::*;

#[derive(Debug)]
pub struct Callback {
    pub name: String,
    pub namespace: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
}

impl Callback {
    /// Try to parse a `CXCursor_TypedefDecl` cursor as a callback (function-pointer typedef).
    ///
    /// Returns `Ok(Some(Callback))` when the typedef's underlying type is a pointer to a
    /// function prototype, `Ok(None)` otherwise.
    pub fn parse(cursor: Cursor, namespace: &str) -> Result<Option<Self>, Error> {
        let name = cursor.name();
        if name.is_empty() || name.starts_with('_') {
            return Ok(None);
        }

        let underlying = cursor.typedef_underlying_type();

        // Resolve elaborated types (e.g. `typedef struct Foo Foo`) first.
        let underlying = if underlying.kind() == CXType_Elaborated {
            underlying.underlying_type()
        } else {
            underlying
        };

        // We only handle pointer-to-function typedefs.
        let fn_type = match underlying.function_pointee() {
            Some(ft) => ft,
            None => return Ok(None),
        };

        let return_type = fn_type.fn_result_type().to_type(namespace);

        // Get parameter names from the TypedefDecl cursor's ParmDecl children.
        let param_names: Vec<String> = cursor
            .children()
            .into_iter()
            .filter(|c| c.kind() == CXCursor_ParmDecl)
            .map(|c| c.name())
            .collect();

        let num_args = fn_type.num_arg_types();
        let mut params = vec![];
        for i in 0..num_args {
            let ty = fn_type.arg_type(i as u32).to_type(namespace);
            let pname = param_names
                .get(i as usize)
                .cloned()
                .filter(|n| !n.is_empty())
                .unwrap_or_else(|| format!("param_{}", i));
            params.push(Param { name: pname, ty });
        }

        Ok(Some(Self {
            name,
            namespace: namespace.to_string(),
            params,
            return_type,
        }))
    }

    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let params = self.params.iter().map(|param| {
            let name = write_ident(&param.name);
            let ty = write_type(&self.namespace, &param.ty);
            quote! { #name: #ty }
        });

        let return_type = match &self.return_type {
            metadata::Type::Void => quote! {},
            ty => {
                let ty = write_type(&self.namespace, ty);
                quote! { -> #ty }
            }
        };

        Ok(quote! {
            extern fn #name(#(#params),*) #return_type;
        })
    }
}
