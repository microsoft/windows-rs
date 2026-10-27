#pragma once
typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;
typedef GUID IID;
typedef GUID CLSID;
#define EXTERN_C extern "C"
#define DECLSPEC_SELECTANY __declspec(selectany)
#define EXTERN_GUID(itf, l1, s1, s2, c1, c2, c3, c4, c5, c6, c7, c8) \
    EXTERN_C const IID DECLSPEC_SELECTANY itf = {l1, s1, s2, {c1, c2, c3, c4, c5, c6, c7, c8}}
#define CLSID_SCOPE __declspec(selectany) extern
