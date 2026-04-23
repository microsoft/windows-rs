// Tests nested struct/union types within other structs/unions.
//
// All nested types (named or anonymous) receive a synthesised name
// {Outer}_{index}, where index is the 0-based position of the nested
// definition among all struct/union definitions in the parent body.
// Named types are renamed to avoid collisions across different outer types.

struct Outer {
    struct Inner {
        int x;
        int y;
    } pos;   // Inner → Outer_0
    union Variant {
        int i;
        float f;
    } val;   // Variant → Outer_1
};

union TaggedUnion {
    struct TaggedFirst {
        int kind;
        int value;
    } first;   // TaggedFirst → TaggedUnion_0
    struct TaggedSecond {
        int kind;
        float fvalue;
    } second;  // TaggedSecond → TaggedUnion_1
};

struct WithAnon {
    struct { int x; int y; } pos; // anonymous → WithAnon_0
    union  { int i; float f; } val; // anonymous → WithAnon_1
};

// Deeply-nested: anonymous inside anonymous.
struct DeepNested {
    struct {                           // anonymous → DeepNested_0
        union { int a; float b; } u;   // anonymous → DeepNested_0_0
        int c;
    } outer;
};
