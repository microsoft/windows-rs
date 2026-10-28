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

typedef Status (__stdcall *AContract)(
    _When_(capacity > 0, _Out_writes_bytes_to_(capacity, *written)) void* buffer,
    _In_ unsigned int capacity,
    _Out_ unsigned int* written);
typedef Status (__stdcall *ZPlain)(void*, unsigned int, unsigned int*);
typedef void (*Scoped)(
    _When_(capacity > 0, _At_(value, _Group_(_In_reads_(REVIEW_LIMIT) _Always_(_In_))))
    int* value, unsigned int capacity);
typedef void (*ScopedBuffer)(
    _At_buffer_(value, index, capacity, _When_(index > 0, _Out_))
    int* value, unsigned int capacity);
typedef void (*Unscoped)(int* value, unsigned int capacity);
typedef _On_failure_(_Unchanged_(*value)) void (*Failure)(int* value);
typedef AContract CopiedContract;
typedef AContract* ContractAddress;
struct CallbackHolder {
    AContract named;
    ZPlain plain;
    Status (__stdcall *inferred)(void*, unsigned int, unsigned int*);
};
struct OnlyInferred {
    Status (__stdcall *callback)(void*, unsigned int, unsigned int*);
};
typedef _Must_inspect_result_ Status (*AResultContract)(void* context);
typedef AResultContract CopiedResult;
typedef AResultContract* ResultAddress;
struct ResultOnly {
    Status (*callback)(void*);
};

typedef _Struct_size_bytes_(bytes) struct _Struct_size_bytes_(bytes) NativeTagged {
    unsigned int bytes;
    int value;
} Tagged;

typedef _Return_type_success_(return >= 0) enum NativeCode {
    CodeOk = 0,
    CodeError = -1,
} Code;
