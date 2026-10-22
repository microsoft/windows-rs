typedef unsigned int Word;
typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;
extern "C" {
extern const Word NUMBER = 42;
extern const void* const ADDRESS = (const void*)-1;
extern const GUID GUID_VALUE = {0x12345678, 0x9abc, 0xdef0, {1, 2, 3, 4, 5, 6, 7, 8}};
}
