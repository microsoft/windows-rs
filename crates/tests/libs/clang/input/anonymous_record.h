// Anonymous struct/union *members* (the C11 / MSVC anonymous aggregate idiom):
// an aggregate declared inside another with no member declarator, whose fields
// are promoted into the parent. libclang emits no `FieldDecl` for these, so they
// must be reconstructed as `Anonymous`, `Anonymous2`, … fields of the lifted
// nested type. This mirrors the shape of `PROPVARIANT` in the Windows SDK.
struct Variant {
    unsigned short tag;
    union {
        struct {
            int a;
            int b;
        };
        double d;
    };
    union {
        float first;
        long second;
    };
};
