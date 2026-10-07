#ifdef _WIN64
#define POINTER_ALIGNMENT __declspec(align(8))
#else
#define POINTER_ALIGNMENT
#endif

struct DeviceIoControl {
    unsigned long OutputBufferLength;
    unsigned long POINTER_ALIGNMENT InputBufferLength;
    unsigned long POINTER_ALIGNMENT IoControlCode;
    void* Type3InputBuffer;
};

struct QuerySecurity {
    unsigned long SecurityInformation;
    unsigned long POINTER_ALIGNMENT Length;
};
