#include "native_bases.h"

struct Implementation : Combined {
    unsigned int references = 1;

    unsigned int __stdcall Read(unsigned int value) override { return value + 37; }
    long __stdcall QueryInterface(const GUID&, void** result) override {
        *result = static_cast<IUnknown*>(this);
        AddRef();
        return 0;
    }
    unsigned long __stdcall AddRef() override { return ++references; }
    unsigned long __stdcall Release() override { return --references; }
};

static Implementation instance;

extern "C" Combined* __stdcall NativeObject() { return &instance; }
extern "C" unsigned int __stdcall ReferenceCount() { return instance.references; }
