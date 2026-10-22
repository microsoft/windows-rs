#include "native_references.h"

unsigned __stdcall ApplyReference(const ReferenceAlias& value, unsigned& total) {
    total += value.low + value.high;
    return total;
}

unsigned __stdcall InvokeReference(ReferenceCallback callback, const ReferenceAlias& value, unsigned& total) {
    return callback(value, total);
}

unsigned __stdcall InvokeAnonymous(unsigned (__cdecl *callback)(const ReferenceAlias&, unsigned&, int), const ReferenceAlias& value, unsigned& total) {
    return callback(value, total, 3);
}

struct References : IReferences {
    unsigned __stdcall Apply(const ReferenceAlias& value, unsigned& total) override {
        return ApplyReference(value, total);
    }
};

extern "C" void* __stdcall ReferenceObject() {
    static References value;
    return &value;
}

unsigned __stdcall InvokeObject(IReferences* object, const ReferenceAlias& value, unsigned& total) {
    return object->Apply(value, total);
}
