use super::*;

#[derive(Debug)]
pub struct Typedef {
    pub name: String,
    pub ty: metadata::Type,
}

impl Typedef {
    /// Prefer `PFOO -> *mut FOO` over an equivalent pointer through another alias.
    pub fn is_direct_pointer_alias(&self) -> bool {
        let Some(target) = self.name.strip_prefix('P') else {
            return false;
        };
        let pointee = match &self.ty {
            metadata::Type::PtrMut(ty, 1) | metadata::Type::PtrConst(ty, 1) => ty.as_ref(),
            _ => return false,
        };
        match pointee {
            metadata::Type::ClassName(name) | metadata::Type::ValueName(name) => {
                name.name == target
            }
            _ => false,
        }
    }

    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>) -> Result<Option<Self>, Error> {
        let name = cursor.name();
        let underlying = cursor.typedef_underlying_type();

        // The enum/flags merge emits the public name with this typedef's storage type.
        if parser.enum_merge.contains_key(&name) {
            return Ok(None);
        }

        // GUID synonyms collapse to `GUID` at reference sites.
        if guid_alias(&name) {
            return Ok(None);
        }

        // Generic void-pointer aliases collapse at reference sites; handles stay named.
        if void_pointer_alias(&name).is_some() {
            return Ok(None);
        }

        // Flat Win32 scrapes collapse Direct2D 1.1 compatibility aliases to `D2D_*`.
        if parser.header_root.is_some() && d2d_compat_alias(&name).is_some() {
            return Ok(None);
        }

        // Flat Win32 scrapes keep only the canonical string-pointer wrappers. Namespaced
        // scrapes must keep local `LP*` aliases because their references are not normalized.
        if parser.header_root.is_some()
            && let Some(canonical) = string_alias_canonical(&name)
            && canonical != name
        {
            return Ok(None);
        }

        // Flat Win32 scrapes collapse curated portability/ABI aliases to primitives; keeping
        // pointer-sized aliases named would create false per-arch width splits.
        if parser.header_root.is_some()
            && (fundamental_scalar(&name).is_some() || pointer_sized_abi(&name).is_some())
        {
            return Ok(None);
        }

        // Semantic scalar aliases such as `BOOLEAN` collapse to primitives.
        if parser.header_root.is_some() && semantic_scalar(&name).is_some() {
            return Ok(None);
        }

        // Floating typedefs all collapse by canonical kind to `f32`/`f64`.
        if parser.header_root.is_some() && floating_typedef(&underlying).is_some() {
            return Ok(None);
        }

        // Flat Win32 scrapes collapse COM interface pointer aliases to the interface type.
        if parser.header_root.is_some() && is_interface_alias(&underlying) {
            return Ok(None);
        }

        // Skip self-aliases, but preserve secondary record aliases in per-header mode.
        let elaborated = underlying.kind() == CXType_Elaborated;
        let inner_kind = if elaborated {
            underlying.underlying_type().kind()
        } else {
            underlying.kind()
        };
        if matches!(inner_kind, CXType_Record | CXType_Enum) {
            let inner = if elaborated {
                underlying.underlying_type()
            } else {
                underlying
            };
            let tag = inner.ty().name();
            if parser.header_root.is_none() || is_anonymous_name(&tag) {
                // Legacy mode or an anonymous tag already emitted under this typedef name.
                return Ok(None);
            }
            let public = parser
                .tag_rename
                .get(&tag)
                .cloned()
                .unwrap_or_else(|| tag.clone());
            if name == public
                || (name == tag
                    && inner_kind == CXType_Enum
                    && parser.enum_merge.contains_key(&public))
            {
                // The record/enum is already emitted under this public name.
                return Ok(None);
            }
            let ty = inner.to_type(parser);
            return Ok(Some(Self { name, ty }));
        }

        // Variadic function-pointer typedefs cannot be metadata callbacks; keep an opaque alias.
        if underlying.is_function_pointer() {
            if let Some(fn_ty) = underlying.function_pointee()
                && fn_ty.is_variadic()
            {
                let ty = underlying.to_type(parser);
                return Ok(Some(Self { name, ty }));
            }
            return Ok(None);
        }

        // `DECLARE_HANDLE` emits an opaque handle rather than a one-off empty tag pointer.
        if underlying.is_handle_tag(&name) {
            let ty = metadata::Type::PtrMut(Box::new(metadata::Type::Void), 1);
            return Ok(Some(Self { name, ty }));
        }

        let ty = underlying.to_type(parser);

        // Base pointer-sized ABI typedefs become `usize`/`isize`; chained aliases inherit.
        let ty = match (pointer_sized_abi(&name), &ty) {
            (Some(scalar), metadata::Type::U64 | metadata::Type::I64) => scalar,
            _ => ty,
        };
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
