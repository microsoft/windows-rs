use super::*;

#[derive(Debug)]
pub struct Callback {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: metadata::Type,
}

impl Callback {
    /// Try to parse a `CXCursor_TypedefDecl` cursor as a callback (function-pointer typedef).
    ///
    /// Returns `Ok(Some(Callback))` when the typedef's underlying type is a pointer to a
    /// function prototype, `Ok(None)` otherwise.
    pub fn parse(
        cursor: Cursor,
        namespace: &str,
        ref_map: &HashMap<String, String>,
        tag_rename: &HashMap<String, String>,
        pending: &mut Vec<Cursor>,
    ) -> Result<Option<Self>, Error> {
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

        // Variadic function-pointer typedefs (e.g. `typedef int (*printf_t)(const char*, ...)`)
        // cannot be represented as Windows metadata callbacks.  Skip them; the caller will
        // fall through to Typedef::parse which emits a `type` alias instead.
        if fn_type.is_variadic() {
            return Ok(None);
        }

        let return_type = fn_type
            .fn_result_type()
            .to_type(namespace, ref_map, tag_rename, pending);

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
            let ty = fn_type
                .arg_type(i as u32)
                .to_type(namespace, ref_map, tag_rename, pending);
            let pname = param_names
                .get(i as usize)
                .cloned()
                .filter(|n| !n.is_empty())
                .unwrap_or_else(|| format!("param_{}", i));
            params.push(Param { name: pname, ty });
        }

        Ok(Some(Self {
            name,
            params,
            return_type,
        }))
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let params = self.params.iter().map(|param| {
            let name = write_ident(&param.name);
            let ty = write_type(namespace, &param.ty);
            quote! { #name: #ty }
        });

        let return_type = match &self.return_type {
            metadata::Type::Void => quote! {},
            ty => {
                let ty = write_type(namespace, ty);
                quote! { -> #ty }
            }
        };

        Ok(quote! {
            extern fn #name(#(#params),*) #return_type;
        })
    }
}
