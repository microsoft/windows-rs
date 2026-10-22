struct Unsigned {
    unsigned int low : 3;
    unsigned int : 2;
    unsigned int high : 5;
    unsigned int tail;
};

struct Signed {
    signed int low : 3;
    signed int high : 5;
};

struct Separated {
    unsigned int first : 3;
    unsigned int : 0;
    unsigned int second : 7;
    char tail;
};

struct Widths {
    unsigned char a : 3;
    unsigned short b : 9;
    unsigned int c : 18;
    unsigned long long d : 40;
};

#pragma pack(push, 1)
struct Packed {
    char lead;
    unsigned int a : 3;
    unsigned int b : 5;
    short tail;
};
#pragma pack(pop)

struct Full {
    unsigned long long value : 64;
};

struct Computed {
    unsigned int value : sizeof(short) * 8;
};

struct Platform {
#if defined(_M_IX86)
    unsigned int value : 7;
#else
    unsigned long long value : 40;
#endif
};

struct Padding {
    unsigned int : 5;
    unsigned int value : 3;
    unsigned int : 0;
};

struct Anonymous {
    unsigned int : 7;
    unsigned int tail;
};

struct Trailing {
    unsigned int value : 4;
    unsigned int : 9;
};

union Overlay {
    unsigned int a : 3;
    signed int b : 5;
    unsigned int whole;
};

#pragma pack(push, 1)
union PackedOverlay {
    unsigned char small : 3;
    unsigned long long large : 40;
};
#pragma pack(pop)

union PlatformOverlay {
#if defined(_M_IX86)
    unsigned int value : 7;
    unsigned int whole;
#else
    unsigned long long value : 40;
    unsigned long long whole;
#endif
};

struct UnsignedPadding {
    signed int : 5;
    unsigned int value : 3;
    signed int : 7;
    unsigned int next : 4;
    signed int : 9;
};

struct SignedPadding {
    unsigned int : 5;
    signed int value : 3;
    unsigned int : 7;
};
