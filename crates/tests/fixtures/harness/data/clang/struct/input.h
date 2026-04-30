struct Empty {
};

struct Numbers {
    unsigned char f1;
    unsigned short f2;
    unsigned int f3;
    unsigned long long f4;
    signed char f5;
    short f6;
    int f7;
    long long f8;
    float f9;
    double f10;
    unsigned long f11;
    long f12;
    wchar_t f13;
};

struct Named {
    Empty f1;
    Numbers f2;
};

struct Pointers {
    Named* f1;
    int* f2;
};

struct Arrays {
    unsigned char f1[16];
    int f2[4];
};

struct ConstPtrs {
    const int* read_only;
    const unsigned char* bytes;
};
