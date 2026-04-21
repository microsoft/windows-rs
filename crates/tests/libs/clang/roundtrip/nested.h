// Tests nested named struct and union declarations within outer struct/union types.

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
