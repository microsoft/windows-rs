union Empty {
};

union Numbers {
    unsigned char f1;
    unsigned short f2;
    unsigned int f3;
    int f4;
};

struct WithUnion {
    Empty f1;
    Numbers f2;
};
