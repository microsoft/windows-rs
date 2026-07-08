//! library test.dll
// Forced over-alignment from `__declspec(align(N))` / `alignas(N)` raises a
// type's alignment *above* its natural field alignment. The winmd `ClassLayout`
// can only lower alignment (packing), so this is surfaced as `#[align(N)]` and
// carried through to `#[repr(C, align(N))]` by bindgen. This is the `CONTEXT` /
// #3761 fidelity class that win32metadata cannot represent.

// Leaf type whose declared alignment (16) exceeds the natural alignment of its
// two 8-byte fields. Mirrors `M128A` from `winnt.h`.
struct __declspec(align(16)) M128A {
    unsigned long long Low;
    long long High;
};

// Inherits 16-byte alignment through its `M128A` field, so it needs no attribute
// of its own — the alignment is derived from the field and propagated by bindgen.
struct XmmFrame {
    M128A Xmm0;
    unsigned long Flags;
};

// The C++ `alignas` keyword form, raising alignment to 32.
struct alignas(32) OverAligned {
    int value;
};

// Regression: a struct whose largest alignment comes from an *anonymous* aggregate
// member (here an 8-byte-aligned union holding a pointer) must NOT be flagged as
// forced-over-aligned — its 8-byte alignment is natural, derived from the member,
// not a declared `__declspec(align)`. Mirrors `STRRET` from `shtypes.h`, which a
// `packed(1)` parent (`SHELLDETAILS`) then embeds; a spurious `#[align(8)]` here
// would make Rust reject the packed parent (E0588).
struct NaturalAnon {
    unsigned int uType;
    union {
        void *ptr;
        unsigned int off;
    };
};
