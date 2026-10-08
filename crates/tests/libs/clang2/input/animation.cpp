#include <windows.h>
#include <UIAnimation.h>
#include <dcomp.h>

extern "C" bool AnimationIdentities(const GUID* values) {
    const GUID expected[] = {
        __uuidof(UIAnimationManager2),
        __uuidof(UIAnimationTransitionLibrary2),
        __uuidof(IUIAnimationManager2),
        __uuidof(IUIAnimationStoryboard2),
        __uuidof(IUIAnimationTransition2),
        __uuidof(IUIAnimationTransitionLibrary2),
        __uuidof(IUIAnimationVariable2),
        __uuidof(IDCompositionAnimation),
    };
    for (unsigned i = 0; i < ARRAYSIZE(expected); ++i) {
        if (values[i] != expected[i]) return false;
    }
    return true;
}

struct CurveStats {
    unsigned reset;
    unsigned begin;
    unsigned cubic;
    unsigned sinusoidal;
    unsigned repeat;
    unsigned end;
};

class Curve final : public IDCompositionAnimation {
    ULONG references = 1;
    CurveStats* stats;
public:
    explicit Curve(CurveStats* value) : stats(value) {}
    HRESULT __stdcall QueryInterface(REFIID iid, void** result) override {
        *result = nullptr;
        if (iid != __uuidof(IUnknown) && iid != __uuidof(IDCompositionAnimation)) {
            return E_NOINTERFACE;
        }
        *result = static_cast<IDCompositionAnimation*>(this);
        AddRef();
        return S_OK;
    }
    ULONG __stdcall AddRef() override { return ++references; }
    ULONG __stdcall Release() override {
        auto remaining = --references;
        if (!remaining) delete this;
        return remaining;
    }
    HRESULT __stdcall Reset() override {
        ++stats->reset;
        return S_OK;
    }
    HRESULT __stdcall SetAbsoluteBeginTime(LARGE_INTEGER) override {
        ++stats->begin;
        return S_OK;
    }
    HRESULT __stdcall AddCubic(double, float, float, float, float) override {
        ++stats->cubic;
        return S_OK;
    }
    HRESULT __stdcall AddSinusoidal(double, float, float, float, float) override {
        ++stats->sinusoidal;
        return S_OK;
    }
    HRESULT __stdcall AddRepeat(double, double) override {
        ++stats->repeat;
        return S_OK;
    }
    HRESULT __stdcall End(double, float) override {
        ++stats->end;
        return S_OK;
    }
};

extern "C" IDCompositionAnimation* AnimationCurve(CurveStats* stats) {
    return new Curve(stats);
}
