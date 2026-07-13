//! flat
//! namespace Windows.Win32

// Caller-chosen-type COM creators that the SDK headers ship WITHOUT `_COM_Outptr_` SAL
// and WITHOUT a MIDL `[iid_is]` block comment (the real `DCompositionCreateDevice`,
// `OleCreate*`/`OleLoad*`, and property-system `PS*` families). `infer_iid_is` recovers
// the `ComOutPtr` (`#[iid_is]`) marker from the signature shape rather than a per-parameter
// annotation, so the projected surface matches the annotated creators (`QueryInterface`
// idiom) instead of a raw double pointer.
//
// The gate is deliberately narrow so it is a promotion of a genuine idiom, never a lossy
// guess: the function must return `HRESULT`, a `*const GUID` parameter must be named
// `riid`/`iid`/`riidltf`, and the promoted parameter must be a caller-chosen-type output —
// either a `*mut *mut void` or a base-interface `*mut IUnknown`/`*mut IInspectable`.

typedef int HRESULT;
typedef struct _GUID {
    unsigned long Data1;
} GUID;
typedef const GUID *REFIID;

#define _Out_writes_to_(c, w) __attribute__((annotate("_Out_writes_to_(" #c "," #w ")")))

// Minimal base COM interfaces + a concretely-typed interface, so the base-interface arm of
// `infer_iid_is` (a `*mut IUnknown`/`*mut IInspectable` out) can be exercised against a
// concretely-typed out (`*mut IFoo`) that must stay faithfully typed.
struct IUnknown {
    virtual HRESULT QueryInterface(REFIID riid, void **ppvObject) = 0;
    virtual unsigned long AddRef() = 0;
    virtual unsigned long Release() = 0;
};
struct IInspectable : IUnknown {
    virtual HRESULT GetIids(unsigned long *count, GUID **iids) = 0;
};
struct IFoo : IUnknown {
    virtual HRESULT Bar() = 0;
};

// POSITIVE: `HRESULT` return + `REFIID` named `iid` + `void**` out -> `#[iid_is]` inferred.
// This is the `DCompositionCreateDevice` shape.
HRESULT CreateThing(REFIID iid, void **ppThing);

// POSITIVE: adjacency is NOT required. Several parameters sit between the `riid` selector
// and the `void**` out-pointer (the `OleCreate*` shape).
HRESULT LoadThing(unsigned int flags, unsigned int mode, const GUID *riid, void **ppv);

// NEGATIVE: the return type is not `HRESULT` (`unsigned long` -> `u32`), so the `void**`
// out-pointer stays a bare `*mut *mut void`. This is what excludes the buffer-style
// GUID-out functions (`RpcServerInqIf`, `WlanQueryInterface`) that return a status code.
unsigned long QueryThing(REFIID iid, void **ppThing);

// NEGATIVE: the `*const GUID` parameter is data, not a type selector — its name is not
// `riid`/`iid`/`riidltf` — so the `void**` out-pointer stays a bare `*mut *mut void`.
// This is the `DsReplicaGetInfoW`/`TdhCreatePayloadFilter` shape.
HRESULT GetThing(const GUID *providerGuid, void **ppData);

// NEGATIVE: a caller-chosen-type ARRAY enumerator. The `void**` carries a `size_is(celt)`
// length, so it is a counted array, not a single-object out-pointer; promoting it would
// drop the count parameter the projection still references. This is the `IEnumObjects::Next`
// shape.
HRESULT NextThings(unsigned long celt, REFIID riid, _Out_writes_to_(celt, *pFetched) void **rgelt, unsigned long *pFetched);

// POSITIVE: the base-interface arm. The out-parameter is spelled as the base interface
// `IUnknown**` (an upstream header inconsistency), but the `iid` selects its type, so it is
// promoted to `#[iid_is]` just like the `void**` form. This is the `DWriteCreateFactory`
// shape; `IInspectable**` is treated identically.
HRESULT CreateFactory(REFIID iid, IUnknown **factory);
HRESULT CreateInspectable(REFIID iid, IInspectable **inspectable);

// NEGATIVE: the out-parameter is a *concretely* typed interface (`IFoo**`), so the `iid`
// selects a different object than this fixed-type out — it stays faithfully typed and is not
// promoted. This is the `ActivateAudioInterfaceAsync`/`RoGetAgileReference` shape.
HRESULT CreateTyped(REFIID iid, IFoo **ppFoo);

// The inference applies to COM interface methods too — the `GetService`/`Activate`/
// `CreateInstance` families that lack `_COM_Outptr_` SAL and a `[iid_is]` comment.
struct __declspec(uuid("aec22fb8-76f3-4639-9be0-28eb43a67a2e")) IThingFactory {
    // POSITIVE: `HRESULT` method + `REFIID` named `riid` + `void**` out -> `#[iid_is]`.
    virtual HRESULT GetService(REFIID riid, void **ppv) = 0;

    // NEGATIVE: no `riid`-named selector -> the `void**` out-pointer stays bare.
    virtual HRESULT GetBuffer(void **ppBuffer) = 0;
};

