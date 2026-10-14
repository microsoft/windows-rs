#include <sal.h>

#ifndef REVIEW_LIMIT
#define REVIEW_LIMIT 16
#endif

typedef _Return_type_success_(return >= 0) long Status;

struct BufferView {
    unsigned int count;
    _Field_size_(count) unsigned char* data;
};

extern "C" _Must_inspect_result_ Status ReadBuffer(
    _Out_writes_bytes_to_opt_(capacity, *written) void* buffer,
    _In_ unsigned int capacity,
    _Out_opt_ unsigned int* written);

extern "C" void ReadFixed(_In_reads_(REVIEW_LIMIT) const int* values);
extern "C" void ReadHook(_SAL2_Source_(_In_reads_, (REVIEW_LIMIT), ) const int* values);
extern "C" void InspectResult(_Outptr_result_maybenull_ void** result);
extern "C" void OptionalResult(_Outptr_opt_ void** result);
extern "C" void Conditional(_When_(capacity > 0, _Out_writes_(capacity)) void* buffer,
    unsigned int capacity);

struct __declspec(uuid("11111111-1111-1111-1111-111111111111")) IAnnotated {
    virtual Status __stdcall Get(_COM_Outptr_ void** object) = 0;
};
