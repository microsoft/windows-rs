//! library kernel32.dll
// SAL annotations expressed as portable `annotate` stubs so libclang produces
// `CXCursor_AnnotateAttr` children on each ParmDecl. This exercises the SAL path
// in `clang/annotation.rs` (extract_param_annotation + apply_sal_string) and the
// attribute-emission rules in `param_attrs_for_annotation`.

#define _In_ __attribute__((annotate("_In_")))
#define _In_opt_ __attribute__((annotate("_In_opt_")))
#define _Out_ __attribute__((annotate("_Out_")))
#define _Out_opt_ __attribute__((annotate("_Out_opt_")))
#define _Inout_ __attribute__((annotate("_Inout_")))
#define _Inout_opt_ __attribute__((annotate("_Inout_opt_")))
#define _In_reads_(c) __attribute__((annotate("_In_reads_")))
#define _Out_writes_(c) __attribute__((annotate("_Out_writes_")))

typedef unsigned long DWORD;
typedef void* HANDLE;
typedef int BOOL;
typedef const char* LPCSTR;

extern "C" {
    // _In_ on an opaque typedef matches the inferred In default (no attribute).
    BOOL CloseObject(_In_ HANDLE object);

    // _In_ on a mutable pointer overrides the inferred Out default -> #[r#in].
    BOOL ScanValue(_In_ DWORD* value);

    // _Out_ on a mutable pointer matches the inferred default (no attribute).
    BOOL GetValue(_Out_ DWORD* result);

    // _Inout_ on a mutable pointer is both In and Out -> #[r#in] #[out].
    BOOL UpdateValue(_Inout_ DWORD* value);

    // _In_opt_ on a non-mutable type -> #[opt].
    BOOL OpenByName(_In_opt_ LPCSTR name, _Out_ HANDLE* object);

    // _Out_opt_ on a mutable pointer -> #[opt].
    BOOL TryGetValue(_Out_opt_ DWORD* result);

    // _Inout_opt_ -> #[r#in] #[out] #[opt].
    BOOL UpdateOptional(_Inout_opt_ DWORD* value);

    // Buffer-size SAL macros collapse to plain In/Out.
    BOOL ReadBuffer(_In_reads_(size) const void* buffer, DWORD size);
    BOOL WriteBuffer(_Out_writes_(size) void* buffer, DWORD size);
}
