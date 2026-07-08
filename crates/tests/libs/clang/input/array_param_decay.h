//! library kernel32.dll
// Array *parameters* decay per C11 §6.7.6.3p7 (an array parameter is adjusted to a
// pointer). The scraper decays an array parameter to a pointer only when the array is
// unsized (a C flexible array `T[]`) or carries a SAL element count (`_Out_writes_(N)
// T buf[N]`, whose length is already expressed by the count) — keeping both the array
// type and the count would double-encode and make bindgen wrap `&mut [[T; N]; N]`. A
// plain fixed-size array parameter with no count (`float m[4]`) is preserved as `[T; N]`
// so bindgen can project a length-checked `&[T; N]`.

#define _Out_writes_(c) __attribute__((annotate("_Out_writes_(" #c ")")))

typedef unsigned short WCHAR;

extern "C" {
    // Unsized flexible array parameter -> decays to a pointer.
    void SumValues(int values[], unsigned int count);

    // Counted array parameter: the SAL count carries the length, so the array type
    // decays to a pointer -> #[len_const(80)] szName: *mut u16 (not [[u16; 80]; 80]).
    void GetName(_Out_writes_(80) WCHAR szName[80]);

    // Plain fixed-size array parameter, no count -> preserved as [f32; 4] so bindgen
    // projects a length-checked `&[f32; 4]` (the ID3D10Device::ClearRenderTargetView
    // ColorRGBA[4] idiom).
    void ClearColor(const float ColorRGBA[4]);
}
