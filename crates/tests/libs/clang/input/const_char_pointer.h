//! flat
// A string-pointer *sentinel* constant is spelled with an inline pointer-to-char cast
// (`(OLECHAR*)`/`(CHAR*)`) rather than a named alias, e.g.
// `#define COLE_DEFAULT_PRINCIPAL ((OLECHAR*)(INT_PTR)-1)`. `parse_nested_cast` rejects
// it at the `*` (its cast chain only accepts bare typedef names), so it is matched by its
// fixed token shape in `parse_body` and emitted as the canonical `PWSTR`/`PSTR` carrying
// the sign-extended sentinel value (bindgen projects `PCWSTR(-1 as _)`, so
// `COLE_DEFAULT_PRINCIPAL.0 as usize == usize::MAX`).
#define COLE_DEFAULT_PRINCIPAL ((OLECHAR*)(INT_PTR)-1)
#define COLE_NARROW_SENTINEL ((CHAR*)(INT_PTR)-1)
