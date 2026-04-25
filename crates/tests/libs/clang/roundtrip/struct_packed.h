// Structs with explicit packing via #pragma pack.

#pragma pack(push, 1)
struct Packed1 {
    unsigned char a;
    unsigned int b;
    unsigned short c;
};
#pragma pack(pop)

#pragma pack(push, 2)
struct Packed2 {
    unsigned short a;
    unsigned int b;
    unsigned long long c;
};
#pragma pack(pop)

#pragma pack(push, 4)
struct Packed4 {
    unsigned int a;
    unsigned long long b;
};
#pragma pack(pop)

// An unpacked struct — must NOT get a #[packed] attribute.
struct Unpacked {
    unsigned char a;
    unsigned int b;
};
