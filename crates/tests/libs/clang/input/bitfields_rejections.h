struct Mixed {
    unsigned int a : 3;
    signed int b : 5;
};

#ifdef TOO_WIDE
struct TooWide {
    unsigned char value : 9;
};
#endif

enum State { Off, On };
struct EnumBits {
    State value : 2;
};

struct BoolBits {
    bool value : 1;
};

struct Collision {
    unsigned int _bitfield;
    unsigned int value : 3;
};

struct AlignedField {
    __declspec(align(8)) unsigned int value : 3;
};

struct Dependent {
    Mixed value;
};

struct Safe {
    unsigned int value : 3;
};

union Underaligned {
    unsigned int value : 3;
};
