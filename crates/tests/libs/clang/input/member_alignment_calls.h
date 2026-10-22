#include "member_alignment.h"
typedef DeviceIoControl Items[2];
typedef void (*AlignedCallback)(FloatGap value);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IAligned {
    virtual void __stdcall Pass(Request value) = 0;
};
extern "C" void ByPointer(DeviceIoControl* value);
extern "C" void ArrayParameter(DeviceIoControl values[]);
extern "C" void AliasArrayParameter(Items values);
extern "C" void ByValue(FloatGap value);
extern "C" Request ByReturn();
extern "C" void ByContainingArray(ArrayContainer value);
#pragma pack(push, 1)
struct PackedMember { char tag; __declspec(align(8)) unsigned long value; };
struct PackedContainer { AlignedArray value; };
#pragma pack(pop)
struct AlignedBitfield { __declspec(align(8)) unsigned long value : 2; };
