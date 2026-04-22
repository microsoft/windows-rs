struct Empty {
};

struct Numbers {
    unsigned char f1;
    unsigned short f2;
    unsigned int f3;
    unsigned long long f4;
    signed char f5;
    short f6;
    int f7;
    long long f8;
    float f9;
    double f10;
    unsigned long f11;
    long f12;
    wchar_t f13;
};

struct Named {
    Empty f1;
    Numbers f2;
};

struct Pointers {
    Named* f1;
    int* f2;
};

struct Arrays {
    unsigned char f1[16];
    int f2[4];
};

// Unions — renamed to avoid collisions with the struct types above.
union EmptyUnion {
};

union NumbersUnion {
    unsigned char f1;
    unsigned short f2;
    unsigned int f3;
    int f4;
};

struct WithUnion {
    EmptyUnion f1;
    NumbersUnion f2;
};

// Nested struct/union types within other structs/unions.
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
