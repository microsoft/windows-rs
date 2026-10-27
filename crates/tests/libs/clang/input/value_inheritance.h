struct Base {
    int count;
    void* data;
};
typedef Base BaseAlias;
struct Derived : Base {
    unsigned int flags;
};
struct Leaf : Derived {
    unsigned char marker;
};
struct AliasDerived : BaseAlias {
    int extra;
};
struct BaseOnly : Base {};
struct Shadow : Base {
    int count;
};
#pragma pack(push, 1)
struct Packed : Base {
    char tag;
};
#pragma pack(pop)
struct Container {
    Derived items[2];
    Leaf* reference;
};
typedef void (__stdcall *Inspect)(const Derived* value, Leaf& other);
