#include <sal.h>
#include <rpcsal.h>

#define FIXED_COUNT (31 + 1)
typedef unsigned long Count;
typedef unsigned char Byte;

typedef void (*ReadFixed)(_In_reads_(FIXED_COUNT) const Byte* buffer);
typedef void (*FullFixed)(_Out_writes_all_((31) + (1)) Byte* buffer);
typedef void (*ZeroReturned)(_Outptr_result_buffer_all_(0) Byte** buffer);
typedef void (*InputOffset)(_In_reads_((count) + 1) const Byte* buffer, Count count);
typedef void (*FullOffset)(
    _Out_writes_bytes_all_((*count) + 1) Byte* buffer, _Inout_opt_ Count* count);
typedef void (*PartialOffset)(
    _Out_writes_to_(capacity, *length + 1) Byte* buffer,
    Count capacity, _Out_opt_ Count* length);
typedef void (*FallbackOffset)(
    _Out_writes_to_(capacity, length ? *length + 1 : capacity) Byte* buffer,
    Count capacity, _Out_opt_ Count* length);
typedef void (*ReturnedOffset)(
    _Outptr_result_bytebuffer_to_(*capacity + 1, *length + 1) Byte** buffer,
    _Out_opt_ Count* capacity, _Out_opt_ Count* length);
typedef void (*RpcFull)(__RPC__out_ecount_full(*count + 1) Byte* buffer, Count* count);
typedef void (*RpcFixed)(__RPC__in_ecount(32) const Byte* buffer);

extern "C" void ReadCount(_In_reads_(count + 1) const Byte* buffer, Count count);

typedef void (*Octal)(_Out_writes_(010) Byte* buffer);
typedef void (*Hexadecimal)(_Out_writes_(0x20) Byte* buffer);
typedef void (*Suffix)(_Out_writes_(32u) Byte* buffer);
typedef void (*Overflow)(_Out_writes_(2147483647 + 1) Byte* buffer);
typedef void (*Huge)(_Out_writes_(18446744073709551616) Byte* buffer);
typedef void (*Negative)(_Out_writes_(-1) Byte* buffer);
typedef void (*Product)(_Out_writes_(count * 2) Byte* buffer, Count count);
typedef void (*Nested)(_Out_writes_(**count + 1) Byte* buffer, Count** count);
typedef void (*PointerArithmetic)(_Out_writes_(*(count + 1)) Byte* buffer, Count* count);
typedef void (*Cast)(_Out_writes_((Count)count + 1) Byte* buffer, Count count);
typedef Count (*ReturnLength)(
    _Out_writes_to_(capacity, return + 1) Byte* buffer, Count capacity);
