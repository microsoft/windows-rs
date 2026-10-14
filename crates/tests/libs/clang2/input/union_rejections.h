union Bad {
#if defined(_M_IX86)
    bool bits : 1;
#else
    unsigned bits;
#endif
};
struct BadConsumer { Bad* value; };
struct Nested {
    union { int integer; float real; } value;
};
struct NestedConsumer { Nested* value; };
struct Safe { int value; };
