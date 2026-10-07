struct Adjusted {
    unsigned char __padding1;
    __declspec(align(8)) unsigned long value;
};
struct Outer {
    unsigned char tag;
    Adjusted value;
};
struct __declspec(align(16)) Aligned {
    unsigned long value;
};
extern "C" void ByValue(Outer value);
extern "C" Aligned ReturnValue();
extern "C" void ByPointer(Outer* value);
using OuterAlias = Outer;
extern "C" void AliasByValue(OuterAlias value);
