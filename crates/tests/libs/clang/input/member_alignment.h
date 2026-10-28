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
struct Request {
    DeviceIoControl device;
    QuerySecurity security;
};
struct Survivor { unsigned long value; };
struct AlignedValue { __declspec(align(8)) unsigned long Value; };
struct AlignedArray {
    char tag;
    __declspec(align(16)) unsigned short values[3];
    char tail;
};
union AlignedUnion {
    __declspec(align(8)) unsigned long value;
    char bytes[3];
};
struct PaddingName {
    unsigned long __padding_1;
    __declspec(align(8)) unsigned long value;
};
struct FloatGap {
    float first;
    __declspec(align(8)) float second;
};
struct ArrayContainer {
    AlignedArray values[2];
    char tail;
};
struct AlignedPointer {
    char tag;
    __declspec(align(16)) void* value;
};
struct PointerBefore {
    void* first;
    __declspec(align(16)) unsigned long value;
};
#ifndef MEMBER_REQUEST
#define MEMBER_REQUEST 8
#endif
struct __declspec(align(16)) AlignmentConflict {
    __declspec(align(MEMBER_REQUEST)) char value;
};
#ifdef _WIN64
static_assert(__builtin_offsetof(DeviceIoControl, InputBufferLength) == 8);
static_assert(__builtin_offsetof(DeviceIoControl, IoControlCode) == 16);
static_assert(__builtin_offsetof(DeviceIoControl, Type3InputBuffer) == 24);
static_assert(sizeof(DeviceIoControl) == 32);
static_assert(__builtin_offsetof(QuerySecurity, Length) == 8);
static_assert(sizeof(QuerySecurity) == 16);
#endif
