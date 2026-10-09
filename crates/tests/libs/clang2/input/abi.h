#include "enums.h"
#include "layouts.h"
#include "packed.h"
#include "union_calls.h"

struct AbiPacket {
    unsigned char tag;
    unsigned long count;
    double scale;
    long long value;
};

struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) IAbiBase {
    virtual long __stdcall Base(long value) = 0;
    virtual double __stdcall Measure(double value) = 0;
};

struct __declspec(uuid("12345678-1234-1234-1234-123456789abd")) IAbiDerived : IAbiBase {
    virtual unsigned long __stdcall Mutate(
        AbiPacket* packet, int* values, unsigned long count) = 0;
};

extern "C" unsigned long AbiLayout(unsigned long index);
extern "C" AbiPacket AbiRoundtrip(AbiPacket packet);
extern "C" IAbiDerived* AbiGet();
extern "C" long __stdcall AbiCall(
    IAbiDerived* object, long value, double measure,
    AbiPacket* packet, int* values, unsigned long count);
extern "C" IEnums* AbiEnums();
extern "C" State AbiEnumCall(IEnums* object, Scoped value, State* output);
