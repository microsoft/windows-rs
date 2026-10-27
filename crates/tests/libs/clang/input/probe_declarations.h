typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;
#define PROBE_IDENTITY __uuidof((struct __declspec(uuid("12345678-1234-5678-90ab-cdef01234567")) ProbeIdentity*)0)
#ifdef AUTHORED_EXPANSION
const GUID AuthoredIdentity = PROBE_IDENTITY;
#endif
