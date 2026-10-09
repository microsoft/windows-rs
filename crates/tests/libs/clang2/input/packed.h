#if defined(_MSC_VER) && !defined(__clang__)
#pragma warning(push)
#pragma warning(disable: 4121 4201 4324)
#endif

union NaturalChoice {
    unsigned long value;
    float real;
};

#pragma pack(push, 1)
struct Packed1 {
    unsigned char tag;
    unsigned long value;
    unsigned long long wide;
    void* pointer;
};
union PackedChoice {
    Packed1 record;
    unsigned long long bits;
};
struct PackedAnonymous {
    unsigned char tag;
    union {
        unsigned long value;
        unsigned short halves[2];
    };
};
struct PackedField {
    unsigned char tag;
    NaturalChoice choice;
};
#pragma pack(pop)

#pragma pack(push, 2)
struct Packed2 {
    unsigned char tag;
    unsigned long values[2];
    PackedChoice choice;
};
struct PackedGap {
    unsigned char __padding1;
    __declspec(align(2)) unsigned char marker;
    unsigned long value;
};
#pragma pack(pop)

#pragma pack(push, 4)
struct Packed4 {
    unsigned char tag;
    unsigned long long wide;
    void* pointer;
};
#pragma pack(pop)

struct PackedContainer {
    Packed1 records[2];
    unsigned long tail;
};

typedef void (*PackedCallback)(Packed1* packet);
extern "C" unsigned long PackedLayoutEvidence(unsigned long index);
extern "C" void PackedMutate(Packed1* packet);
extern "C" void PackedInvoke(PackedCallback callback, Packed1* packet);

#if defined(_MSC_VER) && !defined(__clang__)
#pragma warning(pop)
#endif
