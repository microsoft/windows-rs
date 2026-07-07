//! library kernel32.dll
// C `const` is captured structurally: a pointer-to-const lowers to `*const T`
// (RDL `PtrConst`, which the winmd writer encodes as the `IsConst` modreq), and
// a reference-to-const to `RefConst`. No synthetic `ConstAttribute` is emitted —
// the constness lives in the type signature itself.

typedef unsigned short WCHAR;

// A struct field whose type is a pointer-to-const.
struct StringView {
    const WCHAR* data;
    unsigned int length;
};

extern "C" {
    // Return type is a pointer-to-const.
    const WCHAR* GetName(void);

    // Pointer-to-pointer-to-const (the inner const is preserved).
    const WCHAR** GetNames(unsigned int* count);
}
