//! file contract_a.h
#pragma once
struct __declspec(uuid("6b44d453-343e-4246-826d-208c460655c5")) IContract {
    virtual long __stdcall Reset() = 0;
};
//! file contract_b.h
#pragma once
struct __declspec(uuid("6b44d453-343e-4246-826d-208c460655c5")) IContract {
    virtual long __stdcall Reset() = 0;
};
//! input first.h
#include "contract_a.h"
extern "C" void __stdcall UseFirst(IContract* value);
//! input second.h
#include "contract_b.h"
extern "C" void __stdcall UseSecond(IContract* value);
