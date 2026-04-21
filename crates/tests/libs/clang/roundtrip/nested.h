// Tests nested struct/union types within other structs/unions.

// Named nested types.
struct Outer {
    struct Inner {
        int x;
        int y;
    } pos;
    union Variant {
        int i;
        float f;
    } val;
};

union TaggedUnion {
    struct TaggedFirst {
        int kind;
        int value;
    } first;
    struct TaggedSecond {
        int kind;
        float fvalue;
    } second;
};

// Unnamed (anonymous) nested types.  Each receives a synthesised name
// {Outer}_{index}, where index is the 0-based position of the nested
// definition among all struct/union definitions in the parent body —
// matching the scheme used by the windows-rdl writer when un-nesting
// NestedPublic types from Windows metadata.

struct WithAnon {
    struct { int x; int y; } pos; // → WithAnon_0
    union  { int i; float f; } val; // → WithAnon_1
};

// Deeply-nested anonymous types: anonymous inside anonymous.
struct DeepNested {
    struct {                           // → DeepNested_0
        union { int a; float b; } u;   // → DeepNested_0_0
        int c;
    } outer;
};
