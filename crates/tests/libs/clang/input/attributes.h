struct __attribute__((packed)) Packed {
    char first;
    int value;
};

struct alignas(16) Aligned {
    char value;
};

struct FieldAligned {
    char first;
    __declspec(align(16)) int value;
};

#pragma pack(push, 1)
struct PragmaOne {
    char value;
};
#pragma pack(pop)

#pragma pack(push, 2)
struct PragmaTwo {
    char value;
};
#pragma pack(pop)
