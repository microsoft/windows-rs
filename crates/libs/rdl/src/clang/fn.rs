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
            let annotation = extract_sal(&child, parser.tu);
            params.push(Param {
                name,
                ty,
                annotation,
            });
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
                let (in_attr, out_attr, opt_attr) =
                    sal_attrs_for_param(&param.annotation, &param.ty);
                quote! { #in_attr #out_attr #opt_attr #name: #ty }
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

/// Compute the `#[r#in]`, `#[out]`, and `#[opt]` token streams for a parameter
/// given its SAL annotation and type.
///
/// The emission rules mirror those of [`writer::write_params`] so that the
/// clang → RDL → winmd → RDL roundtrip is stable: attributes emitted here are
/// the same attributes that the writer would re-emit after encoding into winmd.
///
/// When `annotation` has no flags set (no SAL was detected), no attributes are
/// emitted (the reader will infer direction from the type).
pub fn sal_attrs_for_param(
    annotation: &ParamAnnotation,
    ty: &metadata::Type,
) -> (TokenStream, TokenStream, TokenStream) {
    if !annotation.is_annotated() {
        return (quote! {}, quote! {}, quote! {});
    }

    let in_param = annotation.in_param;
    let out_param = annotation.out_param;
    let optional = annotation.optional;

    let is_mutable = matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..));

    // effective_in: treat as In when explicitly in or when not explicitly out
    let effective_in = in_param || !out_param;

    // Emit #[r#in] only when it is needed to override the default direction:
    //   - always when the param is both In and Out
    //   - when In but the type is mutable (default would otherwise be Out)
    let in_attr = if effective_in && (out_param || is_mutable) {
        quote! { #[r#in] }
    } else {
        quote! {}
    };

    // Emit #[out] only when it is needed to override the default direction:
    //   - always when the param is both In and Out
    //   - when Out but the type is not mutable (default would otherwise be In)
    let out_attr = if out_param && (effective_in || !is_mutable) {
        quote! { #[out] }
    } else {
        quote! {}
    };

    let opt_attr = if optional {
        quote! { #[opt] }
    } else {
        quote! {}
    };

    (in_attr, out_attr, opt_attr)
}
