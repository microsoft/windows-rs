//! library kernel32.dll
// Array *parameters* decay per C11 §6.7.6.3p7 (an array parameter is adjusted to a
// pointer). The scraper decays *every* array parameter to a pointer, matching the reference
// (win32metadata), whose ABI carries no by-value array parameter. An inline fixed-size array
// (`float m[4]`) additionally records its length as `#[len_const(N)]` (NativeArrayInfo) so
// bindgen reconstructs a length-checked `&[T; N]` in the safe wrapper only; an unsized
// flexible array (`T[]`) or a SAL-counted buffer carries the count from the SAL macro.

#define _Out_writes_(c) __attribute__((annotate("_Out_writes_(" #c ")")))
#define _In_ __attribute__((annotate("_In_")))

typedef unsigned short WCHAR;
typedef unsigned char UVersionInfo[4];

extern "C" {
    // Unsized flexible array parameter -> decays to a bare pointer (no count).
    void SumValues(int values[], unsigned int count);

    // Counted array parameter: the SAL count carries the length, so the array type
    // decays to a pointer -> #[len_const(80)] szName: *mut u16 (not [[u16; 80]; 80]).
    void GetName(_Out_writes_(80) WCHAR szName[80]);

    // `_In_` const inline fixed-size array -> decays to a pointer plus #[len_const(4)] so
    // bindgen projects a length-checked `&[f32; 4]` in the safe wrapper; the `_In_` fixes
    // the pointee const-ness (`*const f32`). This is the ID3D10Device::ClearRenderTargetView
    // `_In_ const FLOAT ColorRGBA[4]` idiom.
    void ClearColor(_In_ const float ColorRGBA[4]);

    // Unannotated *mutable* inline array (an output buffer with no [in]/[out] and no SAL
    // count) -> decays to a bare pointer with no count, matching the reference (e.g. ICU
    // `ucnv_getStarters`); the count is omitted so no spurious direction is inferred.
    void FillState(unsigned int state[2]);

    // A *named* array typedef parameter (`UVersionInfo` = `[u8; 4]`) -> decays to a bare
    // element pointer (`*mut u8`) with no count, dropping the alias, matching the reference
    // (e.g. ICU `u_getVersion`). The length lives on the typedef, not the parameter, so no
    // `NativeArrayInfo` is recorded.
    void GetVersion(UVersionInfo version);
}
