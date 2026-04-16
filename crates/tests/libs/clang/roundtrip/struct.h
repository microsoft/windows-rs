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
};

struct Named {
    struct Empty f1;
    struct Numbers f2
};

struct Pointers {
    struct Named* f1;
    int* f2;
};
