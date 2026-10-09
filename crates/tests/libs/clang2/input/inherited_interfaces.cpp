#include "inherited_interfaces.h"

class InheritedValue final : public IExtended {
    InheritedStats* stats;
public:
    explicit InheritedValue(InheritedStats* value) : stats(value) {
        stats->references = 1;
    }
    HRESULT __stdcall QueryInterface(REFIID iid, void** result) override {
        ++stats->queries;
        *result = nullptr;
        if (iid != __uuidof(IUnknown) && iid != __uuidof(IBaseValue) &&
            iid != __uuidof(IMarker) && iid != __uuidof(ILeaf) && iid != __uuidof(IExtended)) {
            return E_NOINTERFACE;
        }
        *result = static_cast<IExtended*>(this);
        AddRef();
        return S_OK;
    }
    ULONG __stdcall AddRef() override {
        ++stats->adds;
        return ++stats->references;
    }
    ULONG __stdcall Release() override {
        ++stats->releases;
        auto remaining = --stats->references;
        if (!remaining) {
            ++stats->destroyed;
            delete this;
        }
        return remaining;
    }
    long __stdcall Read() override { return 42; }
    long __stdcall Extra() override { return 64; }
};

IExtended* InheritedCreate(InheritedStats* stats) {
    return new InheritedValue(stats);
}

long InheritedRead(ILeaf* value) {
    return value->Read();
}

long InheritedExtra(IExtended* value) {
    return value->Extra();
}
