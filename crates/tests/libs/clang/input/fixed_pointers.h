typedef int Element;
typedef Element* __ptr64 Address64;
typedef const Element* __ptr64 ConstAddress64;
typedef Address64 InheritedAddress64;
struct Opaque;
struct Addresses {
    int prefix;
    Address64 address;
    ConstAddress64 constant;
    InheritedAddress64 inherited;
    Address64 array[2];
    Address64* storage;
    Address64 const* constant_storage;
    const int* __ptr64* mixed_outer;
    int** __ptr64 mixed_inner;
    Opaque* __ptr64 opaque;
    struct Node { int value; }* __ptr64 node;
};
union Segment {
    void* __ptr64 buffer;
    unsigned long long alignment;
};
#pragma pack(push, 1)
struct PackedAddresses {
    char prefix;
    Address64 address;
};
#pragma pack(pop)
static_assert(__is_same(decltype(Addresses::constant_storage), Address64 const*));
