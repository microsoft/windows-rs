#include "namespace_reference.h"

struct __declspec(uuid("905a0fef-bc53-11df-8c49-001e4fc686db")) IBuffer : IUnknown {
    virtual long __stdcall Read(int* value) = 0;
};

typedef int Handler;

struct CollisionConsumer {
    IBuffer* local;
    ABI::External::IBuffer* external;
    Handler value;
};
