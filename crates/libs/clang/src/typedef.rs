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

        // The public side of the C flags/enum idiom (`enum _FOO {...}; typedef DWORD FOO;`):
        // `merge_enum_typedef_idiom` renamed the enum tag `_FOO` to `FOO`, which now carries
        // the members and this typedef's storage type, so the integer typedef is redundant.
        // Dropping it leaves a single unscoped enum `FOO` projecting as a bare integer alias.
        if parser.enum_merge.contains_key(&name) {
            return Ok(None);
        }

        // `IID`/`CLSID`/`FMTID` are `typedef GUID X` synonyms; every reference collapses
        // to `GUID` (see `guid_alias` in `to_type`), so the redundant alias item is never
        // emitted - matching the reference metadata, which carries no such types. This
        // holds in both per-header and namespaced modes.
        if guid_alias(&name) {
            return Ok(None);
        }

        // `PVOID`/`LPVOID`/`LPCVOID`/... are generic void-pointer portability spellings that
        // collapse to raw `*mut void` / `*const void` at every reference (see
        // [`void_pointer_alias`] in `to_type`), so the redundant `type LPVOID = *mut c_void`
        // item is never emitted - matching the reference metadata, which has a bare
        // `*mut c_void` everywhere and no such alias types. A `void*` *handle* (`HANDLE`,
        // the `DECLARE_HANDLE` tags) is excluded and still emitted as a named opaque handle.
        if void_pointer_alias(&name).is_some() {
            return Ok(None);
        }

        // `D2D1_POINT_2F`/`D2D1_RECT_F`/... are Direct2D 1.1 `typedef D2D_X D2D1_X`
        // compatibility synonyms; every reference collapses to the shared `D2D_` base
        // (see [`d2d_compat_alias`] in `to_type`), so the redundant
        // `type D2D1_POINT_2F = D2D_POINT_2F` item is not emitted - matching the
        // reference metadata, which has no `D2D1_*` alias layer. Flat scrape only,
        // mirroring the reference-site collapse: a namespaced scrape resolves these
        // against its reference winmd rather than a local root-namespace base.
        if parser.header_root.is_some() && d2d_compat_alias(&name).is_some() {
            return Ok(None);
        }

        // A redundant string-pointer alias (`LPCWSTR`->`PCWSTR`, `LPWSTR`->`PWSTR`,
        // `LPOLESTR`/`POLESTR`->`PWSTR`, ...) normalises to its canonical spelling at every
        // reference (see [`string_alias_canonical`] / [`normalize_string_alias`] in
        // `to_type`), so its `type LPCWSTR = *const u16` item is not emitted - matching the
        // reference metadata, which carries only the four canonical wrappers and no `LP*`
        // spellings. The canonical `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR` themselves (where
        // `canonical == name`) are the projection targets bindgen recognises and are kept.
        //
        // Gated to the flat per-header scrape, mirroring the reference-site normalization in
        // `to_type`: a *namespaced* scrape (WebView2) does not normalise `LP*` references
        // (its reference winmd lacks the const wrappers), so it still emits and references
        // the local `LP*` typedef verbatim - suppressing the definition here would leave
        // those references dangling ("type not found").
        if parser.header_root.is_some()
            && let Some(canonical) = string_alias_canonical(&name)
            && canonical != name
        {
            return Ok(None);
        }

        // A pure fixed-width portability alias (`DWORD` -> u32, `WORD` -> u16, ...) or a
        // pointer-sized ABI alias (`ULONG_PTR`/`SIZE_T` -> usize, `LONG_PTR` -> isize, ...)
        // is collapsed to its primitive at every reference (`to_type`), so the redundant
        // `type DWORD = u32` / `type ULONG_PTR = usize` item is not emitted - matching the
        // reference metadata, which has no such alias types. Emitting `ULONG_PTR` as a named
        // typedef would also freeze its canonical width per-arch (u32 on x86, usize on 64-bit)
        // and produce a spurious `#[arch]` split. Every other scalar typedef (`HFILE`, `ATOM`,
        // `COLORREF`, ...) is preserved by name; the collapse-lists are curated, see
        // [`fundamental_scalar`] and [`pointer_sized_abi`].
        if parser.header_root.is_some()
            && (fundamental_scalar(&name).is_some() || pointer_sized_abi(&name).is_some())
        {
            return Ok(None);
        }

        // Floating-point typedefs (`FLOAT`/`DOUBLE` and every domain alias whose canonical
        // type is `float`/`double`, e.g. `DATE`, `REFTIME`, `UI_ANIMATION_SECONDS`) collapse
        // structurally to `f32`/`f64` at every reference (see [`floating_typedef`] in
        // `to_type`), so the redundant `type FLOAT = f32` item is not emitted - matching the
        // reference metadata, which carries no floating typedefs at all. Unlike the integer
        // side there is no domain-vs-portability split to preserve, so this is keyed on the
        // canonical kind rather than a curated name list.
        if parser.header_root.is_some() && floating_typedef(&underlying).is_some() {
            return Ok(None);
        }

        // `LPSTORAGE`/`LPOLEOBJECT`/`LPDIRECTDRAWSURFACE`/... are `typedef IFoo *NAME`
        // (or the rarer `typedef IFoo NAME`) aliases to a COM interface. Interfaces are
        // implied pointers in Windows metadata, so every reference collapses to the
        // interface itself (see [`is_interface_alias`] in `to_type`); the redundant alias
        // is not emitted - matching the reference metadata, which omits these aliases and
        // types the field/parameter as the interface directly. Flat scrape only, mirroring
        // the reference-site collapse: a namespaced scrape resolves interfaces against its
        // reference winmd rather than the local closure.
        if parser.header_root.is_some() && is_interface_alias(&underlying) {
            return Ok(None);
        }

        // Skip idiomatic C `typedef struct Foo Foo;` aliases. They share a name with
        // the underlying record and would produce a nonsensical `type Foo = Foo;`.
        //
        // A tagged record may, however, carry *several* typedef aliases
        // (`typedef struct _CRYPTOAPI_BLOB { ... } CRYPT_INTEGER_BLOB, ..., DATA_BLOB;`).
        // The struct is emitted once under its public name (the first alias, via the
        // tag->typedef rename); in per-header mode the remaining aliases are
        // preserved as `type DATA_BLOB = CRYPT_INTEGER_BLOB` so references to them
        // resolve instead of dangling.
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
                // Legacy mode, or an anonymous tag emitted directly under this
                // typedef name - nothing extra to alias.
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
                // This typedef *is* the record/enum's public name, or the MIDL
                // `[v1_enum]` self-alias (`typedef enum _FOO {...} _FOO;`) whose tag the
                // flags/enum merge renamed to the public integer typedef `FOO` - either
                // way the enum is already emitted under `FOO`, so this alias is redundant.
                return Ok(None);
            }
            let ty = inner.to_type(parser);
            return Ok(Some(Self { name, ty }));
        }

        // Function-pointer typedefs are normally emitted as named callbacks
        // (`Callback::parse`). Variadic ones cannot be represented as metadata
        // delegates, so `Callback::parse` skips them; emit an opaque alias
        // here instead so functions that take the typedef still resolve (matching the
        // inline function-pointer convention, an opaque `*mut u8`).
        if underlying.is_function_pointer() {
            if let Some(fn_ty) = underlying.function_pointee()
                && fn_ty.is_variadic()
            {
                let ty = underlying.to_type(parser);
                return Ok(Some(Self { name, ty }));
            }
            return Ok(None);
        }

        // DECLARE_HANDLE idiom (`typedef struct X__ *X`): emit an opaque handle
        // (`*mut void`) rather than a pointer to a one-off empty tag struct. The
        // resulting `NativeTypedef` matches the hand-authored `Foundation::HANDLE`.
        if underlying.is_handle_tag(&name) {
            let ty = metadata::Type::PtrMut(Box::new(metadata::Type::Void), 1);
            return Ok(Some(Self { name, ty }));
        }

        let ty = underlying.to_type(parser);

        // Pointer-sized ABI typedefs (`ULONG_PTR`, `size_t`, `intptr_t`, ...) are
        // `usize`/`isize`, not the fixed-width `u64`/`i64` their canonical type
        // resolves to on a 64-bit parse. Only the *base* typedef (whose underlying is
        // the raw integer) is remapped; chained aliases like `SIZE_T = ULONG_PTR`
        // keep their alias and inherit the size through it.
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
