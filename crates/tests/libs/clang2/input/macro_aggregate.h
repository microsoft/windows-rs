struct NativeGuid {
    unsigned a;
    unsigned short b, c;
    unsigned char d[8];
};
#ifdef AS_MACRO
#define ID NativeGuid{1, 2, 3, {4, 5, 6, 7, 8, 9, 10, 11}}
#else
const NativeGuid ID = {1, 2, 3, {4, 5, 6, 7, 8, 9, 10, 11}};
#endif
#define HANDLE_VALUE ((void*)-1)
#define SCALAR_VALUE 42
