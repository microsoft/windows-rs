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
#define _In_reads_(c) __attribute__((annotate("_In_reads_(" #c ")")))
#define _Out_writes_(c) __attribute__((annotate("_Out_writes_(" #c ")")))
#define _In_reads_bytes_(c) __attribute__((annotate("_In_reads_bytes_(" #c ")")))
#define _Out_writes_bytes_(c) __attribute__((annotate("_Out_writes_bytes_(" #c ")")))
#define _Inout_updates_(c) __attribute__((annotate("_Inout_updates_(" #c ")")))
#define _Out_writes_z_(c) __attribute__((annotate("_Out_writes_z_(" #c ")")))
#define _Inout_updates_z_(c) __attribute__((annotate("_Inout_updates_z_(" #c ")")))
#define _Out_writes_to_opt_(c, w) __attribute__((annotate("_Out_writes_to_opt_(" #c "," #w ")")))
#define _Out_writes_to_(c, w) __attribute__((annotate("_Out_writes_to_(" #c "," #w ")")))
#define _Out_writes_all_(c) __attribute__((annotate("_Out_writes_all_(" #c ")")))
#define _Inout_updates_to_(c, w) __attribute__((annotate("_Inout_updates_to_(" #c "," #w ")")))
#define _Outptr_result_buffer_(c) __attribute__((annotate("_Outptr_result_buffer_(" #c ")")))
#define _Reserved_ __attribute__((annotate("_Reserved_")))

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

    // Element-count buffer: the count parameter resolves to a 0-based index
    // -> NativeArrayInfo(CountParamIndex = 1).
    BOOL ReadBuffer(_In_reads_(size) const void* buffer, DWORD size);
    BOOL WriteBuffer(_Out_writes_(size) void* buffer, DWORD size);

    // Byte-count buffer: the byte-count parameter resolves to a 0-based index
    // -> MemorySize(BytesParamIndex = 1).
    BOOL ReadBytes(_In_reads_bytes_(cb) const void* buffer, DWORD cb);
    BOOL WriteBytes(_Out_writes_bytes_(cb) void* buffer, DWORD cb);

    // Fixed element count -> NativeArrayInfo(CountConst = 16).
    BOOL FillFixed(_Out_writes_(16) DWORD* buffer);

    // _Inout_updates_ count parameter -> #[r#in] #[out] + NativeArrayInfo(CountParamIndex = 1).
    BOOL UpdateBuffer(_Inout_updates_(count) DWORD* buffer, DWORD count);

    // _Out_writes_z_ null-terminated element-count buffer: the _z_ contract adds no
    // attribute of its own, so this matches _Out_writes_ -> NativeArrayInfo(CountParamIndex = 1).
    BOOL WriteString(_Out_writes_z_(size) unsigned short* buffer, DWORD size);

    // _Inout_updates_z_ -> #[r#in] #[out] + NativeArrayInfo(CountParamIndex = 1).
    BOOL UpdateString(_Inout_updates_z_(count) unsigned short* buffer, DWORD count);

    // _Out_writes_to_opt_ is an output buffer written up to a count; the [Out]
    // direction must be captured (the two-arg extent's first arg is the capacity)
    // -> #[out] #[opt] + NativeArrayInfo(CountParamIndex = 1). Without [Out] a
    // writable string buffer would be wrongly const-downgraded downstream.
    BOOL GetName(_Out_writes_to_opt_(size, *size) unsigned short* buffer, DWORD size);

    // _Out_writes_to_ whose write capacity is held in an in/out pointer parameter,
    // written `*len` in SAL (the RtlIdnToAscii idiom): the leading dereference
    // resolves to the pointer parameter `len` itself, so the buffer count is
    // captured -> #[len_param(1)]. Without stripping the `*` the count is dropped.
    BOOL GetPath(_Out_writes_to_(*len, *len) unsigned short* buffer, _Inout_ DWORD* len);

    // _Out_writes_all_ output buffer fully written -> #[out] + NativeArrayInfo.
    BOOL FillAll(_Out_writes_all_(count) DWORD* buffer, DWORD count);

    // _Inout_updates_to_ -> #[r#in] #[out] + NativeArrayInfo(CountParamIndex = 1).
    BOOL GrowBuffer(_Inout_updates_to_(count, *count) DWORD* buffer, DWORD count);

    // _Outptr_result_buffer_ marks an out-pointer; its element count is not captured
    // (the name carries no _reads_/_writes_/_updates_ stem).
    BOOL AllocBuffer(_Outptr_result_buffer_(count) void** buffer, DWORD count);

    // Complex size expression is not representable -> no size attribute.
    BOOL ReadScaled(_In_reads_bytes_(cb * 2) const void* buffer, DWORD cb);

    // _Reserved_ by value -> Reserved marker, In default (no direction attribute).
    BOOL DoWork(_In_ HANDLE object, _Reserved_ DWORD reserved);
}
