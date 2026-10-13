#pragma once
#include "interfaces.h"

#ifdef CHANGE_SCALAR
typedef long Scalar;
#else
typedef int Scalar;
#endif

#ifdef CHANGE_PACKING
#pragma pack(push, 4)
#else
#pragma pack(push, 2)
#endif
struct Packed {
    char first;
    short second;
};
#pragma pack(pop)

#define DECLARE_FIELD(name) union { int first; unsigned second; } name
struct Packet {
    DECLARE_FIELD(one);
    DECLARE_FIELD(two);
    Scalar scalar;
    Packed packed;
};

enum Status : unsigned {
#ifdef CHANGE_ENUM
    Value = 5
#else
    Value = 3
#endif
};

#ifdef CHANGE_CONVENTION
#define CONVENTION __cdecl
#else
#define CONVENTION __stdcall
#endif
#ifdef CHANGE_SIGNATURE
extern "C" Scalar CONVENTION Invoke(const Packet *packet, Status status);
#else
extern "C" Scalar CONVENTION Invoke(Packet *packet, Status status);
#endif

#ifdef CHANGE_CONSTANT
#define Limit (7 + 4)
#else
#define Limit (7 + 2)
#endif

#ifdef CHANGE_GUID
#define NATIVE_UUID "22334455-6677-8899-aabb-ccddeeff0011"
#else
#define NATIVE_UUID "33445566-7788-99aa-bbcc-ddeeff001122"
#endif
struct __declspec(uuid(NATIVE_UUID)) Derived : Root {
    virtual Result __stdcall Extra(Scalar value) = 0;
#ifdef CHANGE_SLOT
    virtual Result __stdcall More() = 0;
#endif
};
