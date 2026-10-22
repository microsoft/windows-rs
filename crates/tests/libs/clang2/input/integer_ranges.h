#include <sal.h>
#include <rpcsal.h>

typedef unsigned int Count;
typedef void (*Slots)(
    _In_range_(0, (14) - 1) Count start,
    _In_range_(0, (14) - start) Count count);
typedef void (*Positive)(_In_range_(>, 0) int count);
typedef void (*AtLeastOne)(_In_range_(>=, 1) Count count);
typedef void (*Negative)(_In_range_(-8, -1) int count);
typedef void (*Grouped)(_In_range_(0, 128 - (start + count)) Count end, Count start, Count count);
typedef void (*Promoted)(_In_range_(0, 14 - start) Count count, unsigned short start);
typedef void (*RpcRange)(
    __RPC__in_range(0, 0xff) Count format,
    __RPC__in_range(1, 0X7fffffff) Count count,
    __RPC__in_range(0, capacity - 1) Count length, Count capacity);
typedef void (*UnsupportedHex)(
    _In_range_(0, 0x80000000) Count unsigned_bound,
    _In_range_(0, 0xffu) Count suffixed_bound);
typedef void (*Unsupported)(
    _In_range_(0, sizeof(Count)) Count size,
    _In_range_(0, (Count)size) Count cast,
    _In_range_(0, 4 * 128) Count product,
    _In_range_(0, 2147483647 + 1) Count overflow,
    _In_range_(0, 1) float floating);

typedef unsigned char Byte;
typedef void (*Remaining)(
    _In_reads_(capacity - start) const Byte* values, Count capacity, Count start);
typedef void (*Combined)(
    _Out_writes_(first + second) Byte* values, Count first, Count second);
typedef void (*ReturnedArithmetic)(
    _Outptr_result_buffer_to_(*capacity - start, *written - start) Byte** values,
    _Out_opt_ Count* capacity, _Out_opt_ Count* written, Count start);
typedef void (*InoutArithmetic)(
    _Inout_updates_all_(*count - start) Byte* values, _Inout_ Count* count, Count start);
typedef void (*UnsupportedArithmetic)(
    _In_reads_(count - start) const Byte* values, float count, float start);
typedef void (*Products)(
    _Out_writes_((count + extra) * 2) Byte* first,
    _Out_writes_(count + extra * 2) Byte* second, Count count, Count extra);
typedef void (*InputBound)(_Deref_in_range_(0, 64) const Count* value);
typedef void (*OutputBound)(
    _Out_opt_ _Deref_out_range_(0, capacity) Count* written, Count capacity);
typedef void (*PointerRange)(
    _Out_ _Out_range_(0, capacity) Count* written, Count capacity);
struct RangedFields {
    Count capacity;
    _Field_range_(0, capacity) Count length;
    _Field_range_(1, 4) Count mode;
    _Field_range_(0, 1) float floating;
    _Field_range_(0, floating) Count unsupported;
};

extern "C" _Ret_range_(1, capacity) Count ReturnCount(Count capacity);
typedef _Ret_range_(0, capacity - start) Count (*ReturnRemaining)(Count capacity, Count start);
typedef _Ret_range_(0, 1) float (*ReturnFloat)();
typedef _Ret_range_(0, 1) Count* (*ReturnPointer)();
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IRange {
    virtual _Ret_range_(0, capacity) Count __stdcall Get(Count capacity) = 0;
};
struct FieldBuffers {
    Count width;
    Count height;
    Count count;
    _Field_size_(width * height) Byte* pixels;
    _Field_size_(count + 1) Byte* terminated;
    _Field_size_part_(16, count) Byte values[16];
    _Field_size_opt_(1) IRange* object;
    _Field_size_(1) Count scalar;
    _Field_size_(-1) Byte* negative;
    float floating;
    _Field_size_(floating + 1) Byte* unsupported;
};
