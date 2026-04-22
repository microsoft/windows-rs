enum Name {
    One = 1,
    Two = 2,
};

enum {
    ONE = 1,
    TWO,
};

// Enum used as a function parameter.
enum FooEnum { A, B, C };
void FooFunction(/* [in] */ enum FooEnum foo);
