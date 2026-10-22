#include <sal.h>
#include <rpcsal.h>

typedef unsigned char Byte;
typedef unsigned long Count;
typedef Byte* Bytes;

typedef void (*Capacity)(_Outptr_result_buffer_(capacity) Byte** buffer, Count capacity);
typedef void (*Full)(_Outptr_result_buffer_all_(*length) const Byte** buffer, _Out_ Count* length);
typedef void (*Partial)(
    _Outptr_result_bytebuffer_to_(capacity, *length) Byte** buffer,
    Count capacity, _Out_ Count* length);
typedef void (*RpcCapacity)(__RPC__deref_out_ecount(capacity) Byte** buffer, Count capacity);
typedef void (*RpcPartial)(
    __RPC__deref_out_ecount_part(capacity, *length) Byte** buffer,
    Count capacity, _Out_ Count* length);
typedef void (*RpcFull)(__RPC__deref_out_ecount_full(*length) Byte** buffer, _Out_ Count* length);
typedef void (*RpcNullable)(
    __RPC__deref_out_ecount_full_opt(*length) Byte** buffer, _Out_ Count* length);
typedef void (*RpcNullableCapacity)(
    __RPC__deref_out_ecount_opt(capacity) Byte** buffer, Count capacity);
typedef void (*RpcNullablePartial)(
    __RPC__deref_out_ecount_part_opt(capacity, *length) Byte** buffer,
    Count capacity, _Out_ Count* length);
typedef _Ret_maybenull_ Byte* (*NullableResult)();
typedef NullableResult NullableResultAlias;
typedef _Ret_maybenull_ Count (*ScalarResult)();
extern "C" _Ret_maybenull_ Byte* Lookup();
typedef void (*RpcNullableFullString)(
    __RPC__deref_out_ecount_full_opt_string(*length) wchar_t** text, _Out_ Count* length);
typedef void (*RpcString)(__RPC__deref_out_opt_string wchar_t** text);
typedef void (*RpcReturnedString)(__RPC__deref_out_string wchar_t** text);
typedef void (*RpcFullString)(
    __RPC__deref_out_ecount_full_string(*length) wchar_t** text, _Out_ Count* length);
typedef void (*ReturnedString)(_Outptr_result_z_ wchar_t** text);
typedef void (*OptionalReturnedString)(_Outptr_opt_result_z_ wchar_t** text);
typedef void (*NullableReturnedString)(_Outptr_result_maybenull_z_ wchar_t** text);
typedef void (*OptionalNullableReturnedString)(_Outptr_opt_result_maybenull_z_ wchar_t** text);
typedef void (*CountedString)(
    _Outptr_result_z_ _Outptr_result_buffer_(capacity) wchar_t** text, Count capacity);
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
    virtual _Ret_maybenull_ Byte* __stdcall Find() = 0;
};
struct NullableFields {
    _Maybenull_ Bytes data;
    _Maybenull_ IObject* object;
    _Maybenull_ Count scalar;
    _Maybenull_ _Field_size_(length) Byte* sized;
    Count length;
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
