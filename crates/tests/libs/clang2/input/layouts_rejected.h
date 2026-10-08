union NaturalUnion {
    int number;
    float real;
};

#pragma pack(push, 1)
union PackedUnion {
    int number;
    short other;
};
struct PackedNested {
    char tag;
    union {
        int number;
        float real;
    };
};
struct PackedField {
    char tag;
    NaturalUnion value;
};
#pragma pack(pop)

union BitfieldUnion {
    unsigned int value : 3;
    unsigned int other;
};

struct BitfieldNested {
    union {
        struct {
            unsigned int value : 3;
        };
        unsigned int other;
    };
};
