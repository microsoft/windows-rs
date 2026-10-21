#include <sal.h>

typedef unsigned char Byte;
typedef unsigned long Count;
typedef Byte* Bytes;

typedef void (*Capacity)(_Outptr_result_buffer_(capacity) Byte** buffer, Count capacity);
typedef void (*Full)(_Outptr_result_buffer_all_(*length) const Byte** buffer, _Out_ Count* length);
typedef void (*Partial)(
    _Outptr_result_bytebuffer_to_(capacity, *length) Byte** buffer,
    Count capacity, _Out_ Count* length);
typedef void (*OptionalCounts)(
    _Outptr_result_bytebuffer_to_(*capacity, *length) Byte** buffer,
    _Out_opt_ Count* capacity, _Out_opt_ Count* length);
extern "C" void Fetch(
    _Outptr_result_bytebuffer_to_(*capacity, *length) Byte** buffer,
    _Out_opt_ Count* capacity, _Out_opt_ Count* length);
typedef void (*OptionalStorage)(
    _Outptr_opt_result_bytebuffer_to_maybenull_(*capacity, *length) Byte** buffer,
    _Out_opt_ Count* capacity, _Out_opt_ Count* length);
typedef void (*AliasStorage)(_Outptr_result_buffer_(capacity) Bytes* buffer, Count capacity);
typedef void (*PointerElements)(_Outptr_result_buffer_all_(length) Byte*** buffer, Count length);
typedef void (*Callback)();
typedef void (*CallbackElements)(
    _Outptr_result_buffer_all_(length) Callback** buffer, Count length);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IObject {
    virtual void __stdcall Invoke() = 0;
};
typedef void (*InterfaceElements)(
    _Outptr_result_buffer_all_(length) IObject*** buffer, Count length);
typedef void (*Repeated)(
    _Outptr_result_buffer_(capacity) _Outptr_result_buffer_(capacity) Byte** buffer,
    Count capacity);
typedef void (*FailurePointer)(
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(capacity) Byte** buffer,
    Count capacity);
typedef void (*FailurePointerReversed)(
    _Outptr_result_buffer_(capacity) _Outptr_result_nullonfailure_ Byte** buffer,
    Count capacity);
typedef void (*MismatchedNullable)(
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_maybenull_(capacity) Byte** buffer,
    Count capacity);
typedef void (*MismatchedOptional)(
    _Outptr_result_nullonfailure_ _Outptr_opt_result_buffer_(capacity) Byte** buffer,
    Count capacity);
typedef void (*Distinct)(
    _Outptr_result_buffer_(capacity) _Outptr_result_buffer_all_(capacity) Byte** buffer,
    Count capacity);
typedef void (*Constant)(_Outptr_result_buffer_all_(0) Byte** buffer);
typedef void (*Expression)(_Outptr_result_buffer_(capacity + 1) Byte** buffer, Count capacity);
typedef void (*OptionalIncoming)(
    _Out_writes_bytes_all_(*length) Byte* buffer, _Inout_opt_ Count* length);
typedef void (*Fallback)(
    _Out_writes_to_(capacity, length ? *length : capacity) Byte* buffer,
    Count capacity, _Out_opt_ Count* length);
