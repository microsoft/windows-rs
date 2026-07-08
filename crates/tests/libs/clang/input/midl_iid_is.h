// MIDL-generated COM interfaces annotate parameters with block comments
// (`/* [in] */`, `/* [iid_is][retval][out] */`, …) rather than SAL macros. This
// exercises the MIDL-comment path in `clang/annotation.rs`
// (scan_method_param_annotations + apply_midl_param_comment): an `[iid_is]`
// interface out-pointer (the QueryInterface / Resolve idiom) must map to the
// `ComOutPtr` marker, matching the `_COM_Outptr_` SAL path in sal_com.h.
//
// It also covers the `guid_alias` collapse in `clang/cx.rs`: `IID` is `typedef GUID
// IID`, so it (and the `REFIID` = `const IID*` pointer form) resolve to `GUID`, and
// no redundant `type IID = GUID` alias is emitted.
//
// Finally it covers `[iid_is]` outputs declared with a *concrete* interface type
// (`IUnknown**`) — the SDK's `IWeakReference::Resolve` is declared `IInspectable**`
// but is really a generic `void**` (the pointee is redundant with the sibling
// `REFIID`). Such ComOutPtr outputs are normalised to `*mut *mut void`, matching the
// canonical Win32 metadata and driving bindgen's ergonomic `Resolve<T>()` projection.

typedef int HRESULT;
typedef struct _GUID {
    unsigned long Data1;
} GUID;
typedef GUID IID;
typedef const IID* REFIID;

struct IUnknown {
    virtual HRESULT QueryInterface(REFIID riid, void** ppv) = 0;
};

struct __declspec(uuid("c03f6a43-65a4-9818-987e-e0b810d2a6f2")) IResolver {
    // `[iid_is]` on the void** output -> #[...::ComOutPtr] (plus the retval marker);
    // the `[in]` `REFIID` collapses to a plain `*const GUID` input.
    virtual HRESULT Resolve(
        /* [in] */ REFIID riid,
        /* [iid_is][retval][out] */ void** ppvObject) = 0;

    // A `[iid_is]` output declared with a concrete interface type (`IUnknown**`) is
    // erased to `*mut *mut void` — the IWeakReference::Resolve idiom.
    virtual HRESULT ResolveTyped(
        /* [in] */ REFIID riid,
        /* [iid_is][retval][out] */ IUnknown** objectReference) = 0;

    // An `[in][iid_is]` interface *input* pointer must NOT become a ComOutPtr.
    virtual HRESULT Bind(
        /* [in] */ REFIID riid,
        /* [in][iid_is] */ void* punk) = 0;
};
