// The `_COM_Outptr_` SAL macro as it survives in real MIDL-generated SDK headers
// (e.g. DXGI's `IDXGIObject::GetParent` / `IDXGIFactory::EnumAdapters`): clang does
// NOT expose it as a `ParmDecl` `AnnotateAttr` (the SDK's own `sal.h` expands it to
// inner annotations that clang drops on abstract-virtual methods), so only the bare
// `_COM_Outptr_` source token remains, and the MIDL block comment carries `[out]` /
// `[retval]` but never `[iid_is]`.
//
// The out-pointer is recovered from the `_COM_Outptr_` token in
// `scan_method_param_annotations`, but only *promoted* to a `ComOutPtr` (`#[iid_is]`)
// when the pointee is `void**` — the caller-chosen-type idiom (`GetParent`), which
// drives bindgen's generic `GetParent<T>() -> Result<T>`. A `_COM_Outptr_` on a
// concrete interface pointee (`EnumThings` → `IThing**`) has no sibling `REFIID`
// to select the type, so it stays faithfully typed (`*mut IThing`), matching the
// canonical `EnumAdapters -> Result<IThing>` projection.
//
// Contrast sal_com.h, where `_COM_Outptr_` *does* attach as an `AnnotateAttr` and is
// caught by the per-`ParmDecl` SAL path, and midl_iid_is.h, where an explicit
// `[iid_is]` block comment always erases the pointee to `void**`.

// Empty expansion: the identifier survives for the source tokenizer but attaches no
// attribute, mirroring the real SDK header after its own sal.h has been included.
#define _COM_Outptr_

typedef int HRESULT;
typedef struct _GUID {
    unsigned long Data1;
} GUID;
typedef const GUID* REFIID;

struct __declspec(uuid("aec22fb8-76f3-4639-9be0-28eb43a67a2e")) IThing {
    virtual HRESULT DoThing() = 0;
};

struct __declspec(uuid("7b7166ec-21c7-44ae-b21a-c9ae321ae369")) IDXGIObject {
    // `void**` pointee + companion `REFIID` -> promoted to #[iid_is] #[retval].
    virtual HRESULT GetParent(
        /* [annotation][in] */ REFIID riid,
        /* [annotation][retval][out] */ _COM_Outptr_ void** ppParent) = 0;

    // Concrete interface pointee, no companion `REFIID` -> NOT promoted; stays the
    // faithful `*mut IThing` out-parameter (the `EnumAdapters` idiom).
    virtual HRESULT EnumThings(
        /* [in] */ unsigned int Thing,
        /* [annotation][out] */ _COM_Outptr_ IThing** ppThing) = 0;
};
