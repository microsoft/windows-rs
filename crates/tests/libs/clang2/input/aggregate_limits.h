struct Id {
    unsigned long a;
    unsigned short b;
    unsigned short c;
    unsigned char d[8];
};
const Id Partial = { 1, 2, 3, { 4 } };
int RuntimeValue();
struct Scalar { int value; };
const Scalar Dynamic = { RuntimeValue() };
struct Other {
    unsigned long a;
    unsigned short b;
    unsigned short c;
    unsigned char d[7];
};
const Other WrongShape = { 1, 2, 3, { 1, 2, 3, 4, 5, 6, 7 } };
struct Signed {
    long a;
    unsigned short b;
    unsigned short c;
    unsigned char d[8];
};
const Signed WrongSignedness = { 1, 2, 3, { 1, 2, 3, 4, 5, 6, 7, 8 } };
