//! namespace PointerSized
//! library api.dll
//! args -x c++ --target=x86_64-pc-windows-msvc -fms-extensions

typedef unsigned __int64 ULONG_PTR;
             typedef ULONG_PTR SIZE_T;
             void Allocate(SIZE_T size);
