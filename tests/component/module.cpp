#include "pch.h"

HRESULT __stdcall DllGetActivationFactory(void*, void** factory) noexcept {
    *factory = nullptr;
    return E_NOTIMPL;
}
