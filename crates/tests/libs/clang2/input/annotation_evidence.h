#include <specstrings.h>

struct Packet {
    _Field_range_(1, 32) unsigned size;
    _Field_size_bytes_(size) unsigned char* data;
};
enum Choice {
    First __attribute__((annotate("_Vendor_(\"first\")"))) = 1,
};

extern "C" _Success_(return >= 0) _Ret_range_(0, 100) int Query(
    _In_range_(0, 10) int count,
    _When_(count > 0, _Out_writes_(count)) int* values,
    /* [out][size_is(count)] */ int* raw);

typedef _Return_type_success_(return >= 0) long Result;
extern "C" Result Status();

typedef _Success_(return == 0) int Callback(_In_range_(0, 10) int count, _When_(count > 0, _In_reads_(count)) int* values);
typedef Callback CallbackAlias;
extern "C" CallbackAlias Invoke;

typedef Packet AnnotatedPacket __attribute__((annotate("_Vendor_(record)")));
typedef int Values[4] __attribute__((annotate("_Vendor_(array)")));
typedef int* Buffer __attribute__((annotate("_Vendor_(pointer)")));
extern "C" void InspectAliases(AnnotatedPacket* packet, Values* values, _Out_writes_(count) Buffer buffer, unsigned count);

struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) Annotated {
    virtual _Ret_range_(0, 100) int __stdcall Query(_In_range_(0, 10) int count) = 0;
};
