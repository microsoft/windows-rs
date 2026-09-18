//! args -x c++ -fms-extensions --target=x86_64-pc-windows-msvc

typedef struct DEVICE_CONTROL {
    unsigned long OutputBufferLength;
    __declspec(align(8)) unsigned long InputBufferLength;
    __declspec(align(8)) unsigned long IoControlCode;
    void* Type3InputBuffer;
} DEVICE_CONTROL;

typedef struct QUERY_SECURITY {
    unsigned long SecurityInformation;
    __declspec(align(8)) unsigned long Length;
} QUERY_SECURITY;

typedef struct WIDE_ALIGNMENT {
    unsigned char First;
    __declspec(align(32)) unsigned long Value;
} WIDE_ALIGNMENT;
