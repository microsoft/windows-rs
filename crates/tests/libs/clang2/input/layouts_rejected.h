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
