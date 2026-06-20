// Struct with nested anonymous struct and union.
struct Outer {
    int tag;
    struct {
        int a;
        int b;
    } inner;
    union {
        float f;
        int i;
    } value;
};
