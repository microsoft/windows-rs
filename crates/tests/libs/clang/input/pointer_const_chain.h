// The winmd `Type` model stores a pointer run as a single const bit + depth, so it cannot
// represent a mixed-constness chain (`*mut *const T`). The scraper collapses such chains to a
// uniform chain governed by the outermost level — the parameter's real read/write direction —
// so an output `const wchar_t **` retval projects to `*mut *mut u16` (matching the canonical
// projection, writable by the callee) rather than the winmd-corrupt `*const *const u16`, while
// an input `const wchar_t * const *` stays `*const *const u16`.

typedef int HRESULT;
#define STDMETHODCALLTYPE __stdcall

struct __declspec(uuid("aec22fb8-76f3-4639-9be0-28eb43a67a2e")) ISAXLocator {
    // `[retval][out] const wchar_t **` -> output double-pointer -> `*mut *mut u16`.
    virtual HRESULT STDMETHODCALLTYPE getPublicId(
        /* [retval][out] */ const wchar_t **ppwchPublicId) = 0;

    // `[in] const wchar_t * const *` -> input double-pointer -> `*const *const u16`.
    virtual HRESULT STDMETHODCALLTYPE setPrefixes(
        /* [in] */ const wchar_t * const *ppwchPrefixes) = 0;
};
