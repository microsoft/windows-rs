#pragma pack(push, 1)
struct PackOne {
    char tag;
    int value;
};
#pragma pack(pop)

#pragma pack(push, 2)
struct PackTwo {
    char tag;
    short value;
};
#pragma pack(pop)

#pragma pack(push, 4)
struct PackFour {
    char tag;
    short value;
};
#pragma pack(pop)

#define RECORD_ALIGNMENT (8 + 8)
struct __declspec(align(RECORD_ALIGNMENT)) Aligned {
    int value;
};

struct __declspec(align(1)) AlignOne {
    int value;
};

struct __declspec(align(4)) AlignFour {
    int value;
};

#ifdef __clang__
struct __attribute__((packed)) PackedAttr {
    char tag;
    int value;
};

struct __attribute__((aligned(16))) GnuAligned {
    int value;
};
#endif

struct alignas(16) CxxAligned {
    int value;
};

struct FieldAligned {
    char tag;
    __declspec(align(16)) int value;
};

#pragma pack(push, 2)
struct PackedPointer {
    char tag;
    void *value;
};

union PackedUnion {
    short code;
    double value;
};

struct PackedContainer {
    char tag;
    Aligned values[2];
    PackOne bytes;
};

struct PackedOuter {
    char tag;
    PackedContainer value;
};
#pragma pack(pop)
