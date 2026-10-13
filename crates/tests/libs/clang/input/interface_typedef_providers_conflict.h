//! file provider_a.h
#pragma once
typedef long Status;
//! file provider_b.h
#pragma once
typedef short Status;
//! file contract.h
#pragma once
struct __declspec(uuid("6b44d453-343e-4246-826d-208c460655c4")) IBase {
    virtual Status __stdcall Ping() = 0;
};
struct __declspec(uuid("6b44d453-343e-4246-826d-208c460655c5")) IContract : IBase {
    virtual Status __stdcall Reset(Status value, IContract** next) = 0;
};
//! input first.h
#include "provider_a.h"
#include "contract.h"
extern "C" void __stdcall UseFirst(IContract* value);
//! input second.h
#include "provider_b.h"
#include "contract.h"
extern "C" void __stdcall UseSecond(IContract* value);
