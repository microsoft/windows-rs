#include <sal.h>

struct QueryId {
    unsigned int data1;
    unsigned short data2;
    unsigned short data3;
    unsigned char data4[8];
};

extern "C" void Input(_In_reads_(count) const int* values, _In_ unsigned int count);
extern "C" void Fixed(_In_reads_(4) const int* values);
extern "C" void Optional(_In_reads_opt_(count) const int* values, _In_ unsigned int count);
extern "C" void InOut(_Inout_updates_(count) int* values, _In_ unsigned int count);
extern "C" void InOutWritten(
    _Inout_updates_to_(capacity, *written) int* values,
    _In_ unsigned int capacity,
    _Out_ unsigned int* written);
extern "C" void InOutOptionalWritten(
    _Inout_updates_to_(capacity, *written) int* values,
    _In_ unsigned int capacity,
    _Out_opt_ unsigned int* written);
extern "C" void Bytes(_In_reads_bytes_(count) const unsigned char* values, _In_ unsigned int count);
extern "C" _Success_(return != 0) int Output(
    _Out_writes_to_(capacity, *written) int* values,
    _In_ unsigned int capacity,
    _Out_ unsigned int* written);
extern "C" void Arithmetic(_In_reads_(count + 1) const int* values, _In_ unsigned int count);
extern "C" void OptionalCount(
    _Out_writes_to_(capacity, *written) int* values,
    _In_ unsigned int capacity,
    _Out_opt_ unsigned int* written);
extern "C" void IncomingCount(
    _Out_writes_all_(*count) int* values, _Inout_ unsigned int* count);
extern "C" void Returned(
    _Outptr_result_buffer_(*count) int** values, _Out_ unsigned int* count);

struct BufferView {
    unsigned int count;
    _Field_size_(count) int* values;
};

struct __declspec(uuid("6b0f3f5b-d5b3-4ba7-8fd7-978325d413cb")) IBufferQuery {
    virtual long __stdcall Query(
        _In_ const QueryId* iid,
        _COM_Outptr_
        _At_(*value, _Post_readable_size_(_Inexpressible_(varies)))
        void** value) = 0;
    virtual long __stdcall QueryArray(
        _In_ const QueryId* iid,
        _COM_Outptr_ _Outptr_result_buffer_(2) void** values) = 0;
};
