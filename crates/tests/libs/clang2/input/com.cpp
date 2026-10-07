#include "com.h"

class Instance final : public IUnknown {
    ComStats* stats;
public:
    explicit Instance(ComStats* value) : stats(value) {
        stats->references = 1;
        ++stats->created;
    }
    HRESULT __stdcall QueryInterface(REFIID iid, void** result) override {
        ++stats->queries;
        *result = nullptr;
        if (iid != __uuidof(IUnknown)) return E_NOINTERFACE;
        *result = static_cast<IUnknown*>(this);
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
};

class Factory final : public IClassFactory {
    ComStats* stats;
    ComStats* instance;
public:
    Factory(ComStats* value, ComStats* child) : stats(value), instance(child) {
        stats->references = 1;
        ++stats->created;
    }
    HRESULT __stdcall QueryInterface(REFIID iid, void** result) override {
        ++stats->queries;
        *result = nullptr;
        if (iid != __uuidof(IUnknown) && iid != __uuidof(IClassFactory)) return E_NOINTERFACE;
        *result = static_cast<IClassFactory*>(this);
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
    HRESULT __stdcall CreateInstance(IUnknown* outer, REFIID iid, void** result) override {
        *result = nullptr;
        if (outer) return CLASS_E_NOAGGREGATION;
        if (iid != __uuidof(IUnknown)) return E_NOINTERFACE;
        auto object = new Instance(instance);
        auto status = object->QueryInterface(iid, result);
        object->Release();
        return status;
    }
    HRESULT __stdcall LockServer(BOOL) override { return S_OK; }
};

IClassFactory* ComFactory(ComStats* factory, ComStats* instance) {
    return new Factory(factory, instance);
}

class Properties final : public IProperties {
    ULONG references = 1;
    HWND window = nullptr;
    LPCWSTR text = L"";
public:
    HRESULT __stdcall QueryInterface(REFIID iid, void** result) override {
        *result = nullptr;
        if (iid != __uuidof(IUnknown) && iid != __uuidof(IProperties)) return E_NOINTERFACE;
        *result = static_cast<IProperties*>(this);
        AddRef();
        return S_OK;
    }
    ULONG __stdcall AddRef() override { return ++references; }
    ULONG __stdcall Release() override {
        auto remaining = --references;
        if (!remaining) delete this;
        return remaining;
    }
    HRESULT __stdcall put_Window(HWND value) override {
        window = value;
        return S_OK;
    }
    HRESULT __stdcall get_Window(HWND* value) override {
        *value = window;
        return S_OK;
    }
    HRESULT __stdcall SetText(LPCWSTR value) override {
        if (!value) return E_INVALIDARG;
        text = value;
        return S_OK;
    }
    HRESULT __stdcall MatchText(LPCWSTR value, BOOL* equal) override {
        if (!value) return E_INVALIDARG;
        auto stored = text;
        while (*stored && *stored == *value) {
            ++stored;
            ++value;
        }
        *equal = *stored == *value;
        return S_OK;
    }
    HRESULT __stdcall Inspect(IUnknown* value) override {
        if (!value) return E_INVALIDARG;
        IUnknown* identity = nullptr;
        auto status = value->QueryInterface(__uuidof(IUnknown), reinterpret_cast<void**>(&identity));
        if (identity) identity->Release();
        return status;
    }
};

IProperties* ComProperties() {
    return new Properties();
}
