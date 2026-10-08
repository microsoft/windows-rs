struct Outer {
    struct { int value; } inner;
};
using Inner = decltype(Outer::inner);
extern "C" void ByValue(Outer value);
