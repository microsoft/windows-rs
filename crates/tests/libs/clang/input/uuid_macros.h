typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;
struct __declspec(uuid("12345678-1234-5678-90ab-cdef01234567")) NativeIdentity;
#define IDENTITY_VALUE __uuidof(NativeIdentity)
