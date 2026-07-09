//! library test.dll
// `_COM_Outptr_*` out-pointer recovery from the raw source token stream.
//
// The Windows SDK's `specstrings_strict.h` redefines the `_COM_Outptr_*` family
// (after the `crates/tools/win32/src/sal.h` shim runs) to a form that clang drops,
// so these macros never survive as a `ParmDecl` `AnnotateAttr`. They ARE still
// present as bare identifier tokens in the enclosing function/method extent, where
// `scan_method_param_annotations` recovers them. That recovery must preserve the
// `_opt_` optionality (the `D3D11CreateDevice` `ppDevice`/`ppImmediateContext`
// shape uses `_COM_Outptr_opt_`) while keeping the deferred `void**`-vs-concrete
// `ComOutPtr` decision intact.
//
// The empty `_COM_Outptr_*` macros below mimic that clobbered state; `_Out_opt_`
// keeps the shim's annotate stub (it survives strict redefinition) as a control.

#define _Out_opt_ __attribute__((annotate("_Out_opt_")))
#define _COM_Outptr_
#define _COM_Outptr_opt_

typedef int HRESULT;

struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IThing {
    virtual HRESULT M() = 0;
};

extern "C" {
    // Control: `_Out_opt_` scalar via the AnnotateAttr path -> #[opt].
    HRESULT GetScalar(_Out_opt_ int* value);

    // `_COM_Outptr_opt_` on a concrete interface pointer-to-pointer: stays
    // faithfully typed (`*mut IThing`, no #[iid_is]) but is optional -> #[opt].
    HRESULT GetThing(_COM_Outptr_opt_ IThing** thing);

    // `_COM_Outptr_` (non-opt) concrete interface pointer-to-pointer: typed, no
    // #[opt] (guards against the fix adding spurious optionality).
    HRESULT MakeThing(_COM_Outptr_ IThing** thing);

    // `_COM_Outptr_opt_` on a caller-chosen-type `void**`: erased to `*mut *mut void`
    // with #[iid_is], and optional -> #[opt].
    HRESULT QueryThing(_COM_Outptr_opt_ void** thing);
}
