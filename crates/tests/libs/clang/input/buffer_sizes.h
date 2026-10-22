#include <sal.h>

typedef unsigned long Count;
#define CAPACITY capacity

typedef void (*ReadBytes)(
    _Out_writes_bytes_to_opt_(capacity, *written) void* buffer,
    Count capacity, Count* written);
typedef void (*WriteBytes)(
    Count capacity, _SAL2_Source_(_In_reads_bytes_, (CAPACITY), ) const unsigned short* buffer);
typedef void (*UpdateBytes)(
    _Inout_updates_bytes_opt_(capacity) unsigned int* buffer, Count capacity);
typedef void (*RepeatedBytes)(
    _Out_writes_bytes_(capacity) _Out_writes_bytes_to_(capacity, written) void* buffer,
    Count capacity, Count written);
typedef void (*Plain)(Count capacity, const unsigned short* buffer);
typedef void (*ConditionalBytes)(
    _When_(capacity > 0, _Out_writes_bytes_(capacity)) void* buffer, Count capacity);
typedef void (*ExpressionBytes)(
    _Out_writes_bytes_(capacity + 1) void* buffer, Count capacity);
typedef void (*IndirectBytes)(
    _Out_writes_bytes_(*capacity) void* buffer, Count* capacity);
typedef void (*QueryBytes)(
    _Out_writes_bytes_to_opt_(*size, *size) void* buffer, _Inout_ Count* size);
typedef void (*FullBytes)(
    _Out_writes_bytes_all_(*size) void* buffer, Count* size);
typedef void (*NestedCount)(
    _Out_writes_bytes_to_(capacity, **written) void* buffer, Count capacity, Count** written);
typedef void (*ConstantBytes)(_Out_writes_bytes_(16) void* buffer);
typedef void (*ElementCount)(
    _In_reads_(capacity) const unsigned short* buffer, Count capacity);
typedef void (*ElementOutput)(
    Count capacity, _Out_writes_to_opt_(capacity, written) unsigned short* buffer,
    Count written);
typedef void (*ElementUpdate)(
    _Inout_updates_(capacity) unsigned short* buffer, Count capacity);

struct __declspec(uuid("22222222-1111-1111-1111-111111111111")) IBuffer {
    virtual void __stdcall Read(
        Count capacity, _Out_writes_bytes_(capacity) void* buffer) = 0;
};
